module Parser

open Tokens
open Types

let rec parseTypeAnnotation (tokens: Token list) : TypeAnnotation * Token list =
    match tokens with
    | Token.Identifier typeName :: LeftAngleBracket :: rest ->
        let typeArg, rest' = parseTypeAnnotation rest

        match rest' with
        | RightAngleBracket :: remaining -> GenericType(typeName, [ typeArg ]), remaining
        | _ -> failwith "Expected >"
    | Token.Identifier typeName :: rest -> SimpleType typeName, rest
    | _ -> failwith "Expected type name"

let rec parseExpression (tokens: Token list) : Expression * Token list =
    match tokens with
    | Token.IntLiteral value :: tail -> IntLiteral value, tail
    | Token.FloatLiteral value :: tail -> FloatLiteral value, tail
    | Token.StringLiteral str :: tail -> StringLiteral str, tail
    | Token.Identifier name :: LeftParenthesis :: tail ->
        // function call
        let args, rest = parseArguments tail
        FunctionCall(name, args), rest
    | Token.Identifier name :: tail ->
        // variable
        Identifier name, tail
    | _ -> failwith "Unexpected token in expression"

and parseArguments (tokens: Token list) : (string * Expression) list * Token list =
    match tokens with
    | RightParenthesis :: rest -> ([], rest)
    | Token.Identifier argName :: Colon :: tail ->
        let argExpr, rest' = parseExpression tail

        match rest' with
        | Comma :: rest'' ->
            let moreArgs, rest''' = parseArguments rest''
            (argName, argExpr) :: moreArgs, rest'''
        | RightParenthesis :: rest'' -> [ (argName, argExpr) ], rest''
        | _ -> failwith "Expected ',' or ')' after argument"
    | _ -> failwith "Expected argument name"

let (|DeclWithOptionalType|_|)
    (tokens: Token list)
    : (string * TypeAnnotation option * Expression * Token list) option =
    match tokens with
    | Token.Identifier name :: Colon :: tail ->
        let annotation, rest = parseTypeAnnotation tail

        match rest with
        | Equals :: rest' ->
            let expr, rest'' = parseExpression rest'
            Some(name, Some annotation, expr, rest'')
        | _ -> failwith "Expected `=` after type declaration"
    | Token.Identifier name :: Equals :: tail ->
        let expr, rest = parseExpression tail
        Some(name, None, expr, rest)
    | _ -> None

let rec parseStatements (tokens: Token list) : Statement list * Token list =
    match tokens with
    | SingleLineComment _ :: tail -> parseStatements tail
    | End :: tail -> [], tail
    | Token.Return :: tail ->
        let expr, rest = parseExpression tail
        let otherStatements, rest' = parseStatements rest
        Return expr :: otherStatements, rest'
    | Var :: DeclWithOptionalType(name, maybeType, expr, rest) ->
        let otherStatements, rest' = parseStatements rest
        VarDecl(name, maybeType, expr) :: otherStatements, rest'
    | Const :: DeclWithOptionalType(name, maybeType, expr, rest) ->
        let otherStatements, rest' = parseStatements rest
        ConstDecl(name, maybeType, expr) :: otherStatements, rest'
    | Token.Identifier _ :: _ ->
        // Expression statement
        let expr, rest = parseExpression tokens
        let otherStatements, rest' = parseStatements rest
        ExpressionStatement expr :: otherStatements, rest'
    | t -> failwith $"Not implemented for {t}"

let rec parseParameter (tokens: Token list) : Parameter * Token list =
    match tokens with
    | Token.Identifier paramName :: Colon :: tail ->
        let paramType, remaining = parseTypeAnnotation tail
        let param = { Name = paramName; Type = paramType }
        param, remaining
    | _ -> failwith "Expected parameter"

let rec parseParameters (tokens: Token list) : Parameter list * Token list =
    match tokens with
    | RightParenthesis :: _ -> [], tokens
    | _ ->
        let param, remaining = parseParameter tokens

        match remaining with
        | Comma :: rest ->
            let otherParams, finalTokens = parseParameters rest
            param :: otherParams, finalTokens
        | RightParenthesis :: _ -> ([ param ], remaining)
        | _ -> failwith "Expected `,` or `)` after parameter"

let parseFunctionDeclaration (tokens: Token list) : FunctionDeclaration * Token list =
    match tokens with
    // e.g. println( [...]
    | Token.Identifier name :: LeftParenthesis :: tail ->
        let parameters, rest = parseParameters tail

        match rest with
        | RightParenthesis :: Colon :: rest' ->
            let returnType, rest'' = parseTypeAnnotation rest'
            let statements, rest''' = parseStatements rest''

            { Name = name
              Parameters = Some parameters
              ReturnType = Some returnType
              Body = statements },
            rest'''
        | _ -> failwith "Expected ')' and return type"
    | _ -> failwith "Expected function name and parameters"

let rec parse (tokens: Token list) : FunctionDeclaration =
    match tokens with
    | Func :: tail -> parseFunctionDeclaration tail |> fst
    | SingleLineComment _ :: tail -> parse tail
    | t -> failwith $"Unsupported token {t}"
