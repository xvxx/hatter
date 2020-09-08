# Hatter

<img src="./img/rhetoric.jpg" align="right" width="350" alt="The Mad Hatter discussing Hatter" />

<em>~ **Hatter** is a positively mad, HTML templating language. ~</em>

## Features

- Tag shorthand:
  - `<div#id>`
  - `<div.class1.class2>`
  - `<input@name>`
  - `<input:type>`
- Implicit div:
  - `<#main> -> <div id='main'>`
- Auto-closing tags through indentation:

```
<ul#items>
  <li.item>
    Item 1
  <li.item> <i> Item 2
  <li.item> <b>Item</> 3
```

- `for` loops:

```
<ul#people>
  for name in names
    <li.person> name
```

- `if else` statements:

```
<div>
  if show-hint?
    <.hint> <b>Psst...</> You can type `?` for help.
```

## TODO

- [x] if
  - [x] else
  - [ ] else ifs
- [ ] \<style> tag
- [ ] \<script> tag
- [ ] string interpolation
- [ ] VSCode Extension
- [ ] VSCode + luacheck-style LSP
- [ ] show error location when compiling
- [ ] luacheck-style tool
- [ ] <!-- html comments -->
- [ ] shortcut interpolation
      (ex: \<div .{name}> -> \<div class="dog"> when name="dog")
- [ ] def <tag>
- [ ] operators
- [ ] def fn?
