module Tokens

type Token =
    | SingleLineComment of string
    | MultiLineComment of string
    | Func
    | Colon
    | LeftParenthesis
    | RightParenthesis
    | Identifier of string
    | IntLiteral of int
    | LeftAngleBracket
    | RightAngleBracket
    | StringLiteral of string
    | Comma
    | Return
    | End
    | Unknown
