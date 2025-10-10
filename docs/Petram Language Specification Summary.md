# Petram Language Specification

Version 0.1 - Draft

## Overview

Petram is a statically-typed, compiled programming language designed for explicitness, safety, and clean syntax. It features pattern matching, algebraic data types, trait-based generics, and explicit error handling.

## Design Principles

1. **Explicitness over implicitness** - Control flow and behavior should be clear
2. **Consistency** - Similar constructs should work similarly
3. **Safety** - Errors are values, memory is managed automatically
4. **Readability** - Code should be easy to read and understand

---

## Lexical Structure

### Comments

```petra
-- Single-line comment

{-
   Multi-line comment
   Can span multiple lines
-}
```

### Keywords

Reserved keywords that cannot be used as identifiers:

```
func, var, const, return, if, elif, else, match, case, while, for, in, end,
struct, field, method, trait, module, import, defer, break, continue, from
```

### Identifiers

Identifiers start with a letter or underscore, followed by letters, digits, or underscores.

```
valid_identifier
_private
MyStruct
variable123
```

### Literals

**Integer Literals:**
```petra
42
-17
0
1_000_000  -- underscores for readability
```

**Float Literals:**
```petra
3.14
-0.5
2.0
1.5e10
```

**String Literals:**
```petra
"Hello, world!"
"Escape sequences: \n \t \\ \""
```

**Boolean Literals:**
```petra
true
false
```

### Operators

**Arithmetic:**
```
+  -  *  /  %
```

**Comparison:**
```
==  !=  <  <=  >  >=
```

**Logical:**
```
&&  ||  !
```

**Other:**
```
=   -- assignment
->  -- match arm, function return indicator
?   -- error propagation
@   -- instance field reference
&   -- address-of
*   -- pointer dereference (postfix: ptr*)
|   -- lambda parameter delimiter
```

### Delimiters

```
(  )  -- parentheses
{  }  -- braces (dynamic collections)
[  ]  -- brackets (arrays)
,     -- comma
:     -- type annotation
;     -- statement separator (optional)
.     -- member access
..    -- range operator
```

---

## Type System

### Primitive Types

```petra
Int     -- signed integer (platform-dependent, typically 64-bit)
i8      -- 8-bit signed integer
i16     -- 16-bit signed integer
i32     -- 32-bit signed integer
i64     -- 64-bit signed integer
u8      -- 8-bit unsigned integer
u16     -- 16-bit unsigned integer
u32     -- 32-bit unsigned integer
u64     -- 64-bit unsigned integer
Float   -- 64-bit floating point
f32     -- 32-bit floating point
f64     -- 64-bit floating point
Bool    -- boolean (true/false)
String  -- UTF-8 string
Char    -- single Unicode character
```

### Compound Types

**Tuples:**
```petra
(Int, String)           -- tuple of int and string
(Float, Float, Float)   -- 3D point as tuple
```

**Arrays (fixed size):**
```petra
[Int; 5]                -- array of 5 integers
[String; 10]            -- array of 10 strings
```

**Lists (dynamic size):**
```petra
List<Int>               -- dynamic list of integers
List<String>            -- dynamic list of strings
```

**Pointers:**
```petra
*Int                    -- pointer to integer
*String                 -- pointer to string
```

### Type Annotations

Variables and function parameters can have explicit type annotations:

```petra
var x: Int = 5
const name: String = "Alice"

func add(a: Int, b: Int): Int
    return a + b
end
```

Type inference is supported for local variables:

```petra
var x = 5              -- inferred as Int
var name = "Alice"     -- inferred as String
```

---

## Variables and Constants

### Variable Declaration

```petra
var x = 5              -- mutable variable (inferred type)
var y: Int = 10        -- mutable variable (explicit type)

x = 15                 -- OK: x is mutable
```

### Constant Declaration

```petra
const PI = 3.14159     -- immutable constant (inferred type)
const MAX: Int = 100   -- immutable constant (explicit type)

PI = 3.14              -- ERROR: PI is immutable
```

---

## Functions

### Function Declaration

```petra
func function_name(param1: Type1, param2: Type2): ReturnType
    -- function body
    return value
end
```

Example:
```petra
func add(a: Int, b: Int): Int
    return a + b
end

func greet(name: String): String
    return "Hello, {name}!"
end
```

### Named Parameters

Function calls use named parameters for clarity:

```petra
func divide(dividend: Int, divisor: Int): Int
    return dividend / divisor
end

var result = divide(dividend: 10, divisor: 2)
```

### Multiple Return Values (Tuples)

