module Tokens

type Token =
    | SingleLineComment of string
    | MultiLineComment of string
    | Func
    | Colon
    | LeftParenthesis
    | RightParenthesis
    | Identifier of string
    | IntLiteral of int64
    | FloatLiteral of float
    | LeftAngleBracket
    | RightAngleBracket
    | StringLiteral of string
    | Var
    | Const
    | Equals
    | Comma
    | Return
    | Struct
    | End
    | Unknown
