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

-- This also works.
var result_swapped_args = divide(divisor: 2, dividend: 10)
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

Structs contain only data definitions. Methods and functions are defined separately.

```petra
struct StructName
    field1: Type1
    field2: Type2
end
```

Example:
```petra
struct Rectangle
    width: Float
    height: Float
end
```

### Invariants

Structs can have invariants that constrain their fields. Each invariant clause is checked separately for precise error messages.

```petra
struct Square
    side_length: u32
    
    invariant
        side_length > 0
        side_length <= Square::MAX_SIDE_LENGTH
    end
end
```

When a struct has invariants:
- Any constructor automatically returns `Result<Self, String>` instead of `Self`
- Any method that mutates fields automatically returns `Result<(), String>` instead of `()`
- The compiler inserts runtime checks for all invariants
- Each invariant is checked individually with specific error messages

```petra
var sq = Square::new(side_length: -1)
// Error: Invariant violated: side_length > 0 (got -1)

var sq2 = Square::new(side_length: 2000)
// Error: Invariant violated: side_length <= Square::MAX_SIDE_LENGTH (got 2000)
```

### Associated Functions

Associated functions are defined at module level using the `Type::function_name` syntax. These are typically used for constructors and other type-related utilities.

```petra
func Square::new(side_length: u32): Result<Square, String>
    return Result::Ok(Square{ side_length: side_length })
    // Compiler automatically inserts invariant checks
end

func Square::from_area(area: u32): Result<Square, String>
    var side = sqrt(area)
    return Square::new(side_length: side)
end
```

### Instance Methods

Instance methods use receiver syntax similar to Go. The receiver parameter comes before the function name.

**Immutable receiver** (pass by value):
```petra
func (r: Rectangle) area(): Float
    return r.width * r.height
end
```

**Mutable receiver** (pass by pointer):
```petra
func (r: *Rectangle) scale(factor: Float)
    r.width = r.width * factor
    r.height = r.height * factor
end
```

The `*` in the receiver position indicates the method can mutate the struct. This is purely about mutability - Petram uses garbage collection, so no manual memory management is required.

### Struct Instantiation and Method Calls

```petra
// Associated function call (constructor)
var rect = Rectangle::new(width: 10.0, height: 5.0)

// Instance method calls
var area = rect.area()           // immutable method
rect.scale(factor: 2.0)          // mutable method
```

### Associated Constants

Constants can be namespaced to types:

```petra
const Square::MAX_SIDE_LENGTH: u32 = 1000

struct Square
    side_length: u32
    
    invariant
        side_length <= Square::MAX_SIDE_LENGTH
    end
end
```

---

## Generics

### Generic Structs

```petra
struct Box<T>
    value: T
end

func Box::new<T>(value: T): Box<T>
    return Box{ value: value }
end

func (b: Box<T>) get(): T
    return b.value
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
    func (self: Self) method1(param: Type): ReturnType
    func (self: Self) method2(): ReturnType
end
```

Example:
```petra
trait Printable
    func (self: Self) to_string(): String
end

trait Add<T>
    func (self: Self) add(other: T): Self
end
```

### Trait Implementation

```petra
struct Rectangle: Printable
    width: Float
    height: Float
end

func (r: Rectangle) to_string(): String
    return "Rectangle({r.width} x {r.height})"
end
```

### Multiple Traits

```petra
struct Vector3: Printable, Add<Vector3>
    x: Float
    y: Float
    z: Float
end

func (v: Vector3) to_string(): String
    return "Vector3({v.x}, {v.y}, {v.z})"
end

func (v: Vector3) add(other: Vector3): Vector3
    return Vector3{
        x: v.x + other.x,
        y: v.y + other.y,
        z: v.z + other.z
    }
end
```

### Operator Overloading via Traits

Built-in traits for operator overloading:

```petra
trait Add<T>
    func (self: Self) add(other: T): Self
end

trait Sub<T>
    func (self: Self) sub(other: T): Self
end

trait Mul<T>
    func (self: Self) mul(other: T): Self
end

trait Div<T>
    func (self: Self) div(other: T): Self
end

trait Eq
    func (self: Self) equals(other: Self): Bool
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
    name: String
    age: Int
end

func Person::new(name: String, age: Int): Person
    return Person{ name: name, age: age }
end

func (p: Person) greet(): String
    return "Hello, my name is {p.name} and I'm {p.age} years old."
end

func (p: Person) is_adult(): Bool
    return p.age >= 18
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
    func (self: Self) area(): Float
end

struct Circle: Shape
    radius: Float
end

func Circle::new(radius: Float): Circle
    return Circle{ radius: radius }
end

func (c: Circle) area(): Float
    return 3.14159 * c.radius * c.radius
end

struct Rectangle: Shape
    width: Float
    height: Float
end

func Rectangle::new(width: Float, height: Float): Rectangle
    return Rectangle{ width: width, height: height }
end

func (r: Rectangle) area(): Float
    return r.width * r.height
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

top_level_item = func_decl | associated_func_decl | method_decl | struct_decl | trait_decl | const_decl

associated_func_decl = "func" identifier "::" identifier "(" param_list? ")" ":" type
                       statement*
                       "end"

method_decl = "func" "(" receiver ")" identifier "(" param_list? ")" (":" type)?
              statement*
              "end"

receiver = identifier ":" "*"? type

func_decl = "func" identifier "(" param_list? ")" ":" type
            statement*
            "end"

struct_decl = "struct" identifier ("<" type_params ">")? (":" trait_list)?
              struct_member*
              "end"

struct_member = field_decl | invariant_block

field_decl = identifier ":" type

invariant_block = "invariant"
                  (expression)*
                  "end"

const_decl = "const" (identifier "::")? identifier ":" type "=" expr

trait_decl = "trait" identifier
             method_signature*
             "end"

method_signature = "func" "(" "self" ":" "Self" ")" identifier "(" param_list? ")" ":" type

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