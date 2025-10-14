open Lexer

[<EntryPoint>]
let main args =
    let lines = lexFile args[0]
    printfn $"Got lines {lines}"
    0
