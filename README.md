# Hatter

<img src="./img/rhetoric.jpg" align="right" width="350" alt="The Mad Hatter discussing Hatter" />

> It is practically impossible to teach good programming to students
> that have had a prior exposure to _Hatter_: as potential programmers
> they are mentally mutilated beyond hope of regeneration.
>
> -– Edsger W. Dijkstra (allegedly)

Hatter is a small, whitespace sensitive templating language with HTML
support built right in. Its HTML features and syntax are a cheap
knock off of [Imba], except Hatter produces raw, static HTML - no
JavaScript in sight.

Hatter can be used to generate static web sites or to render server
side content in a good ol' fashioned web application. Maybe with
[Vial]?

If you're feeling adventerous, or mad as a hatter, you can use the
standalone binary to turn templates into HTML files, or include the
zero-dependency Rust library in your (web/cli/?) application.

---

## Hello Hatter

Here are a few basic examples of what Hatter looks like:

```html
<!-- Hatter -->
<#main> Hi there!

<!-- Output -->
<div id='main'>Hi there!</div>

<!-- Hatter -->
<span.big.bold> Welcome!

<!-- Output -->
<span class='big bold'>Welcome!</span>

<!-- Hatter -->
<.links> for link in nav-links
  <a href={link.href}> link.text

<!-- Output -->
<div class='links'>
  <a href='/link1'>First Link</a>
  <a href='/link2'>2nd Link</a>
  <a href='/link3'>Final Link</a>
</div>

<!-- Hatter -->
<form GET="/search">
  <input@query:text placeholder="Search..." /> <input:submit/>
  
<!-- Output -->
<form method='GET' action='/search'>
  <input name='query' type='text' placeholder='Search...' />
  <input type='submit' />
</form>
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
- [ ] show error location in source text on runtime errors

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
[vial]: http://github.com/xvxx/vial