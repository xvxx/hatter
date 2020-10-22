#![allow(unused)]

use hatter::{compile, eval, render, Value};

#[macro_use]
mod helpers;

#[test]
fn test_internal_functions() {
    assert_eval!(r#"concat("hi", "-", 23)"#, "hi-23");

    assert_eval!(r#"when(true, "yep")"#, "yep");
    assert_eval!(r#"when(false, "yep")"#, none!());

    assert_eval!(r#"type(false)"#, "bool");
    assert_eval!(r#"type(123)"#, "number");
    assert_eval!(r#"type('heyo')"#, "string");
}

#[test]
fn test_boolean_operators() {
    assert_eval!("true && true", true);
    assert_eval!("true && false", false);
    assert_eval!("false && true", false);
    assert_eval!("false && false", false);

    assert_eval!("true || true", true);
    assert_eval!("true || false", true);
    assert_eval!("false || true", true);
    assert_eval!("false || false", false);

    assert_eval!("1 == 1", true);
    assert_eval!("1 == 2", false);
    assert_eval!("'1' == 1", false);
    assert_eval!("'1' == '1'", true);
    assert_eval!("'1' != '1'", false);

    assert_eval!("!true", false);
    assert_eval!("!!true", true);

    assert_eval!("1 < 2", true);
    assert_eval!("2 < 1", false);
    assert_eval!("1 > 2", false);
    assert_eval!("1 >= 2", false);
    assert_eval!("2 > 1", true);
    assert_eval!("2 > 2", false);
    assert_eval!("2 >= 2", true);
}

#[test]
fn test_math() {
    assert_eval!("1 + 2", 3);
    assert_eval!("2 - 1", 1);
    assert_eval!("1 - 2", -1);
    assert_eval!("10 * 2", 20);
    assert_eval!("10 / 2", 5);
    assert_eval!("1 % 2", 1);
    assert_eval!("2 % 2", 0);
}

#[test]
fn test_string() {
    assert_eval!("'1' + '2'", "12");
    assert_eval!("to-uppercase('bob')", "BOB");
    assert_eval!("to-lowercase('BOB')", "bob");
    assert_eval!("contains?('Bob', 'B')", true);
    assert_eval!("contains?('Bob', 'Z')", false);
    assert_eval!(r#"replace("Mr Rogers", "Ro", "Dod")"#, "Mr Dodgers");
    assert_eval!(r#"replace("Mr Rogers", "Zo", "Dod")"#, "Mr Rogers");
}

#[test]
fn test_container() {
    assert_eval!("[1,2,3][1]", 2);
    assert_eval!("abc := { one: 1 }; abc.one", 1);
    assert_eval!("abc := { one: 1 }; abc.two", none!());
    assert_eval!("abc := { one: 1 }; abc['one']", 1);
    assert_eval!("abc := { one: 1 }; abc['two']", none!());
    assert_eval!("len([1,2,3])", 3);
    assert_eval!("len([])", 0);
    assert_eval!("len({})", 0);
    assert_eval!("len({one: 1, two: 2})", 2);
    assert_eval!("empty?({one: 1, two: 2})", false);
    assert_eval!("empty?({})", true);
    assert_eval!("empty?([])", true);
    assert_eval!("empty?([1,2,3])", false);
}

#[test]
fn test_list() {
    assert_eval!("a := 1..=3; a << 4; a[-1]", 4);
    assert_eval!("a := 1..=3; a << 40; a << 50; len(a)", 5);
    assert_eval!("a := 1..=3; push(a, 5); a[-1]", 5);
}

#[test]
fn test_range() {
    assert_eval!("1..5", vec![1, 2, 3, 4]);
    assert_eval!("1..=5", vec![1, 2, 3, 4, 5]);
}