```petra
func divide_with_remainder(dividend: Int, divisor: Int): (Int, Int)
    var quotient = dividend / divisor
    var remainder = dividend % divisor
    return (quotient, remainder)
end

var (q, r) = divide_with_remainder(dividend: 10, divisor: 3)
```

---

## Control Flow

### If Statement

```petra
if condition
    -- code
end

if condition
    -- code
else
    -- code
end

if condition1
    -- code
elif condition2
    -- code
elif condition3
    -- code
else
    -- code
end
```

Example:
```petra
func classify_age(age: Int): String
    if age < 13
        return "child"
    elif age < 20
        return "teenager"
    elif age < 65
        return "adult"
    else
        return "senior"
    end
end
```

### Match Expression

Pattern matching with exhaustiveness checking:

```petra
match value
    case pattern1 -> expression1
    case pattern2 -> expression2
    case _ -> default_expression
end
```

Example:
```petra
func factorial(n: Int): Int
    return match n
        case 0 -> 1
        case n -> n * factorial(n - 1)
    end
end
```

Match can also be used with complex patterns:

```petra
match result
    case Result::Ok(value) -> 
        println(message: "Success: {value}")
        value
    case Result::Error(err) ->
        println(message: "Error: {err}")
        0
end
```

### While Loop

```petra
while condition
    -- loop body
end
```

Example:
```petra
var i = 0
while i < 10
    println(message: "i = {i}")
    i = i + 1
end
```

### For Loop

**For-in loop:**
```petra
for item in collection
    -- loop body
end
```

Example:
```petra
var numbers = {1, 2, 3, 4, 5}
for num in numbers
    println(message: "Number: {num}")
end
```

**C-style for loop:**
```petra
for init; condition; increment
    -- loop body
end
```

Example:
```petra
for i = 0; i < 10; i = i + 1
    println(message: "i = {i}")
end
```

### Break and Continue

```petra
while true
    if should_exit
        break
    end
    
    if should_skip
        continue
    end
    
    -- process
end
```

---

## Structs

### Struct Definition

```petra
struct StructName
    field field1: Type1
    field field2: Type2
    
    func new(field1: Type1, field2: Type2): Self
        @field1 = field1
        @field2 = field2
        return Self
    end
    
    method method_name(param: Type): ReturnType
        -- method body
        return value
    end
end
```

Example:
```petra
struct Rectangle
    field width: Float
    field height: Float
    
    func new(width: Float, height: Float): Self
        @width = width
        @height = height
        return Self
    end
    
    method area(): Float
        return @width * @height
    end
    
    method scale(factor: Float): Self
        return Rectangle::new(
            width: @width * factor,
            height: @height * factor
        )
    end
end
```

### Struct Instantiation

```petra
var rect = Rectangle::new(width: 10.0, height: 5.0)
var area = rect.area()
```

### Field Access

Use `@` to access instance fields within methods:

```petra
method get_width(): Float
    return @width
end
```

---

## Generics

### Generic Structs

```petra
struct Box<T>
    field value: T
    
    func new(value: T): Self
        @value = value
        return Self
    end
    
    method get(): T
        return @value
    end
end

var int_box = Box<Int>::new(value: 42)
var str_box = Box<String>::new(value: "hello")
```

### Generic Functions

```petra
func identity<T>(value: T): T
    return value
end

var x = identity<Int>(value: 5)
var y = identity<String>(value: "hello")
```

---

## Traits

### Trait Definition

```petra
trait TraitName
    method method1(param: Type): ReturnType
    method method2(): ReturnType
end
```

Example:
```petra
trait Printable
    method to_string(): String
end

trait Add<T>
    method add(other: T): Self
end
```

### Trait Implementation

```petra
struct Rectangle: Printable
    field width: Float
    field height: Float
    
    method to_string(): String
        return "Rectangle({@width} x {@height})"
    end
end
```

### Multiple Traits

```petra
struct Vector3: Printable, Add<Vector3>
    field x: Float
    field y: Float
    field z: Float
    
    method to_string(): String
        return "Vector3({@x}, {@y}, {@z})"
    end
    
    method add(other: Vector3): Vector3
        return Vector3::new(
            x: @x + other.x,
            y: @y + other.y,
            z: @z + other.z
        )
    end
end
```

### Operator Overloading via Traits

Built-in traits for operator overloading:

```petra
trait Add<T>
    method add(other: T): Self
end

trait Sub<T>
    method sub(other: T): Self
end

trait Mul<T>
    method mul(other: T): Self
end

trait Div<T>
    method div(other: T): Self
end

trait Eq
    method equals(other: Self): Bool
end
```

