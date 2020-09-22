use hatter::{eval, parse, scan, Value};

macro_rules! eval {
    ($code:expr) => {
        scan(&$code)
            .and_then(|t| parse(&t))
            .and_then(|ast| eval(&ast))
            .unwrap()
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

#[test]
fn it_works() {
    assert_eq!(eval!("22"), num!(22));
    assert_eq!(eval!("2 + 2"), num!(4));
    assert_eq!(eval!("Hayay!"), string!("'Hayay!'"));
}

#[test]
fn test_list() {
    assert_eq!(eval!("[1,2,3]"), list![num!(1), num!(2), num!(3)]);
}
