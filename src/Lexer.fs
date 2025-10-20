module Lexer

open System
open Tokens

let charArrayToString (chars: char list) : string =
    (chars |> List.toArray |> System.String).Trim()

let isIdentifierChar c =
    System.Char.IsLetterOrDigit c || c = '_'

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
        | c :: rest when System.Char.IsWhiteSpace c -> loop rest acc
        | '-' :: '-' :: tail ->
            let comment, rest = takeWhile (fun c -> c <> '\n') tail

            loop rest (SingleLineComment(charArrayToString comment) :: acc)
        | ':' :: tail -> loop tail (Colon :: acc)
        | '(' :: tail -> loop tail (LeftParenthesis :: acc)
        | ')' :: tail -> loop tail (RightParenthesis :: acc)
        | '<' :: tail -> loop tail (LeftAngleBracket :: acc)
        | '>' :: tail -> loop tail (RightAngleBracket :: acc)
        | ',' :: tail -> loop tail (Comma :: acc)
        | '=' :: tail -> loop tail (Equals :: acc)
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
            | "var" -> loop rest (Var :: acc)
            | "const" -> loop rest (Const :: acc)
            | ident -> loop rest (Identifier ident :: acc)
        | head :: tail when System.Char.IsDigit head ->
            // We're dealing with a number. Need to check if it's an int or a float literal. Untyped defaults to system-specific largest word.
            let consumed, rest = takeWhile System.Char.IsDigit tail

            match rest with
            | '.' :: rest' ->
                let decimalPart, remaining = takeWhile System.Char.IsDigit rest'

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
