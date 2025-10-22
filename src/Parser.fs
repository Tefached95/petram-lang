module Parser

open Tokens
open Types

let rec parseTypeAnnotation (tokens: Token list) : TypeAnnotation * Token list =
    match tokens with
    | Token.Identifier typeName :: LeftAngleBracket :: args ->
        let typeArg, afterArgs = parseTypeAnnotation args

        match afterArgs with
        | RightAngleBracket :: rest -> GenericType(typeName, [ typeArg ]), rest
        | _ -> failwith "Expected >"
    | Token.Identifier typeName :: rest -> SimpleType typeName, rest
    | _ -> failwith "Expected type name"

let rec parseDelimitedList (parseOne: 'a list -> 'b * 'a list) (closingDelimiter: Token) (tokens: Token list) =
    match tokens with
    | token :: _ when token = closingDelimiter -> [], tokens
    | _ ->
        let param, remaining = parseOne tokens

        match remaining with
        | Comma :: rest ->
            let otherParams, finalTokens = parseDelimitedList parseOne closingDelimiter rest
            param :: otherParams, finalTokens
        | token :: _ when token = closingDelimiter -> [ param ], remaining
        | _ -> failwith "Expected `,` or `)` after parameter"

let rec parseParameter (tokens: Token list) : Parameter * Token list =
    match tokens with
    | Token.Identifier paramName :: Colon :: typ ->
        let paramType, remaining = parseTypeAnnotation typ
        let param = { Name = paramName; Type = paramType }
        param, remaining
    | _ -> failwith "Expected parameter"

let parseParameters (tokens: Token list) : Parameter list * Token list =
    parseDelimitedList parseParameter RightParenthesis tokens

let rec parseExpression (tokens: Token list) : Expression * Token list =
    match tokens with
    | Token.IntLiteral value :: tail -> IntLiteral value, tail
    | Token.FloatLiteral value :: tail -> FloatLiteral value, tail
    | Token.StringLiteral str :: tail -> StringLiteral str, tail
    | Token.Identifier name :: LeftParenthesis :: tail ->
        // function call
        let args, rest = parseArguments tail

        match rest with
        | RightParenthesis :: remaining -> FunctionCall(name, args), remaining
        | _ -> failwith "Expected `)` after function arguments"
    | Token.Identifier name :: tail ->
        // variable
        Identifier name, tail
    | _ -> failwith "Unexpected token in expression"

and parseArguments (tokens: Token list) : Argument list * Token list =
    parseDelimitedList parseArgument RightParenthesis tokens

and parseArgument (tokens: Token list) : Argument * Token list =
    match tokens with
    | Token.Identifier argName :: Colon :: expression ->
        let argExpr, rest = parseExpression expression
        let arg = { Name = argName; Expr = argExpr }
        arg, rest

let (|DeclarationWithOptionalTypes|_|)
    (tokens: Token list)
    : (string * TypeAnnotation option * Expression * Token list) option =
    match tokens with
    | Token.Identifier name :: Colon :: typAnnotation ->
        let annotation, afterAnnotation = parseTypeAnnotation typAnnotation

        match afterAnnotation with
        | Equals :: valueExpression ->
            let expr, remaining = parseExpression valueExpression
            Some(name, Some annotation, expr, remaining)
        | _ -> failwith "Expected `=` after type declaration"
    | Token.Identifier name :: Equals :: valueExpression ->
        let expr, rest = parseExpression valueExpression
        Some(name, None, expr, rest)
    | _ -> None

let rec parseStatements (tokens: Token list) : Statement list * Token list =
    match tokens with
    | SingleLineComment _ :: tail -> parseStatements tail
    | End :: tail -> [], tail
    | Token.Return :: returnValue ->
        let expr, afterReturn = parseExpression returnValue
        let otherStatements, remaining = parseStatements afterReturn
        Return expr :: otherStatements, remaining
    | Var :: DeclarationWithOptionalTypes(name, maybeType, expr, statements) ->
        let otherStatements, remaining = parseStatements statements
        VarDecl(name, maybeType, expr) :: otherStatements, remaining
    | Const :: DeclarationWithOptionalTypes(name, maybeType, expr, statements) ->
        let otherStatements, remaining = parseStatements statements
        ConstDecl(name, maybeType, expr) :: otherStatements, remaining
    | Token.Identifier _ :: _ ->
        // Expression statement
        let expr, remainingStatements = parseExpression tokens
        let otherStatements, rest = parseStatements remainingStatements
        ExpressionStatement expr :: otherStatements, rest
    | t -> failwith $"Not implemented for {t}"

let parseFunctionDeclaration (tokens: Token list) : FunctionDeclaration * Token list =
    match tokens with
    | Token.Identifier name :: LeftParenthesis :: parameters ->
        let parameters, afterParameters = parseParameters parameters

        match afterParameters with
        | RightParenthesis :: Colon :: returnAnnotation ->
            let returnType, functionBody = parseTypeAnnotation returnAnnotation
            let statements, remaining = parseStatements functionBody

            { Name = name
              Parameters = Some parameters
              ReturnType = Some returnType
              Body = statements },
            remaining
        | _ -> failwith "Expected ')' and return type"
    | _ -> failwith "Expected function name and parameters"

let rec parse (tokens: Token list) : FunctionDeclaration =
    match tokens with
    | Func :: tail -> parseFunctionDeclaration tail |> fst
    | SingleLineComment _ :: tail -> parse tail
    | t -> failwith $"Unsupported token {t}"
