module Lexer

open System.IO

let charArrayToString (chars: char list) : string =
    (chars |> List.toArray |> System.String).Trim()

let isIdentifierChar c =
    System.Char.IsLetterOrDigit c || c = '_'

type Token =
    | SingleLineComment of string
    | MultiLineComment of string
    | Func
    | Symbol of string
    | Colon
    | LeftParenthesis
    | RightParenthesis
    | Identifier of string
    | IntLiteral of int
    | LeftAngleBracket
    | RightAngleBracket
    | StringLiteral of string
    | Return
    | End
    | Unknown

let rec takeWhile (predicate: char -> bool) (chars: char list) : (char list * char list) =
    match chars with
    | [] -> [], []
    | head :: tail ->
        if predicate head then
            let consumed, remaining = takeWhile predicate tail
            head :: consumed, remaining
        else
            [], head :: tail

let lex (chars: char list) : Token list =
    let rec loop (chars: char list) (acc: Token list) =
        match chars with
        | [] -> acc |> List.rev
        | c :: rest when System.Char.IsWhiteSpace c -> loop rest acc
        | '-' :: '-' :: tail ->
            let comment, rest = takeWhile (fun c -> c <> '\n') tail

            loop rest (SingleLineComment(charArrayToString comment) :: acc)
        | ':' :: tail -> loop tail (Colon :: acc)
        | '(' :: tail -> loop tail (LeftParenthesis :: acc)
        | ')' :: tail -> loop tail (RightParenthesis :: acc)
        | '<' :: tail -> loop tail (LeftAngleBracket :: acc)
        | '>' :: tail -> loop tail (RightAngleBracket :: acc)
        | '"' :: tail ->
            let consumed, rest = takeWhile (fun c -> c <> '"') tail

            match rest with
            | '"' :: remaining -> loop remaining (StringLiteral(charArrayToString consumed) :: acc)
            | _ -> failwith "Unterminated string literal."
        | head :: tail when System.Char.IsLetter head ->
            let consumed, rest = takeWhile isIdentifierChar tail

            match charArrayToString (head :: consumed) with
            | "func" -> loop rest (Func :: acc)
            | "return" -> loop rest (Return :: acc)
            | "end" -> loop rest (End :: acc)
            | ident -> loop rest (Identifier ident :: acc)
        | head :: tail when System.Char.IsDigit head ->
            let consumed, rest = takeWhile System.Char.IsDigit tail
            let numberStr = charArrayToString (head :: consumed)
            let number = System.Int32.Parse numberStr
            loop rest (IntLiteral number :: acc)
        | _ :: tail -> loop tail (Unknown :: acc)

    loop chars []

let lexFile (path: string) : Token list =
    let programInput = File.ReadAllText path
    let chars = programInput.ToCharArray() |> Array.toList
    lex chars
