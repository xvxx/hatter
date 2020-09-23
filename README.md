# Hatter

<img src="./img/rhetoric.jpg" align="right" width="350" alt="The Mad Hatter discussing Hatter" />

> It is practically impossible to teach good programming to students
> that have had a prior exposure to _Hatter_: as potential programmers
> they are mentally mutilated beyond hope of regeneration.
>
> -– Edsger W. Dijkstra (allegedly)

Hatter is a small, whitespace sensitive scripting language with
HTML templating built in. Its HTML features and syntax are a cheap
knock off of [Imba], except Hatter produces raw, static HTML - no
JavaScript in sight.

Hatter can be used to generate static web sites or to render server
side content in an old fashioned web application.

It's like a less powerful, 90s-era PHP. But we're talking PHP/FI, none
of that Easy-Bake Oven PHP3 stuff that you could use to build actual
sites.

If you're feeling adventerous, or mad as a hatter, you can use the
standalone binary to turn templates into HTML files, or include the
zero-dependency Rust library in your (web/cli/?) application.

---

## Hello Hatter

Here's what it looks like:

```html
<nav .webview-app=webview?>
  <a href="/signin"> sign in
  <a href="/signup"> sign up
  <ul> for link in nav-links
    <li.small-link> <a href=link.href> link.text
  <form GET="/search">
    <input@query:text placeholder="Search..." /> <input:submit/>

<#main.markdown-body>
  if logged-in?
    <h1> Welcome back, <span.username> name </>!
  else
    <h1> Nice to, uh, see you. <span.aside> Have we met..?
```

Which, if we're logged in as `The Mad Hatter` and `webview?` is
`false`, will generate this:

```html
<nav>
  <a href='/signin'> sign in </a>
  <a href='/signup'> sign up </a>
  <ul>
    <li class='small-link'> <a href='/hats'> Hats </a> </li>
    <li class='small-link'> <a href='/cards'> Cards </a> </li>
    <li class='small-link'> <a href='/tea'> Tea </a> </li>
  </ul>
  <form method='GET' action='/search'>
    <input name='query' type='text' placeholder='Search...' />
    <input type='submit' />
  </form>
</nav>

<div id='main' class='markdown-body'>
  <h1> Welcome back, <span class='username'> The Mad Hatter </span> !
</div>
```

## Features

- Auto-closing HTML tags and code blocks based on indentation.
- Shorthand for `id`, `class`, `type`, and `name` attributes:
  - `<div#id>`
  - `<div.class1.class2>`
  - `<input@form-field-name>`
  - `<input:text>`
- Basic types:
  - `bool`, `int,` `float`, `string`, `list`, `map`, `fn()`
- Loop over `list` and `map`:
  - `<ul> for page in pages do <li id=page-{page.id}> page.name`
  - `for k, v in some-map do <td> k </> <td> v`
- if/else statements
  - `if logged_in? then <h2> Welcome back!`
- Error-checked assignment with `:=` and `=`:
  - `name := 'Bob'`  will error if name **is** already set.
  - `name = 'Bob'`  will error if name **isn't** already set.
- Dynamic values for regular attributes:
  - `<div page-num={page.id}>`
- Conditionally set attributes or enable shorthand:
  - `<div .logged-in=logged-in?>`
  - `<div data-map=is-map?>`
- String interpolation:
  - `<span.greeting> "Hey there {name}. 2 + 2 is {2 + 2}"`
- Shorthand interpolation:
  - `<span #page-{page.id} .is-{page.type}> page.title`
- Implicit divs:
  - `<#main>` becomes `<div id='main'>`
- Implicit closing tags:
  - `<i>delicious</>` becomes `<i>delicious</i>`
- Call functions defined in Rust:
  - `<div.name> to-uppercase(name)`
- Define your own Hatter functions:
  - `def greet(name) do print("Hey there, {name}!")`
  - `greet("Lydia")` prints `Hey there, Lydia!`
- Easy inline JavaScript:
  - `<li> <a onclick=(alert("Oink!"))> 🐷`
- Hatter will add a `<!DOCTYPE>` and wrap everything in `<html>` if
  the first tag in your template is `<head>`.

## Future Features

- Define your own tags:
  - `def <item(item)> do <li.item data-id={item.id}> item.text`.
- Define your own operators:
  - `def !!(a, b) do return concat(to-uppercase(a), ' ', to-uppercase(b))`
- Arity checking for functions.
- Optional type checking for functions(?)

## TODO

### next

- [ ] big
  - [ ] pratt parser
  - [ ] int vs float
  - [ ] HTMLized error page

### future

- [ ] def <tag attr=Type>
- [ ] convert lexer to bytes ala https://github.com/nathanwhit/minimal-yaml
- [ ] show error location when compiling
- [ ] repl: tab completion

### docs

- [ ] design
- [ ] layout
- [ ] generator
- [ ] content

### project

- [ ] bomb-ass test suite
- [ ] stdlib
- [ ] VSCode Extension
- [ ] VSCode + luacheck-style LSP
- [ ] luacheck-style tool

## License

Hatter is licensed under the MIT License. Please see[COPYING](COPYING)
or http://opensource.org/licenses/MIT for details.

[imba]: https://imba.io