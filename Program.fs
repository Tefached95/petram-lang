module Petram.Program

[<EntryPoint>]
let main args =
    let tokenList = Lexer.lexFile args[0]

    Parser.parse tokenList |> fun decl -> printfn $"%s{decl.ToString()}"

    0
