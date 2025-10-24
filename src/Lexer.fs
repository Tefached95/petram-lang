module Lexer

open System
open Tokens
open Keywords

let charArrayToString (chars: char list) : string =
    (chars |> List.toArray |> String).Trim()

let isIdentifierChar c = Char.IsLetterOrDigit c || c = '_'

/// <summary>
/// Consumes the input stream of `chars` while `predicate` holds true. Unlike `List.takeWhile`, this method will return a tuple containing the consumed list and the remainder.
/// </summary>
/// <param name="predicate">Predicate function to check if we should consume a character</param>
/// <param name="chars">List of characters to iterate over.</param>
/// <returns>Tuple where `fst` is the consumed part and `snd` is the remainder.</returns>
let rec takeWhile (predicate: char -> bool) (chars: char list) : char list * char list =
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
        | c :: rest when Char.IsWhiteSpace c -> loop rest acc
        | '-' :: tail ->
            match tail with
            | '-' :: rest ->
                let comment, rest = takeWhile (fun c -> c <> '\n') tail

                loop rest (SingleLineComment(charArrayToString comment) :: acc)
            | '>' :: rest -> loop rest (Arrow :: acc)
            | _ -> loop tail (Minus :: acc)
        | ':' :: tail ->
            match tail with
            | ':' :: rest -> loop rest (DoubleColon :: acc)
            | _ -> loop tail (Colon :: acc)
        | '(' :: tail -> loop tail (LeftParenthesis :: acc)
        | ')' :: tail -> loop tail (RightParenthesis :: acc)
        | '[' :: tail -> loop tail (LeftBracket :: acc)
        | ']' :: tail -> loop tail (RightBracket :: acc)
        | '{' :: tail -> loop tail (LeftBrace :: acc)
        | '}' :: tail -> loop tail (RightBrace :: acc)
        | '.' :: tail ->
            match tail with
            | '.' :: rest -> loop rest (Range :: acc)
            | _ -> loop tail (Dot :: acc)
        | '?' :: tail -> loop tail (Question :: acc)
        | '!' :: tail -> loop tail (Exclamation :: acc)
        | '<' :: tail ->
            match tail with
            | '<' :: rest -> loop rest (ShiftLeft :: acc)
            | _ -> loop tail (LeftAngleBracket :: acc)
        | '>' :: tail ->
            match tail with
            | '>' :: rest -> loop rest (ShiftRight :: acc)
            | _ -> loop tail (RightAngleBracket :: acc)
        | ',' :: tail -> loop tail (Comma :: acc)
        | '=' :: tail -> loop tail (Equals :: acc)
        | '+' :: tail -> loop tail (Plus :: acc)
        | '*' :: tail -> loop tail (Times :: acc)
        | '/' :: tail -> loop tail (Divided :: acc)
        | '%' :: tail -> loop tail (Modulo :: acc)
        | '|' :: tail ->
            match tail with
            | '|' :: rest -> loop rest (LogicalOr :: acc)
            | _ -> loop tail (Pipe :: acc)
        | '&' :: tail ->
            match tail with
            | '&' :: rest -> loop rest (LogicalAnd :: acc)
            | _ -> loop tail (Ampersand :: acc)
        | '"' :: tail ->
            // @TODO: escape sequences
            let consumed, rest = takeWhile (fun c -> c <> '"') tail

            match rest with
            | '"' :: remaining -> loop remaining (StringLiteral(charArrayToString consumed) :: acc)
            | _ -> failwith "Unterminated string literal."
        | head :: tail when Char.IsLetter head ->
            let consumed, rest = takeWhile isIdentifierChar tail
            let maybeKeyword = charArrayToString (head :: consumed)

            match Map.tryFind maybeKeyword keywords with
            | Some keyword -> loop rest (keyword :: acc)
            | None -> loop rest (Identifier maybeKeyword :: acc)
        | head :: tail when Char.IsDigit head ->
            // We're dealing with a number. Need to check if it's an int or a float literal. Untyped defaults to system-specific largest word.
            let consumed, rest = takeWhile Char.IsDigit tail

            match rest with
            | '.' :: rest' ->
                let decimalPart, remaining = takeWhile Char.IsDigit rest'

                // If the decimal part is empty, then ignore the `.` token and just emit an integer literal.
                if List.isEmpty decimalPart then
                    let numberStr = charArrayToString (head :: consumed)
                    let number = int64 numberStr
                    loop rest (IntLiteral number :: acc)
                else
                    let numberStr = charArrayToString (head :: consumed @ '.' :: decimalPart)
                    let number = float numberStr
                    loop remaining (FloatLiteral number :: acc)
            | _ :: rest' ->
                let numberStr = charArrayToString (head :: consumed)
                let number = int64 numberStr
                loop rest' (IntLiteral number :: acc)
        | _ :: tail -> loop tail (Unknown :: acc)

    loop chars []

let lexFile (path: string) : Token list =
    let programInput = IO.File.ReadAllText path
    let chars = programInput.ToCharArray() |> Array.toList
    lex chars
