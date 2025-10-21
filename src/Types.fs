module Types

type TypeAnnotation =
    | SimpleType of string
    | GenericType of name: string * typeArgs: TypeAnnotation list

type Parameter = { Name: string; Type: TypeAnnotation }

type Expression =
    | IntLiteral of int64
    | FloatLiteral of float
    | StringLiteral of string
    | Identifier of string
    | FunctionCall of name: string * arguments: (string * Expression) list

type Statement =
    | ExpressionStatement of Expression
    | Return of Expression
    | VarDecl of name: string * typ: TypeAnnotation option * value: Expression
    | ConstDecl of name: string * typ: TypeAnnotation option * value: Expression

type FunctionDeclaration =
    { Name: string
      Parameters: Parameter list option
      ReturnType: TypeAnnotation option
      Body: Statement list }

let petramToCTypeMap =
    Map.ofList
        [ "Int", "int64_t"
          "int8", "int8_t"
          "int16", "int16_t"
          "int32", "int32_t"
          "int64", "int64_t"
          "uint", "uint32_t"
          "uint8", "uint8_t"
          "uint16", "uint16_t"
          "uint32", "uint32_t"
          "uint64", "uint64_t"
          "Float", "double"
          "float32", "float"
          "float64", "double"
          "String", "char*"
          "()", "void" ]
