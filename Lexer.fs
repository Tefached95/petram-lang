module Lexer

open System.IO

type Token =
    | Func
    | Colon
    | LParen
    | RParen
    | Identifier of string
    | IntLiteral of int
    | LeftAngleBracket
    | RightAngleBracket
    | StringLiteral of string
    | Return
    | End

let lexFile path = File.ReadAllLines path
