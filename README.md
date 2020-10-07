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
side content in a good ol' fashioned web application. Maybe [Vial]?

If you're feeling adventerous, or mad as a hatter, you can use the
standalone binary to turn templates into HTML files, or include the
zero-dependency Rust library in your (web/cli/?) application.

---

## Hello Hatter

Here are a few basic examples of what Hatter looks like:

```html
<!-- Hatter -->
<#main> Hi there!

<!-- Generated HTML -->
<div id="main">Hi there!</div>
```

```html
<span.big.bold> Welcome!

<span class="big bold">Welcome!</span>
```

```html
<.links> for link in nav-links
  <a href="{link.href}"> link.text

<div class="links">
  <a href="/link1">First Link</a>
  <a href="/link2">2nd Link</a>
  <a href="/link3">Final Link</a>
</div>
```

```html
<form GET="/search">
  <input@query:text placeholder="Search..." /> <input:submit />

<form method="GET" action="/search">
  <input name="query" type="text" placeholder="Search..." />
  <input type="submit" />
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
  - `bool, int, float, string, list, map, fn`
- Loop over `list` and `map`:
  - `<ul> for page in pages do <li id=page-{page.id}> page.name`
  - `for k, v in some-map do <td> k </> <td> v`
- if/else statements
  - `if logged_in? then <h2> Welcome back!`
- Error-checked assignment with `:=` and `=`:
  - `name := 'Bob'` will error if name **is** already set.
  - `name = 'Bob'` will error if name **isn't** already set.
- Call functions defined in Rust:
  - `<div.name> to-uppercase(name)`
- Define your own Hatter functions with strict arity and implicit
  return values:
  - `def greet(name) do print("Hey there, {name}!")`
  - `greet("Lydia")` prints `Hey there, Lydia!`
- Define your own Hatter operators:
  - `def ++(a, b) do concat(to-uppercase(a), ' ', to-uppercase(b))`
  - `"one" ++ "two"` returns `ONE TWO`
- Closures and function literals:
  - `adder := fn(x) fn(y) x + y` then `add1 := adder(1)`
  - `add1(200)` returns `201`
- Call functions with keyword arguments:
  - `def greet(title, name) do print("Hiya, {title}. {name}!")`
  - `greet(name: "Marley", title: "Dr")` prints `Hiya, Dr. Marley!`
- `do` keyword for one-line blocks:
  - `if 2 > 1 do print("Obviously")`
  - `for x in list do print(x)`
- `then` keyword for one-line `if` statements:
  - `if 2 > 1 then print("Yup!") else if 2 < 1 then print("Impossible.")`
- Hatter will add a `<!DOCTYPE>` and wrap everything in `<html>` if
  the first tag in your template is `<head>`.

## Future Features

- Define your own tags:
  - `def <item(item)> do <li.item data-id={item.id}> item.text`.
- Optional type checking for functions

## Getting Started

There are two ways to use Hatter:

### 1. `hatter` Executable

Hatter can be used as a regular command line program to turn `.hat`
files into HTML.

Just install it using `cargo`:

    cargo install hatter

Then point it at any `.hat` file:

```bash
$ cat test.hat
<b.test> "Testing 1 2 3 {2 + 2}"

$ hatter test.hat
<b class='test'>Testing 1 2 3 4 </b>
```

You can also install Hatter with a REPL:

    cargo install hatter --features repl

To launch it, start `hatter` with no arguments:

```bash
$ hatter
Hatter v0.0.1 REPL
>> 1 + 2
3
```

### 2. Crate

Hatter can (primarily) be used as a templating language from within
your Rust applications.

Simply add Hatter to `Cargo.toml`:

```toml
[dependencies]
hatter = "0.1"
```

Then create a `hatter::Env`, which represents the top-level Hatter
scope for your template, and set your variables:

```rust
use hatter::{Args, Env, Value};

let mut env = Env::new();
env.set("name", "Bobby Boucher");
env.set("age", 31);
env.render(r#"
<p> <b>Name:</> name
<p> <b>Age:</> age
"#)
```

You can also write functions in Rust and make them available to your
HTML templates:

```rust
fn quote(args: Args) -> Result<Value> {
  let file = std::fs::read_to_string("quotes.txt")?;
  let list_of_quotes: Vec<_> = file.split('\n').collect();
  let line = match args.need_number(0)? as usize {
    n if n > list_of_quotes.len() => 0,
    n => n,
  };

  Value::from(list_of_quotes[line]).ok()
}
fn main() {
    let mut env = Env::new();
    env.set("quote", quote);
    println!("{}", env.render("<div> quote(1)").unwrap());
}
```

For more infomation see the [API Documentation][api-docs].

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
[COPYING](COPYING) or <http://opensource.org/licenses/MIT> for details.

[Imba] is licensed under the [MIT License](https://github.com/imba/imba/blob/master/LICENSE).

[imba]: https://imba.io
[vial]: http://github.com/xvxx/vial
[api-docs]: https://docs.rs/hatter/