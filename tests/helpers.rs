#![allow(unused_macros)]

macro_rules! eval {
    ($code:expr) => {
        eval(&$code).unwrap()
    };
}

macro_rules! render {
    ($code:expr) => {
        render(&$code).unwrap()
    };
}

macro_rules! none {
    () => {
        Value::None
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
        match eval(&$code) {
            Ok(v) => assert!(false, "Expected Error, got {:?}", v),
            Err(..) => assert!(true),
        }
    };
}

macro_rules! assert_eval {
    ($code:expr, $val:expr) => {
        assert_eq!(eval!($code), $val.into());
    };
}

macro_rules! assert_render {
    ($code:expr, $val:expr) => {
        assert_eq!(render!($code), $val);
    };
}
