# Hatter

<img src="./img/rhetoric.jpg" align="right" width="350" alt="The Mad Hatter discussing Hatter" />

<em>~ **Hatter** is a positively mad HTML templating language. ~</em>

## Features

- Tag shorthand:
  - `<div#id>`
  - `<div.class1.class2>`
  - `<input@name>`
  - `<input:type>`
- Implicit div:
  - `<#main> -> <div id='main'>`
- Auto-closing tags through indentation:

```html
<ul#items>
  <li.item>
    Item 1
    <li.item>
      <i>
        Item 2 <li.item> <b>Item</b> 3</li.item></i
      ></li.item
    ></li.item
  ></ul#items
>
```

- `for` loops:

```html
<ul#people>
  for name in names
  <li.person> name</li.person></ul#people
>
```

- `if else` statements:

```html
<div>if show-hint? <.hint> <b>Psst...</b> You can type `?` for help.</div>
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
