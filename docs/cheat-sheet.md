# Hatter Cheat Sheet

```hatter
# This is a comment.

## Types

# bool
false
true

# int
200
-10_150_203
0b101
0o123
0xdeadbeef

# float
3.14
-102.123

# string
"Heya pal!"
'Also, hi.'
`Also, hello.`
"""
    Triple version of ', ", and `
    works for multi-line strings.
"""
"Double quoted strings are interpolated: {2 + 2}" # <- This will be 4

# list
[1, 2, 3]
["John", "Paul", "George", "Ringo"]
[true, 2, "Paul"] # dynamic language, lists can be mixed types

# map
{ one: "one", two: "two" }
{ 0: "oh", 1: "also one" }

# fn
fn(x) return x + 1

## Variables

num := 123      # create var
num = 456       # update var
num = '789'     # error, wrong type
num := 789      # error, exists
rand = "random" # error, doesn't exist

## Functions

def greet(title, name)
    print("Hiya, {title}. {name}!")

greet("Mrs", "Robinson")           #=> Hiya, Mrs. Robinson!
greet("Mrs", "Robinson", "Crusoe") #=> error, wrong nuber of arguments

# Use `return` to return a value:

def mod(num, by, msg)
    if num % by == 0
        return msg
    else
        return ""

def fizz-buzz
    for i in 1..101
        print(mod(i, 3, 'Fizz') + mod(i, 5, 'Buzz'))

## if / else

if i > 0
    print("Positive")
else if i == 0
    print("Cero")
else if i < 0
    print("Negative")
else if i > 100_000_000
    print("Way TOO Positive!")

not true         #=> false
true and true    #=> true
happy? or sad?   #=> depends, i guess

## Loops

for v in [100, 200, 300]
    p(v) #=> 100 \n 200 \n 300

for i, v in [100, 200, 300]
    p(i) #=> 0 \n 1 \n 2

for k, v in { first: 1, second: 2 }
    print("{k} is {v}") #=> first is 1 \n second is 2

while true
    print("O'DOYLE RULES!") #=> he does

for v in 100..500
    if v > 300
        break
    else if v % 2 == 0
        continue
    else
        print(v)

## Errors

def combine(a, b)
    error("Panic!")  # This raises an error.
    concat(a, b)

combine("Mrs.", "Robison")       # error, program halts
ret := try(combine, "Mr.", "Robinson Crusoe")
if ret.err
    print ret.err  # Prints "Panic!"
else
    # it worked
```
