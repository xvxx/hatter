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
