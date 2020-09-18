use hatter::{parse, scan, Expr};

fn print_nodes(i: usize, nodes: &[Expr]) {
    println!("Computed nodes:");
    for (x, token) in nodes.iter().enumerate() {
        let (bold, clear) = if x == i {
            ("\x1b[1;91m", "\x1b[0m")
        } else {
            ("", "")
        };
        println!("  {}({:?}, {}){}", bold, token.kind, token.literal(), clear);
    }
    println!("        left=want, right=got");
}

macro_rules! parse_test {
    ($name:ident, $code:expr, $($kind:expr,)+) => {
        #[test]
        fn $name() {
            let nodes = scan($code).and_then(|t| parse(t)).unwrap();
            let mut i = 0;
            $(
                let node = nodes.get(i).unwrap();
                if node.kind != $kind {
                    print_nodes(i, &nodes);
                }
                assert_eq!($kind, node.kind);
                { i += 1; }
            )+
            let mut nodes = nodes;
            while let Some(tok) = nodes.iter().last() {
                if tok.kind == Semi {
                    nodes.pop();
                } else {
                    break;
                }
            }
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
