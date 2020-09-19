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

parse_test!(
    basic_op,
    "2 + 2",
    Stmt::Call("+".into(), vec![Stmt::Number(2), Stmt::Number(2)])
);

parse_test!(
    basic_call,
    "word(true)",
    Stmt::Call("word".into(), vec![Stmt::Bool(true)])
);
