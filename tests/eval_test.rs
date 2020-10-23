#![allow(unused)]

use hatter::{compile, eval, render, Value};

#[macro_use]
mod helpers;

#[test]
fn it_works() {
    assert_eval!("22", num!(22));
    assert_eval!("2 + 2", num!(4));
    assert_eval!("'Hayay!'", string!("Hayay!"));
    assert_eval!("(((0)))", num!(0));
}

#[test]
fn test_string() {
    assert_render!(r#"<b> "Hi there""#, "<b>Hi there</b>\n");
    assert_render!(r#"<b> """#, "<b></b>\n");
}

#[test]
fn test_list() {
    assert_eval!("[1,2,3]", list![num!(1), num!(2), num!(3)]);
    assert_eval!(
        "['Peter', 'Paul', 'Bob']",
        list![string!("Peter"), string!("Paul"), string!("Bob"),]
    );
}

#[test]
fn test_map() {
    assert_eval!(
        "{ name: 'Bilbo', age: 111 }",
        map!("name" => string!("Bilbo"), "age" => num!(111))
    );

    assert_render!(
        "map := { name: 'Bilbo', age: 111 }
for k, v in map
    <span> <b> k </> v",
        "<span><b>name</b>\nBilbo</span>\n<span><b>age</b>\n111</span>\n"
    );
}

#[test]
fn test_attributes() {
    assert_render!(r#"<div data-id='123'/>"#, "<div data-id='123' />\n");
    assert_render!(r#"<div data-id=123/>"#, "<div data-id=123 />\n");
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
    assert_eval!(
        r#"
list := [1,2,3]
list[-1]
"#,
        num!(3)
    );
    assert_eval!(
        r#"
list := [1,2,3]
list[-2]
"#,
        num!(2)
    );
    assert_eval!(
        r#"
list := [1,2,3]
list[-20]
"#,
        Value::None
    );
}

#[test]
fn test_index_assign() {
    assert_eval!("a := 1..=5; a[1] = 100; a[1]", 100);
    assert_eval!("a := { one: 1, two: 2}; a['one'] = 100; a.one", 100);
    assert_eval!("a := { one: 1, two: 2}; a.one = 100; a.one", 100);
    assert_eval!("a := {m: {}}; a.m['one'] = 100; a.m.one", 100);
    assert_eval!("a := {m: {}}; a.m.one = 100; a.m.one", 100);
}

#[test]
fn test_if() {
    assert_render!(
        r#"
if true
    puts("yay")
"#,
        "yay\n"
    );

    assert_render!(r#"if true do print("yay")"#, "yay");
    assert_render!(r#"if false do print("yay") else do print("nay")"#, "nay");
    assert_render!(
        r#"if false do print("yay") else if 1 > 0 do print("nay") else do print("cray")"#,
        "nay"
    );

    assert_render!(r#"if true then print("yay")"#, "yay");
    assert_render!(r#"if false then print("yay") else print("nay")"#, "nay");
    assert_render!(
        r#"if false then print("yay") else if 1 > 0 then print("nay") else print("cray")"#,
        "nay"
    );
}

#[test]
fn test_and_n_or() {
    assert_eval!("true && true", boo!(true));
    assert_eval!("true && false", boo!(false));
    assert_eval!("false && true", boo!(false));
    assert_eval!("false && false", boo!(false));
    assert_eval!("true || true", boo!(true));
    assert_eval!("true || false", boo!(true));
    assert_eval!("false || true", boo!(true));
    assert_eval!("false || false", boo!(false));
    assert_eval!("!true", boo!(false));
    assert_eval!("!false", boo!(true));
    assert_eval!("!!true", boo!(true));

    assert_render!(
        r#"
def test()
    print("Tested!")
    return false
test() && test()
"#,
        "Tested!false"
    );
}

#[test]
fn test_for() {
    assert_render!(
        r#"
for x in [10,20,30]
    puts(x)
"#,
        "10\n20\n30\n"
    );

    assert_render!(
        r#"
for x in [10,20,30,40]
    if x > 20
        break
    puts(x)
"#,
        "10\n20\n"
    );

    assert_render!(
        r#"
for x in [10,20,30,40]
    if x == 20
        continue
    puts(x)
"#,
        "10\n30\n40\n"
    );

    assert_render!(
        r#"
for x in [10,20,30,40]
    if x == 20 do continue
    puts(x)
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
        "OK"
    );

    assert_render!(
        r#"
def test()
    print("OK")
test()
"#,
        "OK"
    );
}

#[test]
fn test_call_with_keywords() {
    assert_render!(
        r#"
def greet(title, name) do print("Hiya, {title}. {name}!")
greet(name: 'Bob', title: 'Dr')
"#,
        "Hiya, Dr. Bob!"
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
        "BYE"
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
fn test_form_shortcuts() {
    assert_render!(
        "<form POST='/update'> <input:submit/>",
        "<form method='POST' action='/update'><input type='submit' />\n</form>\n"
    );

    assert_render!(
        "<form GET='/search'> <input@query:text/> <input:submit/>",
        "<form method='GET' action='/search'><input name='query' type='text' />\n<input type='submit' />\n</form>\n"
    );
}

#[test]
fn test_math() {
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

    assert_eval!("3.14", num!(3.14));
    assert_eval!("-102.123", num!(-102.123));

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
        "Hiya, Mrs. Robinson!"
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
            puts(word)
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

    assert_render!(concat!("i := 0", ifelse!()), "Cero");
    assert_render!(concat!("i := 10", ifelse!()), "Positive");
    assert_render!(concat!("i := -10", ifelse!()), "Negative");
    assert_render!(concat!("i := 100_000_001", ifelse!()), "Way TOO Positive!");

    assert_render!(
        r#"
for v in [100, 200, 300]
    puts(v) #=> 100 then 200 then 300
"#,
        "100\n200\n300\n"
    );

    assert_render!(
        r#"
for i, v in [100, 200, 300]
    puts(i) #=> 0 then 1 then 2
    "#,
        "0\n1\n2\n"
    );

    assert_render!(
        r#"
for k, v in { first: 1, second: 2 }
    puts("{k} is {v}") #=> `first is 1` then `second is 2`
    "#,
        "first is 1\nsecond is 2\n"
    );

    assert_render!(
        r#"
x := 0
while true
    x += 1
    puts("O'DOYLE RULES!")
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
    puts(v)
    if v > 300
        break
        "#,
        "100\n200\n300\n400\n"
    );
}

#[test]
fn readme_features() {
    // Auto-closing HTML tags and code blocks based on indentation
    assert_render!("<h1> Welcome, <i> Rob", "<h1>Welcome,<i>Rob</i>\n</h1>\n");
    assert_render!("<b> Heya", "<b>Heya</b>\n");
    assert_render!("<b> Heya <i> there", "<b>Heya<i>there</i>\n</b>\n");

    // Shorthand for `id`, `class`, `type`, and `name` attributes
    assert_render!("<div#id>", "<div id='id'></div>\n");
    assert_render!("<div.class1.class2>", "<div class='class1 class2'></div>\n");
    assert_render!(
        "<input@form-field-name>",
        "<input name='form-field-name'></input>\n"
    );
    assert_render!("<input:text>", "<input type='text'></input>\n");

    // Dynamic values for regular attributes
    assert_render!(
        "page := { id: 33 } <div page-num={page.id}>",
        "<div page-num=33></div>\n"
    );

    // Conditionally set attributes or enable shorthand
    assert_render!(
        "logged-in? := true
<div .logged-in=logged-in?>",
        "<div class='logged-in'></div>\n"
    );
    assert_render!(
        "is-map? := true
<div data-map=is-map?>",
        "<div data-map></div>\n"
    );

    // String interpolation
    assert_render!(
        r#"name := 'Bob'
<span.greeting> "Hey there {name}. 2 + 2 is {2 + 2}""#,
        "<span class='greeting'>Hey there Bob. 2 + 2 is 4</span>\n"
    );

    // Shorthand interpolation
    assert_render!(
        r#"page := { id: 22, type: 'index', title: 'Index' }
<span #page-{page.id} .is-{page.type}> page.title"#,
        "<span id='page-22' class='is-index'>Index</span>\n"
    );

    // Implicit divs
    assert_render!(r#"<#main>"#, "<div id='main'></div>\n");

    // Implicit closing tags
    assert_render!(r#"<i>delicious</>"#, "<i>delicious</i>\n");

    // Easy inline JavaScript
    assert_render!(
        r#"<li> <a onclick=(alert("Oink!"))> "🐷""#,
        r#"<li><a onclick='(function(e){ alert("Oink!") })(event);' href='#'>🐷</a>
</li>
"#
    );

    // Basic types:
    // bool, int, float, string, list, map, fn
    assert_eval!("true", boo!(true));
    assert_eval!("false", boo!(false));
    assert_eval!("101", num!(101));
    assert_eval!("3.14", num!(3.14));
    assert_eval!("-200.12", num!(-200.12));
    assert_eval!(r#""hey there""#, string!("hey there"));
    assert_eval!(r#"'hi friends'"#, string!("hi friends"));
    assert_eval!(r#"[1,2,3]"#, list![num!(1), num!(2), num!(3)]);
    assert_eval!(r#"['Bob', 'Rob']"#, list![string!("Bob"), string!("Rob")]);
    assert_eval!(
        r#"{one: 1, two: 2}"#,
        map!(
            "one" => num!(1),
            "two" => num!(2),
        )
    );
    assert_eval!(
        r#"{name: "Ringo", job: "Drums"}"#,
        map!(
            "name" => string!("Ringo"),
            "job" =>  string!("Drums"),
        )
    );

    // Loop over `list` and `map`
    assert_render!(
        r#"pages := [{id: 1, name: "Page 1"}, {id: 2, name: "2nd"}]
<ul> for page in pages do <li id=page-{page.id}> page.name"#,
        "<ul><li id='page-1'>Page 1</li>\n<li id='page-2'>2nd</li>\n</ul>\n"
    );
    assert_render!(
        r#"some-map := { one: 1, two: 2 }
for k, v in some-map do <tr> <td> k </> <td> v"#,
        "<tr><td>one</td>\n<td>1</td>\n</tr>\n<tr><td>two</td>\n<td>2</td>\n</tr>\n"
    );

    // if/else statement
    assert_render!(
        r#"logged_in? := false
if logged_in? then <h2> Welcome back!"#,
        r#""#
    );
    assert_render!(
        "logged_in? := true
if logged_in? then <h2> Welcome back!",
        "<h2>Welcome back!</h2>\n"
    );

    // Error-checked assignment with `:=` and `=`
    assert_error!(
        "
name := 'Bob'
name := 'Rob'"
    );
    assert_error!("name = 'Bob'");

    // Call functions defined in Rust
    assert_render!(
        r#"name := "tony"
<div.name> to-uppercase(name)"#,
        "<div class='name'>TONY</div>\n"
    );

    // Define your own Hatter functions with strict arity and implici
    //   return values:
    assert_render!(
        r#"def greet(name) do puts("Hey there, {name}!")
greet("Lydia")"#,
        "Hey there, Lydia!\n"
    );

    // Define your own Hatter operators
    assert_render!(
        r#"
def ++(a, b) do concat(to-uppercase(a), ' ', to-uppercase(b))
puts("cat" ++ "dog")
"#,
        "CAT DOG\n"
    );

    // Closures and function literals
    assert_eval!(
        r#"
adder := fn(x) fn(y) x + y
add1 := adder(1)
add1(200)
"#,
        num!(201)
    );

    // Call functions with keyword arguments
    assert_render!(
        r#"
def greet(title, name) do puts("Hiya, {title}. {name}!")
greet(name: "Marley", title: "Dr")
"#,
        "Hiya, Dr. Marley!\n"
    );

    // `do` keyword for one-line blocks
    assert_render!(r#"if 2 > 1 do puts("Obviously")"#, "Obviously\n");
    assert_render!(r#"for x in [1,2,3] do puts(x)"#, "1\n2\n3\n");

    // `then` keyword for one-line `if` statements
    assert_render!(
        r#"if 2 > 1 then puts("Yup!") else if 2 < 1 then puts("Impossible.")"#,
        "Yup!\n"
    );

    // Hatter will add a `<!DOCTYPE>` and wrap everything in `<html>` if
    // the first tag in your template is `<head>`.
    assert_render!(
        "<head>
    <title> Kewl
<body>
    <h1> Heya
    <p> Hey ya!",
        "<!DOCTYPE html>
<html><head><title>Kewl</title>
</head>
<body><h1>Heya</h1>
<p>Hey ya!</p>
</body>

</html>
"
    );
}
