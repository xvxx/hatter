use hatter::{scan, Syntax};

macro_rules! scan {
    ($code:expr) => {
        scan($code).unwrap()
    };
}

macro_rules! scan_test {
    ($name:ident, $code:expr, $($kind:ident),+) => {
        #[test]
        fn $name() {
            let nodes = scan!("1 + 1");
            $(assert_eq!(Syntax::$kind, nodes.remove(0).kind);)+
        }
    };
}

scan_test!(one_plus_one, "1 + 1", Number, Op, Number);

#[test]
fn test_comment() {
    let nodes = scan!("1 + 1");
    println!("{:?}", scan!("1 + 1"));
    assert!(false);
}

/*
code: number, string, word, or operator
comments:
    - # to end of line
number:
    - 200
    - -10_150_203
    - 0b101
    - 0o123
    - 0xdeadbeef
    - 3.14
    - -102.123
string:
    "hey pal"
    'hey pal'
    `hey pal`
    """hey pal"""
    '''hey pal'''
    ```hey pal```
word:
    sup
    Yes
    some_thing
    can-include-dashes-{and brackets}
operator:
    no letters, no numbers
    ex: + - * / . = :=
    */
