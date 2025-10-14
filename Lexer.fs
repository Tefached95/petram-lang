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

let rec lex (chars: char list) : Token list =
    match chars with
    | [] -> []
    | c :: rest when System.Char.IsWhiteSpace c -> lex rest
    | '-' :: '-' :: tail -> // Single line comment
        let comment, rest = takeWhile (fun c -> c <> '\n') tail

        SingleLineComment(charArrayToString comment) :: lex rest
    | ':' :: tail -> Colon :: lex tail
    | '(' :: tail -> LeftParenthesis :: lex tail
    | ')' :: tail -> RightParenthesis :: lex tail
    | '<' :: tail -> LeftAngleBracket :: lex tail
    | '>' :: tail -> RightAngleBracket :: lex tail
    | '"' :: tail ->
        let consumed, rest = takeWhile (fun c -> c <> '"') tail

        match rest with
        | '"' :: remaining -> StringLiteral(charArrayToString consumed) :: lex remaining
        | _ -> failwith "Unterminated string literal."
    | head :: tail when System.Char.IsLetter head ->
        let consumed, rest = takeWhile isIdentifierChar tail

        match charArrayToString (head :: consumed) with
        | "func" -> Func :: lex rest
        | "return" -> Return :: lex rest
        | "end" -> End :: lex rest
        | ident -> Identifier ident :: lex rest
    | head :: tail when System.Char.IsDigit head ->
        let consumed, rest = takeWhile System.Char.IsDigit tail
        let numberStr = charArrayToString (head :: consumed)
        let number = System.Int32.Parse numberStr
        IntLiteral number :: lex rest
    | _ :: tail -> Unknown :: lex tail

let lexFile (path: string) : Token list =
    let programInput = File.ReadAllText path
    let chars = programInput.ToCharArray() |> Array.toList
    lex chars
