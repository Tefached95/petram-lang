module Types

type PTypes =
    | IntLiteral of int
    | Int8Literal of int8
    | Int16Literal of int16
    | Int32Literal of int32
    | Int64Literal of int64
    | UintLiteral of uint
    | Uint8Literal of uint8
    | Uint16Literal of uint16
    | Uint32Literal of uint32
    | Uint64Literal of uint64
    | StringLiteral of string
    | ArrayLiteral of PTypes array
    | ListLiteral of PTypes list
