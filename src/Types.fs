module Types

type TypeAnnotation =
    | SimpleType of string
    | GenericType of name: string * typeArgs: TypeAnnotation list

type Parameter = { Name: string; Type: TypeAnnotation }

type Expression =
    | IntLiteral of int
    | StringLiteral of string
    | Identifier of string
    | FunctionCall of name: string * arguments: (string * Expression) list

type Statement =
    | ExpressionStatement of Expression
    | Return of Expression

type FunctionDeclaration =
    { Name: string
      Parameters: Parameter list option
      ReturnType: TypeAnnotation option
      Body: Statement list }