Usage:
```petra
var v1 = Vector3::new(x: 1.0, y: 2.0, z: 3.0)
var v2 = Vector3::new(x: 4.0, y: 5.0, z: 6.0)
var v3 = v1 + v2  -- Desugars to v1.add(v2)
```

---

## Error Handling

### Result Type

Errors are values using the `Result` type:

```petra
enum Result<T, E>
    Ok(T)
    Error(E)
end
```

Example:
```petra
func divide(dividend: Int, divisor: Int): Result<Int, String>
    if divisor == 0
        return Result::Error("Division by zero")
    end
    return Result::Ok(dividend / divisor)
end
```

### Option Type

For optional values:

```petra
enum Option<T>
    Some(T)
    None
end
```

### Question Mark Operator

Early return on error:

```petra
func process(): Result<Int, String>
    var x = might_fail()?        -- Returns Error if might_fail fails
    var y = also_might_fail()?   -- Returns Error if also_might_fail fails
    return Result::Ok(x + y)
end
```

The `?` operator:
- If the result is `Ok(value)`, unwraps to `value`
- If the result is `Error(err)`, returns `Error(err)` from the function

---

## Memory Management

### Defer

Deterministic resource cleanup:

```petra
func process_file(path: String): Result<(), String>
    var file = File::open(path)?
    defer file.close()  -- Guaranteed to run when function exits
    
    var contents = file.read_all()?
    -- process contents
    
    return Result::Ok(())
end
```

Multiple defers execute in reverse order (LIFO):

```petra
func example()
    defer println(message: "Third")
    defer println(message: "Second")
    defer println(message: "First")
    return
end
-- Prints: First, Second, Third
```

### Pointers

Manual pointer manipulation when needed:

```petra
func increment(ptr: *Int)
    ptr* = ptr* + 1  -- Dereference with postfix *
end

var x = 5
increment(ptr: &x)   -- Pass address with &
println(message: "x = {x}")  -- x = 6
```

Pointer syntax:
- `*Type` - pointer type
- `&value` - address-of operator
- `ptr*` - dereference operator (postfix)

---

## Collections

### Lists (Dynamic)

```petra
var numbers = {1, 2, 3, 4, 5}  -- List<Int>
var names = {"Alice", "Bob"}    -- List<String>

numbers.push(6)
var first = numbers[0]
```

### Arrays (Fixed Size)

```petra
var fixed: [Int; 5] = [1, 2, 3, 4, 5]
var matrix: [[Int; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]
```

---

## Lambdas

### Lambda Syntax

```petra
|param1: Type1, param2: Type2| expression end

|param: Type|
    -- multi-line body
    return value
end
```

Example:
```petra
var doubled = numbers.map(|n: Int| n * 2 end)

var filtered = items.filter(|item: Item|
    var score = item.calculate_score()
    return score > 10
end)
```

---

## String Interpolation

Embed values in strings:

```petra
var name = "Alice"
var age = 30
println(message: "Hello, {name}! You are {age} years old.")
```

Expressions are allowed:

```petra
var x = 5
var y = 10
println(message: "Sum: {x + y}")
```

**Restriction:** No method calls inside interpolation. Compute values first:

```petra
-- NOT ALLOWED:
-- println(message: "Area: {square.area()}")

-- Instead:
var area = square.area()
println(message: "Area: {area}")
```

---

## Modules and Imports

### Module Declaration

```petra
-- File: geometry.petra
module Geometry

struct Point
    field x: Float
    field y: Float
end
```

### Importing

```petra
-- Import specific items
import Geometry::{Point, Line}

-- Import everything from module
import Geometry

-- Use imported types
var p = Point::new(x: 1.0, y: 2.0)
```

---

## Standard Library Types

### Result<T, E>

```petra
enum Result<T, E>
    Ok(T)
    Error(E)
end

-- Methods:
-- is_ok(): Bool
-- is_error(): Bool
-- unwrap(): T  (panics if Error)
-- unwrap_or(default: T): T
```

### Option<T>

```petra
enum Option<T>
    Some(T)
    None
end

-- Methods:
-- is_some(): Bool
-- is_none(): Bool
-- unwrap(): T  (panics if None)
-- unwrap_or(default: T): T
```

### List<T>

```petra
-- Methods:
-- push(item: T): ()
-- pop(): Option<T>
-- get(index: Int): Option<T>
-- length(): Int
-- map<U>(f: |T| U end): List<U>
-- filter(f: |T| Bool end): List<T>
-- reduce<U>(f: |U, T| U end, initial: U): U
```

---

## Examples

### Hello World

```petra
func main(args: List<String>): Int
    println(message: "Hello, world!")
    return 0
end
```

