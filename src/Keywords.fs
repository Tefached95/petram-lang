module Keywords

open Tokens

let keywords =
    Map.ofList
        [ "var", Var
          "const", Const
          "func", Func
          "return", Return
          "struct", Struct
          "if", If
          "elif", Elif
          "else", Else
          "while", While
          "for", For
          "in", In
          "end", End
          "and", BitwiseAnd
          "or", BitwiseOr
          "xor", BitwiseXor
          "not", BitwiseNot ]
