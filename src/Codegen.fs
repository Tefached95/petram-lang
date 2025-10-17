module Codegen

open Types

let rec emitExpression (expr: Expression) : string =
    match expr with
    | IntLiteral n -> sprintf "%d" n
    | StringLiteral s -> sprintf "\"%s\\n\"" s
    | Identifier id -> sprintf "%s" id
    | FunctionCall(name, args) ->
        match name with
        | "println" ->
            let _, messageExpr = List.head args
            let messageStr = emitExpression messageExpr
            sprintf $"printf({messageStr})"
        | t -> failwith $"TODO: Support function {t}"

let emitStatement (statement: Statement) : string =
    match statement with
    | Return value -> sprintf $"return {emitExpression value};"
    | ExpressionStatement statement -> sprintf $"{emitExpression statement};"

let rec emitType (typeAnnotation: TypeAnnotation) : string =
    match typeAnnotation with
    | SimpleType t ->
        match t with
        | "Int" -> "int"
        | "int8" -> "int8_t"
        | "int16" -> "int16_t"
        | "int32" -> "int32_t"
        | "int64" -> "int64_t"
        | "Uint" -> "uint32_t"
        | "Uint8" -> "uint8_t"
        | "Uint16" -> "uint16_t"
        | "Uint32" -> "uint32_t"
        | "Uint64" -> "uint64_t"
        | "String" -> "char*"
        | "()" -> "void"
        | t -> failwith $"Unexpected simple return type {t}"
    | GenericType(name, ts) -> failwith $"TODO: Return generic type {name}: {ts}"

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
        $"""#include <stdio.h>
#include <stdint.h>

{functionCall}
"""
