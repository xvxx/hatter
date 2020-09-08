# Hatter

<img src="./img/rhetoric.jpg" align="right" width="350" alt="The Mad Hatter discussing Hatter" />

<em>~ **Hatter** is a positively mad, HTML templating language. ~</em>

## Features

- Tag shorthands (`<div#id>`, `<div.class1.class2>`, `<input@name:type>`)
- Implicit divs (`<#main>` becomes `<div id='main'>`)
- `<html>` gets added if the first tag is `<head>`
- Auto-closing tags through Python-ish indentation
- `for` loops over lists and maps
- `if else` statements
- Call functions defined in Rust
- Easy inline JavaScript

### Tag shorthand

- `<div#id>`
- `<div.class1.class2>`
- `<input@name>`
- `<input:type>`

### Implicit divs

- `<#main> -> <div id='main'>`
- `<html>` gets added if the first tag is `<head>`

### Auto-closing tags through indentation

```
<ul#items>
  <li.item>
    Item 1
  <li.item> <i> Item 2
  <li.item> <b>Item</> 3
```

### `for` loops

```
<ul#people>
  for name in names
    <li.person> name
```

### `if else` statements

```
<div>
  if show-hint?
    <.hint> <b>Psst...</> You can type `?` for help.
  else
    <p.nada> Nothing to see here.
```

### Functions defined in Rust

```
<ul>
  <li> <b>Name:</> to-titlecase("jonny idaho")
  <li> <b>Location:</> to-titlecase("IDAHO!")
  <li>
    <b>Age:
    add(20, mul(4, 10))

```

### Easy inline JavaScript

```
<ul>
    <li> <a onclick=(alert("Clicked me."))> Click me
    <li> <a onclick=(alert("Oink!"))> 🐷
    <li> <a onclick=(history.back())> Go back
```

## TODO

- [x] if
  - [x] else
  - [ ] else ifs
- [ ] \<style> tag
- [ ] \<script> tag
- [ ] string interpolation
- [ ] operators
- [ ] VSCode Extension
- [ ] VSCode + luacheck-style LSP
- [ ] bomb-ass test suite
- [ ] show error location when compiling
- [ ] luacheck-style tool
- [ ] <!-- html comments -->
- [ ] shortcut interpolation
      (ex: \<div .{name}> -> \<div class="dog"> when name="dog")
- [ ] def <tag>
- [ ] def fn?
