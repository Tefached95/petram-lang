module Types

type TypeAnnotation =
    | SimpleType of string
    | GenericType of name: string * typeArgs: TypeAnnotation list