### Factorial

```petra
func factorial(n: Int): Int
    return match n
        case 0 -> 1
        case n -> n * factorial(n - 1)
    end
end
```

### Struct with Methods

```petra
struct Person
    field name: String
    field age: Int
    
    func new(name: String, age: Int): Self
        @name = name
        @age = age
        return Self
    end
    
    method greet(): String
        return "Hello, my name is {@name} and I'm {@age} years old."
    end
    
    method is_adult(): Bool
        return @age >= 18
    end
end

func main(args: List<String>): Int
    var person = Person::new(name: "Alice", age: 30)
    println(message: person.greet())
    
    if person.is_adult()
        println(message: "Alice is an adult")
    end
    
    return 0
end
```

### Error Handling

```petra
func divide(dividend: Int, divisor: Int): Result<Int, String>
    if divisor == 0
        return Result::Error("Division by zero")
    end
    return Result::Ok(dividend / divisor)
end

func process(): Result<Int, String>
    var x = divide(dividend: 10, divisor: 2)?
    var y = divide(dividend: 20, divisor: 4)?
    return Result::Ok(x + y)
end

func main(args: List<String>): Int
    var result = process()
    
    match result
        case Result::Ok(value) ->
            println(message: "Result: {value}")
            return 0
        case Result::Error(err) ->
            println(message: "Error: {err}")
            return 1
    end
end
```

### Generics and Traits

```petra
trait Shape
    method area(): Float
end

struct Circle: Shape
    field radius: Float
    
    func new(radius: Float): Self
        @radius = radius
        return Self
    end
    
    method area(): Float
        return 3.14159 * @radius * @radius
    end
end

struct Rectangle: Shape
    field width: Float
    field height: Float
    
    func new(width: Float, height: Float): Self
        @width = width
        @height = height
        return Self
    end
    
    method area(): Float
        return @width * @height
    end
end

func print_area<T: Shape>(shape: T)
    var area = shape.area()
    println(message: "Area: {area}")
end

func main(args: List<String>): Int
    var circle = Circle::new(radius: 5.0)
    var rect = Rectangle::new(width: 4.0, height: 6.0)
    
    print_area<Circle>(shape: circle)
    print_area<Rectangle>(shape: rect)
    
    return 0
end
```

---

## Future Considerations (Post v0.1)

These features are being considered for future versions:

- Concurrency primitives (threads, async/await)
- Advanced memory management options (arenas, custom allocators)
- Macros and metaprogramming
- Package manager
- FFI (Foreign Function Interface) for C interop
- SIMD support
- More sophisticated pattern matching (guards, OR patterns)

---

## Grammar Summary (EBNF-style)

```
program = module_decl? (import_decl)* (top_level_item)*

module_decl = "module" identifier

import_decl = "import" path ("::" "{" identifier_list "}")?

top_level_item = func_decl | struct_decl | trait_decl

func_decl = "func" identifier "(" param_list? ")" ":" type
            statement*
            "end"

struct_decl = "struct" identifier ("<" type_params ">")? (":" trait_list)?
              struct_member*
              "end"

struct_member = field_decl | func_decl | method_decl

field_decl = "field" identifier ":" type

method_decl = "method" identifier "(" param_list? ")" ":" type
              statement*
              "end"

trait_decl = "trait" identifier
             method_signature*
             "end"

statement = var_decl | const_decl | assignment | return_stmt | 
            if_stmt | match_expr | while_loop | for_loop |
            defer_stmt | expr_stmt

var_decl = "var" identifier (":" type)? "=" expr

const_decl = "const" identifier (":" type)? "=" expr

if_stmt = "if" expr
          statement*
          ("elif" expr statement*)*
          ("else" statement*)?
          "end"

match_expr = "match" expr
             ("case" pattern "->" expr)*
             "end"

while_loop = "while" expr
             statement*
             "end"

for_loop = "for" identifier "in" expr
           statement*
           "end"

type = primitive_type | identifier | pointer_type | 
       generic_type | tuple_type | array_type

pointer_type = "*" type

generic_type = identifier "<" type_list ">"

tuple_type = "(" type_list ")"

array_type = "[" type ";" integer "]"
```

---

## Compilation

Petram compiles to C code, which is then compiled to native machine code using a C compiler (GCC, Clang, etc.).

Compilation pipeline:
1. **Lexing** - Source code → Tokens
2. **Parsing** - Tokens → AST (Abstract Syntax Tree)
3. **Type Checking** - Validate types, resolve generics
4. **Code Generation** - AST → C code
5. **C Compilation** - C code → Native binary

---

End of Specification