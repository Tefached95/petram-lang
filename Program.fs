open System.IO

[<EntryPoint>]
let main args =
    if args.Length = 0 then
        printfn "Usage: petram <file.petra>"
        1
    else
        let filePath = args[0]
        let generatedCode = Lexer.lexFile filePath |> Parser.parse |> Codegen.emit

        let nameOnly = Path.GetFileNameWithoutExtension filePath

        if Path.Exists "output" = false then
            Directory.CreateDirectory "output" |> ignore

        File.WriteAllText($"output/{nameOnly}.c", generatedCode)

        0
