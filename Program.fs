open Lexer

[<EntryPoint>]
let main args =
    let tokenList = lexFile args[0]
    tokenList |> List.iter (fun token -> printfn "%s" (token.ToString()))
    0
