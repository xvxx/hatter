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

#[test]
fn it_works() {
    assert_eq!(num!(4), eval!("2 + 2"));
}
