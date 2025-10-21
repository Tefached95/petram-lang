module Codegen

open Types

let rec emitType (typeAnnotation: TypeAnnotation) : string =
    match typeAnnotation with
    | SimpleType t ->
        Map.tryFind t petramToCTypeMap
        |> Option.defaultWith (fun () -> failwith $"Could not map type {t} to a C equivalent.")
    | GenericType(name, ts) -> failwith $"TODO: Return generic type {name}: {ts}"

let rec emitExpression (expr: Expression) : string =
    match expr with
    | IntLiteral n -> $"%d{n}"
    | FloatLiteral n -> $"%f{n}"
    | StringLiteral s -> $"\"%s{s}\""
    | Identifier id -> $"%s{id}"
    | FunctionCall(name, args) ->
        match name with
        | "println" ->
            let _, messageExpr = List.head args
            let messageStr = emitExpression messageExpr
            //@FIXME: This is ass. We'll handle stdlib calls later. For now, we remove the last `"` and replace it with `"\n`
            let withNewline = messageStr.[.. messageStr.Length - 2] + "\\n\""
            sprintf $"printf({withNewline})"
        | t -> failwith $"TODO: Support function {t}"

let emitStatement (statement: Statement) : string =
    match statement with
    | Return value -> $"return {emitExpression value};"
    | ExpressionStatement exprStatement -> $"{emitExpression exprStatement};"
    | VarDecl(varName, typeAnnotation, varValue) ->
        match typeAnnotation with
        | Some typ -> $"{emitType typ} {varName} = {emitExpression varValue};"
        | None ->
            match varValue with
            | IntLiteral _ -> $"int64_t {varName} = {emitExpression varValue};"
            | FloatLiteral _ -> $"double {varName} = {emitExpression varValue};"
            | StringLiteral _ -> $"const char* {varName} = {emitExpression varValue};"
            | _ -> failwith "TODO: Type inference for complex expressions"
    | ConstDecl(varName, typeAnnotation, varValue) ->
        match typeAnnotation with
        | Some typ -> $"const {emitType typ} {varName} = {emitExpression varValue};"
        | None ->
            match varValue with
            | IntLiteral _ -> $"const int64_t {varName} = {emitExpression varValue};"
            | FloatLiteral _ -> $"const double {varName} = {emitExpression varValue};"
            | StringLiteral _ -> $"const char* {varName} = {emitExpression varValue};"
            | _ -> failwith "TODO: Type inference for complex expressions"

let emitFunction (fn: FunctionDeclaration) : string =
    let returnType =
        match fn.ReturnType with
        | None -> "void"
        | Some retType -> emitType retType

    let args =
        match fn.Parameters with
        | None -> "void"
        | Some parameters ->
            // For now, special case main's List<String> args
            match parameters with
            | [ { Name = _
                  Type = GenericType("List", [ SimpleType "String" ]) } ] -> "int argc, char **argv"
            | _ ->
                parameters
                |> List.map (fun p ->
                    let cType = emitType p.Type
                    $"%s{cType} %s{p.Name}")
                |> String.concat ", "

    let bodyStatements = fn.Body |> List.map emitStatement |> String.concat "\n    "

    $"""
{returnType} {fn.Name}({args}) {{
    {bodyStatements}
}}
    """

let emit (fn: FunctionDeclaration) : string =
    let functionCall = emitFunction fn

    $"""
#include <stdio.h>
#include <stdint.h>

{functionCall}
"""
