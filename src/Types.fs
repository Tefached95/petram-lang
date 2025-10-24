module Types

type TypeAnnotation =
    | SimpleType of string
    | GenericType of name: string * typeArgs: TypeAnnotation list

type Expression =
    | IntLiteral of int64
    | FloatLiteral of float
    | StringLiteral of string
    | Identifier of string
    | FunctionCall of name: string * arguments: Argument list
    | BinaryOp of lhs: Expression * op: Tokens.Token * rhs: Expression

and Argument = { Name: string; Expr: Expression }

type Statement =
    | ExpressionStatement of Expression
    | Return of Expression
    | VarDecl of name: string * typ: TypeAnnotation option * value: Expression
    | ConstDecl of name: string * typ: TypeAnnotation option * value: Expression

type Parameter = { Name: string; Type: TypeAnnotation }

type FunctionDeclaration =
    { Name: string
      Parameters: Parameter list option
      ReturnType: TypeAnnotation option
      Body: Statement list }

let petramToCTypeMap =
    Map.ofList
        [ "Int", "int64_t"
          "i8", "int8_t"
          "i16", "int16_t"
          "i32", "int32_t"
          "i64", "int64_t"
          "uint", "uint32_t"
          "u8", "uint8_t"
          "u16", "uint16_t"
          "u32", "uint32_t"
          "u64", "uint64_t"
          "Float", "double"
          "f32", "float"
          "f64", "double"
          "String", "char*"
          "()", "void" ]
