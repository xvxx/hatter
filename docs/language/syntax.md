# The Syntax of Hatter

Like Python, CoffeeScript, Nim, and Imba, Hatter is an
indentation-sensitive language. Blocks are defined not with `{}` curly
braces or `[]` brackets, but through whitespace. You make your code
look like how your code should look, in theory, and it all just works,
in theory! The essence of good programming.

## Blocks

You can use either tabs or spaces to indent your code, but not both -
Hatter will fail with an error if it senses your indecision in
choosing a path.

```hatter
#!/usr/bin/env hatter -s
num := ask("What's a cool number?").to_int()
if 2 + 2 > num
    print("Too low!")
else
    oos := repeat("o", num)
    print("Co{oos}ol!")
```

The amount you indent per block doesn't matter, as long as it's more
than the current indentation level. Different arms of an `if`/`else`
statement, for example, can have different indentation levels, even
though it's awful:

```hatter
if 2 + 2 > num
    print("Too low!")
else
  print("Co{repeat('o', num)}ol!")
```


