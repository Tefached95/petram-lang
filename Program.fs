open System.IO

[<EntryPoint>]
let main args =
    if args.Length = 0 then
        printfn "Usage: petram <file.petra>"
        1
    else
        let filePath = args[0]
        let tokenList = Lexer.lexFile filePath
        let nameOnly = Path.GetFileNameWithoutExtension filePath

        let generatedCode = Parser.parse tokenList |> Codegen.emit
        File.WriteAllText($"{nameOnly}.c", generatedCode)

        0
