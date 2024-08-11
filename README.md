# Petram: Chiseled for Clarity

## What is Petram?

Petram is a general-purpose programming language with verbosity as a feature. It aims to be extremely strict, rigid, but still petraonomic and expressive enough to the point where writing code in it becomes effortless.

**<u>Petram IS STILL IN ITS INFANCY!</u>**

I'm still very much in the middle of refining the syntax and capabilities of the langauge, despite having laid the foundational guidelines. There's not even an implementation yet! It'll be an interpreted language first, and later on it will become compiled, though we might still keep the interpreted version around.

## What is the point of Petram?

**None!** While I would love it if this project saw some adoption, I am fully aware that it's quirky and could definitely be seen as counterintuitive by some. Let me list what Petram is ****not****
  
* It is **NOT** a C killer or anything
* It is **NOT** meant to be extremely popular or to replace any existing workflow
* It is **NOT** meant to cater to already existing languages and their syntactical principles.

I'm developing Petram as a passion project for myself first, and others second. I have firm principles on how it should operate and what its purpose is, and I will maintaint its "quirks" as much as possible (within reason, of course).
It's an excercise in language design, lexing, parsing, and integrating into a compiler backed ([QBE tentatively](https://c9x.me/compile/))

## Yeah okay, show me some code examples

```petra
-- hello_world.petra

-- This is a single line comment.
-- The `main` function is the entry point of a program, similar to most others out there.
-- It takes a list of strings as command line arguments, again, similar to most others out there.
-- It returns either nothing (Void) or an Int as per Unix convention.
-- Here, we use Void and print "Hello, World!" to stdout.
func #[main :: args: List<String>]#: Void ->
    #[println :: message: "Hello, World!\n"]#
```

```petra
-- variables.petra

-- Variables are prefixed with a dollar sign `$`, similar to PowerShell, Perl, and PHP.
-- This is to disambiguate locally scoped variables from function parameters or struct fields.

func #[main :: args: List<String>]#: Int ->
    -- There are two types of assignment, inspired by languages such as Go, Jai, and Odin.
    -- The first one is the implicit type inferrence operator `:=`
    $retval := 0

    -- The second one is where you explicitly declare the type of the variable.
    -- Note the separation of the colon `:` from the equals sign `=`
    -- Naturally, variables being what they are, they can be re-assigned, provided their type remains consistent
    $retval: Int = 0

    -- Here's some string interpolation too! Simply use curly braces {} with your variable name to interpolate.
    #[println :: message: "The return value is {$retval}"]#

    -- This will return 0, indicating normal program termination
    return $retval

```

```petra
-- functions.petra

{-
    Here's a multiline comment. We will expand this to allow for documenting arguments and return types.
    This is a regular function definition.

    There are two ways to define a function:
    1. Regular block expressions
    2. Lambda (or single line) functions
-}

func #[add_ints :: first: Int, second: Int]#: Int ->
    return first + second

func #[add_ints_lambda :: first: Int, second: Int]#: Int => first + second

func #[main :: args: List<String>]#: Void ->
    -- Any _expression_ is enclosed in #[]#, same as definitions. This is to disambiguate them from statements.
    -- Also note that when calling functions, you _must_ provide the parameter names. This is similar to Jakt's approach, though admittedly less flexible.
    $first_result := #[add_ints :: first: 1, second: 2] -- $first_result == 3

    $second_result := #[add_ints_lambda :: first: 2, second: 3]# -- $second_result == 5

```

```petra
-- structs.petra
{-
    Here's a moderately complex struct with a constraint.
    A constraint is a mechanism that will not allow you to instantiate a struct if the check fails.
    Constraints is a metadata structure that is accessible from within the struct. You can access it with
    Self::constraints.<field name>
-}
struct #[Square]# ->
    -- All struct members are declared with `field`
    field side_length: Int

    -- This is an optional constraint if you want to ensure that your struct cannot be created with improper data.
    -- As mentioned above, you can access the special constraints metadata with `Self.constraints.<constraint_name>`
    -- This is actually a function that runs whatever you defined as the constraint. In this instance, it will run side_length > 0.
    -- Naturally, constraints must resolve to a Bool.
    constrain side_length: Int where #[side_length > 0]#
        message: "Side of a square must be greater than 0."

    -- `new` is one of the rare cases of syntactic sugar for constructing structs.
    -- Currently, you must specify `Self` as the return type, but that will most likely change since it's obvious what type you're instantiating.
    new #[side_length: Int]#: Self ->
        -- Field names are accessed with @, like in Ruby. It's essentially the equivalent to `this` in other languges.
        @side_length = side_length
        -- Constraints are automatically checked here

    -- Struct can have methods, and as we'll see below, they may or may not accept arguments.
    -- This method doesn't.
    method #[calculate_area]#: Int ->
        return @side_length * @side_length

    -- But this one does! We're getting a bit ahead of ourselves here but Petram has no concept of null or nil,
    -- just like Rust. It's called the "Billion dollar mistake" for a reason. Instead, we use the tried and true `Option<T, E>` type that you can match on. More on that later.
    method #[set_side_length :: new_length: Int]#: Option<(), SizeError> ->
        -- Constraints are automatically checked here too
        match Self::constraints.side_length(new_length) ->
            true -> 
                @side_length = new_length
                Option::Some(())
            _ -> Option::Error(SizeError)

    func #[main :: args: List<String>]#: Int ->
        -- Instantiate a struct, which can potentially fail
        -- Here, the type is inferred to be `$square: Option<Square, SizeError>`
        $square :=
        #[
            match #[Square :: side_length: 5]# ->
                Option::Some($square) ->
                    #[println :: message: "Square created successfully"]#
                    Option::Some($square)
                Option::None($error) ->
                    #[println :: message: "Error: {$error}"]#
                    Option::None
        ]#

        match #[Square::new :: side_length: -1]# ->
            Option::Some(_) ->
                return 1 -- should never happen
            Option::None($error) ->
                #[println :: message: "Error as expected: {$error}"]#
        
        return 0
```
