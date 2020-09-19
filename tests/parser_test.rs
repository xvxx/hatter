use hatter::{parse, scan, Stmt};

fn print_nodes(i: usize, nodes: &[Stmt]) {
    println!("Computed nodes:");
    for (x, stmt) in nodes.iter().enumerate() {
        let (bold, clear) = if x == i {
            ("\x1b[1;91m", "\x1b[0m")
        } else {
            ("", "")
        };
        println!("  {}{:?}{}", bold, stmt, clear);
    }
    println!("        left=want, right=got");
}

macro_rules! parse_test {
    ($name:ident, $code:expr, $($stmt:expr,)+) => {
        #[test]
        fn $name() {
            let nodes = scan($code).and_then(|t| parse(&t)).unwrap();
            let mut i = 0;
            $(
                let node = nodes.get(i).unwrap();
                if node == &$stmt {
                    print_nodes(i, &nodes);
                }
                assert_eq!(&$stmt, node);
                { i += 1; }
            )+
            if i != nodes.len() {
                print_nodes(i, &nodes);
            }
            assert_eq!(i, nodes.len());
        }
    };
    ($name:ident, $code:expr, $($kind:expr),+) => {
        parse_test!($name, $code, $($kind,)+);
    }
}

////
// boolean

parse_test!(true_bool, "true", Stmt::Bool(true));
parse_test!(false_bool, "false", Stmt::Bool(false));

////
// number

parse_test!(positive_number, "100", Stmt::Number(100));
parse_test!(negative_number, "-400", Stmt::Number(-400));
parse_test!(underscore_number, "1_422_200", Stmt::Number(1_422_200));
parse_test!(
    negative_underscore_number,
    "-1_422_200",
    Stmt::Number(-1_422_200)
);
parse_test!(binary_number, "0b101", Stmt::Number(5));
parse_test!(octal_number, "0o755", Stmt::Number(493));
parse_test!(hex_number, "0xdeadbeef", Stmt::Number(3735928559));
parse_test!(float_number, "3.14", Stmt::Number(3));
parse_test!(negative_float_number, "-1230.3552", Stmt::Number(-1230));

////
// string

////
// if

////
// for

////
// while

////
// def

////
// assign

parse_test!(
    assign,
    "a := 123",
    Stmt::Assign("a".into(), Box::new(Stmt::Number(123)), false)
);

////
// reassign

parse_test!(
    reassign,
    "a = 123",
    Stmt::Assign("a".into(), Box::new(Stmt::Number(123)), true)
);

////
// call

parse_test!(
    basic_call,
    "word(true)",
    Stmt::Call("word".into(), vec![Stmt::Bool(true)])
);

parse_test!(empty_call, "add()", Stmt::Call("add".into(), vec![]));

parse_test!(
    call_with_args,
    "something(a,b, true ,123)",
    Stmt::Call(
        "something".into(),
        vec![
            Stmt::Word("a".into()),
            Stmt::Word("b".into()),
            Stmt::Bool(true),
            Stmt::Number(123)
        ]
    )
);

////
// op

parse_test!(
    basic_op,
    "2 + 2",
    Stmt::Call("+".into(), vec![Stmt::Number(2), Stmt::Number(2)])
);

////
// tag
