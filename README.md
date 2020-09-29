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
side content in a good ol' fashioned web application.

If you're feeling adventerous, or mad as a hatter, you can use the
standalone binary to turn templates into HTML files, or include the
zero-dependency Rust library in your (web/cli/?) application.

---

## Hello Hatter

Here's what it looks like:

```html
<ul#list>
  for i, person in people
    <li.person.first={i == 0}> person
```

Which turns into:

```html
<ul id='list'>
  <li class='person first'>John</li>
  <li class='person'>Paul</li>
  <li class='person'>George</li>
  <li class='person'>Ringo</li>
</ul>
```

Or, a beefier example:

```html
<nav .webview-app=webview?>
  if not logged-in?
    <a href="/signin"> sign in </> | <a href="/signup"> sign up
  <ul> for link in nav-links
    <li.small-link> <a href={link.href}> link.text
  <form GET="/search">
    <input@query:text placeholder="Search..." /> <input:submit/>

<#main.markdown-body>
  if logged-in?
    <h1> "Welcome back, {<span.username> name}!"
  else
    <h1> Nice to, uh, see you. <span.aside> Have we met..?
```

Which, if we're logged in as `The Mad Hatter` and `webview?` is
`true`, will turn into:

```html
<nav class='webview-app'>
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

- Auto-closing HTML tags and code blocks based on indentation:
  - `<h1> Welcome, <i> Rob` becomes `<h1> Welcome, <i> Rob </i></h1>`
- Shorthand for `id`, `class`, `type`, and `name` attributes:
  - `<div#id>`
  - `<div.class1.class2>`
  - `<input@form-field-name>`
  - `<input:text>`
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
- Easy inline JavaScript:
  - `<li> <a onclick=(alert("Oink!"))> "🐷"`
- Basic types:
  - `bool`, `int,` `float`, `string`, `list`, `map`, `fn()`
- Loop over `list` and `map`:
  - `<ul> for page in pages do <li id=page-{page.id}> page.name`
  - `for k, v in some-map do <td> k </> <td> v`
- if/else statements
  - `if logged_in? then <h2> Welcome back!`
- Error-checked assignment with `:=` and `=`:
  - `name := 'Bob'`  will error if name **is** already set.
  - `name = 'Bob'`   will error if name **isn't** already set.
- Call functions defined in Rust:
  - `<div.name> to-uppercase(name)`
- Define your own Hatter functions with strict arity and implicit
  return values:
  - `def greet(name) do print("Hey there, {name}!")`
  - `greet("Lydia")` prints `Hey there, Lydia!`
- Define your own Hatter operators:
  - `def !!(a, b) do concat(to-uppercase(a), ' ', to-uppercase(b))`
  - `"one" !! "two"` returns `ONE TWO`
- Closures and function literals:
  - `adder := fn(x) fn(y) x + y` then `add1 := adder(1)`
  - `add1(200)` returns `201`
- `do` keyword for one-line blocks:
  - `if 2 > 1 \n\tprint("Obviously")` OR `if 2 > 1 do print("Obviously")`
  - `for x in list\n\tprint(x)` OR `for x in list do print(x)`
- Hatter will add a `<!DOCTYPE>` and wrap everything in `<html>` if
  the first tag in your template is `<head>`.

## Future Features

- Define your own tags:
  - `def <item(item)> do <li.item data-id={item.id}> item.text`.
- Optional type checking for functions

## TODO

- [ ] HTMLized error page
- [ ] def <tag attr=Type>
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

Hatter is licensed under the MIT License. Please see
[COPYING](COPYING) or http://opensource.org/licenses/MIT for details.

[Imba] is licensed under the MIT License.

[imba]: https://imba.io