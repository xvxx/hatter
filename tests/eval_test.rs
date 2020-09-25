#![allow(unused)]

use hatter::{compile, eval, render, Value};

macro_rules! eval {
    ($code:expr) => {
        compile(&$code).and_then(|ast| eval(&ast)).unwrap()
    };
}

macro_rules! render {
    ($code:expr) => {
        compile(&$code).and_then(|ast| render(&ast)).unwrap()
    };
}

macro_rules! boo {
    ($boo:expr) => {
        Value::Bool($boo)
    };
}
macro_rules! num {
    ($num:expr) => {
        Value::from($num)
    };
}

macro_rules! string {
    ($s:expr) => {
        Value::from($s)
    };
}

macro_rules! list {
    ($($args:expr),+) => {
        Value::from(vec![$($args),+])
    };
    ($($args:expr,)+) => {
        Value::from(vec![$($args),+])
    };
}

macro_rules! map {
    ($($key:expr => $val:expr),+) => {{
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        $(
            map.insert($key.to_string(), $val);
        )+
        Value::from(map)
    }};
    ($($key:expr => $val:expr,)+) => {
        map!($($key => $val),+)
    }
}

macro_rules! assert_error {
    ($code:expr) => {
        match compile(&$code).and_then(|ast| eval(&ast)) {
            Ok(v) => assert!(false, "Expected Error, got {:?}", v),
            Err(..) => assert!(true),
        }
    };
}

macro_rules! assert_eval {
    ($code:expr, $val:expr) => {
        assert_eq!(eval!($code), $val);
    };
}

macro_rules! assert_render {
    ($code:expr, $val:expr) => {
        assert_eq!(render!($code), $val);
    };
}

#[test]
fn it_works() {
    assert_eval!("22", num!(22));
    assert_eval!("2 + 2", num!(4));
    assert_eval!("'Hayay!'", string!("Hayay!"));
    assert_eval!("(((0)))", num!(0));
}

