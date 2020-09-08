# Hatter

<img src="./img/rhetoric.jpg" align="right" width="350" alt="The Mad Hatter discussing Hatter" />

_**Hatter** is a positively mad, HTML templating language._

Hatter is a cheap knock-off of [Imba's](https://imba.io) syntax,
without any of the fancy JavaScript stuff. Hatter runs server-side and
produces static HTML, like a less powerful 90s-era PHP. If you're
feeling adventerous, or mad as a hatter, you can use the standalone
binary to turn templates into HTML files, or include the
zero-dependency Rust library in your (web/cli/?) application.

Here's what it looks like:

```
<nav .webview-app=webview?>
  <a href="/signin"> sign in
  <a href="/signup"> sign up
  <ul>
  for link in nav-links
    <li.small-link> <a href=link.href> link.text

<div#main.markdown-body>
  if logged-in?
    <h1> Welcome back, <span.username> name </>!
  else
    <h1> Nice to, uh, see you. <span.aside> Have we met..?
```

## Features

- Tag shorthands (`<div#id>`, `<div.class1.class2>`, `<input@name:type>`)
- Conditional attributes
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

### Conditional attributes

```
<div#main.markdown-body.webview-app=webview?>
  <nav .logged-in=logged-in?>
    <p>
```

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

- [x] allow everything in top-level
- [ ] else ifs
- [x] \<style> tag
- [x] \<script> tag
- [ ] attribute=thing?
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
- [ ] def fn
