# Hatter in a Hurry

In a hurry? If you already know how to program, you already know
Hatter. And if you already know both Python and HTML, well, you're an
expert, kid. Maybe you should be teaching me.

## Blocks

Like Python, CoffeeScript, Nim, and Imba, Hatter is a
whitespace-sensitive language. Blocks are defined by their indentation
level. Any increase in indentation will open a new block, and mixing
and spaces in the same file produces an error.

```hatter
# Blocks in action.
num := ask("Give me a small number: ").to_int()

if num > 0 && num < 100
    print("One")
    for i in 1..num
        print("and a' {i}")
    print("Go!")
else if num >= 100
    print("That's too much.")
else
    print("I've got nothing to say about that.")
```

## Comments

Comments start from `#` and go to the end of the line. Nothing fancy:

```hatter
# This is a comment. Excitement.
```

## Types

There are only a few basic types in Hatter

```hatter
# Bool
false
true

# Number
200
-10_150_203
0b101
0o123
0xdeadbeef
3.14
-102.123

# String
"Heya pal!"
'Also, hi.'
`Also, hello.`
"""
    Triple version of ', ", and `
    works for multi-line strings.
"""
"Double quoted strings are interpolated: {2 + 2}" # <- This will be 4

# List
[1, 2, 3]
["John", "Paul", "George", "Ringo"]
[true, 2, "Paul"] # dynamic language, lists can be mixed types

# Map
{ one: "one", two: "two" }
{ 0: "oh", 1: "also one" }

# Fn
fn(x) return x + 1
```

## Variables

Like Go, variables are created using `:=` and updated using `=`.
Using `:=` when a variable already exists in the nearest scope is an
error, and using `=` when a variable doesn't already exist is an
error. Using `=` with a value of a different type will also raise an
error.

```hatter
num := 123
num = 456       # ok
num = '789'     # error, wrong type
num := 789      # error, exists
rand = "random" # error, doesn't exist
```

## Functions

Functions are set with the `def` keyword and always invoked with `()`.
Arity matters to Hatter: if you pass in too many or too few arguments
to a function you'll get an error:

```hatter
def greet(title, name)
    print("Hiya, {title}. {name}!")

greet("Mrs", "Robinson")           #=> Hiya, Mrs. Robinson!
greet("Mrs", "Robinson", "Crusoe") #=> error, wrong nuber of arguments
```

Use `return` to return a value:

```hatter
def mod(num, by, msg)
    if num % by == 0
        return msg
    else
        return ""

def fizz-buzz
    for i in 1..101
        print(mod(i, 3, 'Fizz') + mod(i, 5, 'Buzz'))
```

## Flow

Hatter uses a basic indented `if`/`else` structure which can have as
many `else if` clauses as you need:

```hatter
if i > 0
    print("Positive")
else if i == 0
    print("Cero")
else if i < 0
    print("Negative")
else if i > 100_000_000
    print("Way TOO Positive!")
```

Like most languages, Hatter includes `&&` and `||` which are both
short-circuiting. It also includes a `!` prefix operator:

```hatter
!true            #=> false
true && true     #=> true
happy? || sad?   #=> depends, i guess
```

## Loops

Hatter has three types of loops: a `for` loop over `List`, a `for`
loop for `Map`, and a basic `while` loop.

Both the `List` and `Map` loops take either one or two variables to
fill with either the value alone or the key/index and value:

```hatter
for v in [100, 200, 300]
    p(v) #=> 100 then 200 then 300

for i, v in [100, 200, 300]
    p(i) #=> 0 then 1 then 2

for k, v in { first: 1, second: 2 }
    print("{k} is {v}") #=> `first is 1` then `second is 2`

while true
    print("O'DOYLE RULES!")
```

All three loop types support the standard `break` and `continue`
keywords:

```hatter
for v in 100..500
    print(v)
    if v > 300
        break
```

## Errors

Hatter has a simple error model borrowed from Lua: any time the
`error()` function is called or an internal error occurs, such as
trying to use `:=` to override an existing variable, the program will
halt and an error message will be displayed. Calling a function with
`try()` disables this behavior and instead returns a "return" object
that describes whether the call succeeded error-free or not:

```hatter
def greet(name)
    error("Panic!")  # This raises an error.
    print("Hi there, {name}!")

greet("Mrs. Robison")       # error, program halts

ret := try(greet, "Mr. Robinson Crusoe")
if ret.err
    print ret.err  # Prints "Panic!"
```
