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
    | BitwiseAnd // 'and' keyword
    | BitwiseOr // 'or' keyword
    | BitwiseXor // 'xor' keyword
    | BitwiseNot // 'not' keyword
    // Symbols
    | Ampersand // &
    | Colon // :
    | DoubleColon // ::
    | Equals // =
    | Comma // ,
    | Pipe // |
    | Dot // .
    | Arrow // ->
    | Question // ?
    | Exclamation // !
    | LeftParenthesis // (
    | RightParenthesis // )
    | LeftBrace // {
    | RightBrace // }
    | LeftBracket // [
    | RightBracket // ]
    | LeftAngleBracket //
    | RightAngleBracket // >
    // Operators
    | Plus // +
    | Minus // -
    | Times // *
    | Divided // /
    | Modulo // %
    | ShiftLeft // <<
    | ShiftRight // >>
    | Range // ..
    | LogicalAnd // &&
    | LogicalOr // ||
    // Literals and identifiers
    | Identifier of string
    | IntLiteral of int64
    | FloatLiteral of float
    | StringLiteral of string
    // Other
    | EOF
    | Unknown
