module Codegen

open Types

let rec emitType (typeAnnotation: TypeAnnotation) : string =
    match typeAnnotation with
    | SimpleType t ->
        match t with
        | "Int" -> "int64_t"
        | "int8" -> "int8_t"
        | "int16" -> "int16_t"
        | "int32" -> "int32_t"
        | "int64" -> "int64_t"
        | "uint" -> "uint32_t"
        | "uint8" -> "uint8_t"
        | "uint16" -> "uint16_t"
        | "uint32" -> "uint32_t"
        | "uint64" -> "uint64_t"
        | "Float" -> "double"
        | "float32" -> "float"
        | "float64" -> "double"
        | "String" -> "char*"
        | "()" -> "void"
        | t -> failwith $"Unexpected simple return type {t}"
    | GenericType(name, ts) -> failwith $"TODO: Return generic type {name}: {ts}"

let rec emitExpression (expr: Expression) : string =
    match expr with
    | IntLiteral n -> sprintf "%d" n
    | FloatLiteral n -> sprintf "%f" n
    | StringLiteral s -> sprintf "\"%s\"" s
    | Identifier id -> sprintf "%s" id
    | FunctionCall(name, args) ->
        match name with
        | "println" ->
            let _, messageExpr = List.head args
            let messageStr = emitExpression messageExpr
            let withNewline = messageStr.[.. messageStr.Length - 2] + "\\n\"" //@FIXME: This is ass. We'll handle stdlib calls later. For now, we remove the last `"` and replace it with `"\n`
            sprintf $"printf({withNewline})"
        | t -> failwith $"TODO: Support function {t}"

let emitStatement (statement: Statement) : string =
    match statement with
    | Return value -> sprintf $"return {emitExpression value};"
    | ExpressionStatement exprStatement -> sprintf $"{emitExpression exprStatement};"
    | VarDecl(varName, typeAnnotation, varValue) ->
        match typeAnnotation with
        | Some typ -> sprintf $"{emitType typ} {varName} = {emitExpression varValue};"
        | None ->
            match varValue with
            | IntLiteral n -> sprintf $"int64_t {varName} = {emitExpression varValue};"
            | FloatLiteral n -> sprintf $"double {varName} = {emitExpression varValue};"
            | StringLiteral s -> sprintf $"const char* {varName} = {emitExpression varValue};"
            | _ -> failwith "TODO: Type inference for complex expressions"
    | ConstDecl(varName, typeAnnotation, varValue) ->
        match typeAnnotation with
        | Some typ -> sprintf $"const {emitType typ} {varName} = {emitExpression varValue};"
        | None ->
            match varValue with
            | IntLiteral n -> sprintf $"const int64_t {varName} = {emitExpression varValue};"
            | FloatLiteral n -> sprintf $"const double {varName} = {emitExpression varValue};"
            | StringLiteral s -> sprintf $"const char* {varName} = {emitExpression varValue};"
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
                    sprintf "%s %s" cType p.Name)
                |> String.concat ", "

    let bodyStatements = fn.Body |> List.map emitStatement |> String.concat "\n    "

    sprintf
        $"""
{returnType} {fn.Name}({args}) {{
    {bodyStatements}
}}
    """

let emit (fn: FunctionDeclaration) : string =
    let functionCall = emitFunction fn

    sprintf
        $"""
#include <stdio.h>
#include <stdint.h>

{functionCall}
"""
