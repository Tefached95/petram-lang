# Petram Syntax Specification

Petram is a statically typed language with type inference. It is whitespace-significant, uses 4 spaces for indentation, and has a block-style syntax. There is no line terminator character.

## Comments

- Line comment: `--`
- Block comment: `{-` and `-}`

## Primitive Types

- `Int`
  - `Int8`, `Int16`, `Int32`, `Int64`
- `Float`
  - `Float32`, `Float64`
- `Bool`
- `String`
- `Char`

## Variables

- Type inferrence and assignment: `$variable := value`
- Explicit typing: `$variable: Type = value`

## Functions

- Function definition: `func #{function_name ~> param1: Type, param2: OtherType, ..., $paramN: TypeN}#: ReturnType ->`

```petra
func #{greet_user ~> name: String}#: () ->
    #{println ~> message: "Hello, {name}!"}#
    return ()
```

- Single-expression functions: `func #{add_two ~> a: Int, b: Int}#: Int => a + b`
- Function call: `#{function_name ~> arg1: value1, arg2: value2}#`
  - Named arguments must always be used in the function call.
- Argument names must always be provided.

## Structs

- Struct definition:

```petra
    struct #{StructName}# ->
        field field_name: Type

        new #{arg_name: Type}#: Self ->
            @field_name = arg_name
```

- Constructor: `new #{arg1: Type1, $arg2: Type2, ..., $argN: TypeN}#: Self`
- Instantiation: `$my_struct := #{StructName::new ~> arg1: value1, arg2: value2, ..., argN: valueN}#`
- Constrained fields:

```petra
    struct #{StructName}# ->
        constrained field field_name: Type
            where #{boolean_expression}# message: "Error message"

        new #{arg_name: Type}#: Result<Self, String> ->
            @field_name = arg_name
```

- If you introduce one or more constrained fields to your struct, then the return type of the `new #{}#` constructor must be `Result<Self, String>`.
  - You must pattern match on the result of the constructor to check for errors.
  - The error string will be the message you've defined in that particular constraint.
- Inheritance: `struct #{Rectangle < Shape}# ->`

## Protocols

- Protocol definition:

```petra
    protocol #{ProtocolName}# ->
        method #{method_name ~> arg1: Type1, arg2: Type2, ..., $argN: TypeN}# -> ReturnType
```

- When inheriting, structs must come before protocols in the inheritance list.

## Control Flow

- `if` expression:
  - `if` is an expression and must be enclosed in `#{}#`.
  - If you don't want to return anything from the `if` expression, you can discard it with the special `_` pattern.
  - The return value of the `if` expression is the value of the last expression in the block. If the last thing evaluated is a statement, then the return value is `()`.

  - ```petra
      _ := #{
            if #{somecond}# ->
                -- statements, expressions
            {- optionally
            else if #{othercond}# ->
                -- ...
            else ->
                -- ... -}
      }#
    ```

## Pattern Matching

- Pattern matching is an expression and must be enclosed in `#{}#`.

  - ```petra
    $somevar := #{
        match $something_else ->
            Pattern1 -> result1
            Pattern2 -> result2
            _ -> default_result
        }#
    ```


- Using `match` as a statement is generally discouraged. However, if you wish to, you may use the `_ :=` syntax:

  - ```petra
    _ := #{
        match $some_value ->
            Pattern1 ->
            #{println ~> message: "Pattern 1"}#
        Pattern2 ->
            #{println ~> message: "Pattern 2"}#
        _ ->
            #{println ~> message: "Default"}#
    }#
    ```

## Loops

- `foreach` loop:

  - ```petra
    -- inferred as List<Int>
    $collection := {|1, 2, 3|}

    -- $item is inferred as Int
    foreach $item in $collection ->
        #{println ~> message: "Item: {$item}"}#

    {-
        Prints:
        Item: 1
        Item: 2
        Item: 3
    -}
    ```
