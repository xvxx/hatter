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
}

#[test]
fn test_list() {
    assert_eval!("[1,2,3]", list![num!(1), num!(2), num!(3)]);
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
}

#[test]
fn test_return() {
    assert_render!(
        r#"
def test()
    print("OK")
test()
"#,
        "OK\n"
    );

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

#[test]
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
