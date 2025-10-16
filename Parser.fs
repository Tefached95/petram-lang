module Parser

type Parameter = { Name: string; Type: Types.PTypes }

type FunctionDeclaration =
    { Name: string
      Parameters: Parameter list }
