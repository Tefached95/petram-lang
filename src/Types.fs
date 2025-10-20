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
