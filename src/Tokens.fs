module Tokens

type Token =
    // Comments
    | SingleLineComment of string
    | MultiLineComment of string
    // Keywords
    | Var
    | Const
    | Func
    | Return
    | Struct
    | If
    | Elif
    | Else
    | While
    | For
    | In
    | End
    // Symbols
    | Colon
    | Equals
    | Comma
    | LeftParenthesis
    | RightParenthesis
    | LeftAngleBracket
    | RightAngleBracket
    // Operators
    | Plus
    | Minus
    | Times
    | Divided
    | Modulo
    | BitShiftRight
    | BitShiftLeft
    | Range // ..
    // Logical operators
    | Or
    | And
    | Xor
    | Not
    // Symbols
    | Identifier of string
    | IntLiteral of int64
    | FloatLiteral of float
    | StringLiteral of string
    // Other
    | EOF
    | Unknown
