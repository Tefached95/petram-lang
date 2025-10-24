module Keywords

open Tokens

let keywords =
    Map.ofList [ "func", Func; "end", End; "var", Var; "const", Const; "struct", Struct ]