#[test]
fn test_string() {
    assert_render!(r#"<b> "Hi there""#, "<b>Hi there\n</b>\n");
    assert_render!(r#"<b> """#, "<b>\n</b>\n");
}

#[test]
fn test_list() {
    assert_eval!("[1,2,3]", list![num!(1), num!(2), num!(3)]);
}

#[test]
fn test_index() {
    assert_eval!(
        r#"
list := [1,2,3]
list[1]
"#,
        num!(2)
    );
    assert_eval!(
        r#"
list := [1,2,3]
list[2]
"#,
        num!(3)
    );
    assert_eval!(
        r#"
list := [1,2,3]
list[3]
"#,
        Value::None
    );
}

#[test]
fn test_if() {
    assert_render!(
        r#"
if true
    print("yay")
"#,
        "yay\n"
    );

    assert_render!(r#"if true do print("yay")"#, "yay\n");
    assert_render!(r#"if false do print("yay") else do print("nay")"#, "nay\n");
    assert_render!(
        r#"if false do print("yay") else if 1 > 0 do print("nay") else do print("cray")"#,
        "nay\n"
    );
}

#[test]
fn test_and_n_or() {
    // assert_eval!("not true", boo!(false));
    // assert_eval!("not false", boo!(true));
    // assert_eval!("not not true", boo!(true));
    assert_eval!("true && true", boo!(true));
    assert_eval!("true && false", boo!(false));
    assert_eval!("false && true", boo!(false));
    assert_eval!("false && false", boo!(false));
    assert_eval!("true || true", boo!(true));
    assert_eval!("true || false", boo!(true));
    assert_eval!("false || true", boo!(true));
    assert_eval!("false || false", boo!(false));

    assert_render!(
        r#"
def test()
    print("Tested!")
    return false
test() && test()
"#,
        "Tested!\n"
    );
}

#[test]
fn test_for() {
    assert_render!(
        r#"
for x in [10,20,30]
    print(x)
"#,
        "10\n20\n30\n"
    );

    assert_render!(
        r#"
for x in [10,20,30,40]
    if x > 20
        break
    print(x)
"#,
        "10\n20\n"
    );

    assert_render!(
        r#"
for x in [10,20,30,40]
    if x == 20
        continue
    print(x)
"#,
        "10\n30\n40\n"
    );

    assert_render!(
        r#"
for x in [10,20,30,40]
    if x == 20 do continue
    print(x)
"#,
        "10\n30\n40\n"
    );
}

#[test]
fn test_def() {
    assert_render!(
        r#"
def test() do print("OK")
test()
"#,
        "OK\n"
    );

    assert_render!(
        r#"
def test()
    print("OK")
test()
"#,
        "OK\n"
    );
}

#[test]
fn test_return() {
    assert_eval!(
        r#"
def test()
    return 1 + 1
test()
"#,
        num!(2)
    );

    assert_render!(
        r#"
def test()
    if true
        print("BYE")
        return
    print("OK!")
test()
"#,
        "BYE\n"
    );
}

// #[test]
fn test_nesting() {
    assert_render!(
        r#"
<ul> for x in [1,2,3]
    <li> x
<span> The End
"#,
        "<ul><li>1\n</li>\n<li>2\n</li>\n<li>3\n</li>\n</ul>
<span>The End\n</span>
"
    );

    assert_render!(
        r#"
<#main> <ul> for x in [1,2,3]
    <li> x
<#footer>
    <p> Thanks
"#,
        "<div id='main'><ul><li>1\n</li>\n<li>2\n</li>\n<li>3\n</li>\n</ul>\n</div>
<div id='footer'><p>Thanks\n</p>\n</div>
"
    );
}

#[test]
fn math() {
    assert_eval!("1 + 1", num!(2));
    assert_eval!("20 * 10", num!(200));
    assert_eval!("2.2 + 3.3", num!(5.5));
}

////
// Hatter in a Hurry

#[test]
fn hatter_in_a_hurry() {
    assert_eval!("false", boo!(false));
    assert_eval!("true", boo!(true));

    assert_eval!("200", num!(200.0));
    assert_eval!("-10_150_203", num!(-10_150_203));
    assert_eval!("0b101", num!(0b101));
    assert_eval!("0o123", num!(0o123));
    assert_eval!("0xabba", num!(0xabba));

    // assert_eval!("3.14", num!(3.14));
    // assert_eval!("-102.123", num!(-102.123));

    assert_eval!(r#""Heya pal!""#, string!("Heya pal!"));
    assert_eval!(r#"'Also, hi.'"#, string!("Also, hi."));
    assert_eval!(r#"`Also, hello.`"#, string!("Also, hello."));
    assert_eval!(
        r#""""
    Triple version of ', ", and `
    works for multi-line strings.
""""#,
        string!(
            r#"
    Triple version of ', ", and `
    works for multi-line strings.
"#
        )
    );
    assert_eval!(
        r#""Double quoted strings are interpolated: {2 + 2}""#,
        string!(r#"Double quoted strings are interpolated: 4"#)
    );

    assert_eval!(r#"[1, 2, 3]"#, list!(num!(1), num!(2), num!(3)));
    assert_eval!(
        r#"["John", "Paul", "George", "Ringo"]"#,
        list!(
            string!("John"),
            string!("Paul"),
            string!("George"),
            string!("Ringo"),
        )
    );
    assert_eval!(
        r#"[true, 2, "Paul"]"#,
        list!(boo!(true), num!(2), string!("Paul"))
    );

    assert_eval!(
        r#"{ one: "one", two: "two" }"#,
        map!(
            "one" => string!("one"),
            "two" => string!("two"),
        )
    );
    assert_eval!(
        r#"{ 0: "oh", 1: "also one" }"#,
        map!(
            "0" => string!("oh"),
            "1" => string!("also one"),
        )
    );

    assert_eval!("z := fn(x) return x + 1\nz(2)", num!(3));

    assert_eval!("num := 123\nnum = 456\nnum", num!(456));
    assert_error!("num := 123\nnum = '456'");
    assert_error!("num := 123\nnum := 456");
    assert_error!("rand = 'random'");

    assert_render!(
        r#"
def greet(title, name)
    print("Hiya, {title}. {name}!")
greet('Mrs', 'Robinson')
"#,
        "Hiya, Mrs. Robinson!\n"
    );

    assert_error!(
        r#"
def greet(title, name)
    print("Hiya, {title}. {name}!")
greet('Mrs', 'Robinson', 'Crusoe')
"#
    );

    assert_render!(
        r#"
def mod(num, by, msg)
    if (num % by) == 0
        return msg
    else
        return ""

def fizz-buzz
    for i in 1..=30
        word := mod(i, 3, 'Fizz') + mod(i, 5, 'Buzz')
        if len(word) > 0
            print(word)
fizz-buzz()
"#,
        "Fizz\nBuzz\nFizz\nFizz\nBuzz\nFizz\nFizzBuzz\nFizz\nBuzz\nFizz\nFizz\nBuzz\nFizz\nFizzBuzz\n"
);

    macro_rules! ifelse {
        () => {
            r#"
if i == 0
    print("Cero")
else if i > 100_000_000
    print("Way TOO Positive!")
else if i > 0
    print("Positive")
else if i < 0
    print("Negative")
"#;
        };
    }

    assert_render!(concat!("i := 0", ifelse!()), "Cero\n");
    assert_render!(concat!("i := 10", ifelse!()), "Positive\n");
    assert_render!(concat!("i := -10", ifelse!()), "Negative\n");
    assert_render!(
        concat!("i := 100_000_001", ifelse!()),
        "Way TOO Positive!\n"
    );

    // assert_eval!("not true", boo!(false));
    // assert_eval!("not false", boo!(true));
    // assert_eval!("not not true", boo!(true));
    // assert_eval!("true and true", boo!(true));
    // assert_eval!("true and false", boo!(false));
    // assert_eval!("false and true", boo!(false));
    // assert_eval!("false and false", boo!(false));
    // assert_eval!("true or true", boo!(true));
    // assert_eval!("true or false", boo!(true));
    // assert_eval!("false or true", boo!(true));
    // assert_eval!("false or false", boo!(false));

    assert_render!(
        r#"
for v in [100, 200, 300]
    print(v) #=> 100 then 200 then 300
"#,
        "100\n200\n300\n"
    );

    assert_render!(
        r#"
for i, v in [100, 200, 300]
    print(i) #=> 0 then 1 then 2
    "#,
        "0\n1\n2\n"
    );

    assert_render!(
        r#"
for k, v in { first: 1, second: 2 }
    print("{k} is {v}") #=> `first is 1` then `second is 2`
    "#,
        "first is 1\nsecond is 2\n"
    );

    assert_render!(
        r#"
x := 0
while true
    x += 1
    print("O'DOYLE RULES!")
    if x > 1
        break
    "#,
        "O'DOYLE RULES!\nO'DOYLE RULES!\n"
    );

    assert_render!(
        r#"
def hundreds(list)
    new-list := []
    for n in list
        new-list << (n * 100)
    return new-list

for v in hundreds(1..5)
    print(v)
    if v > 300
        break
        "#,
        "100\n200\n300\n400\n"
    );
}

#[test]
fn readme_features() {
    // Auto-closing HTML tags and code blocks based on indentation.
    assert_render!("<b> Heya", "<b>Heya\n</b>\n");
    assert_render!("<b> Heya <i> there", "<b>Heya\n<i>there\n</i>\n</b>\n");
    /*
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
      */
}
