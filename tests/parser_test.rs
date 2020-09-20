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
// parse_test!(float_number, "3.14", Stmt::Number(3));
// parse_test!(negative_float_number, "-1230.3552", Stmt::Number(-1230));

////
// string

parse_test!(
    basic_string,
    r#""just a regular string""#,
    Stmt::String("just a regular string".into())
);
parse_test!(
    interpolated_string,
    r#""just checking that {2 + 2} = 4""#,
    Stmt::Call(
        "concat".into(),
        vec![
            Stmt::String("just checking that ".into()),
            Stmt::Call("+".into(), vec![Stmt::Number(2), Stmt::Number(2)]),
            Stmt::String(" = 4".into()),
        ]
    )
);
parse_test!(
    single_quote_string,
    "'single quote'",
    Stmt::String("single quote".into())
);
parse_test!(
    escaped_single_quote_string,
    r#"'what\'s up'"#,
    Stmt::String("what\\'s up".into())
);
parse_test!(
    grave_string,
    r#"`one two three`"#,
    Stmt::String("one two three".into())
);
parse_test!(
    triple_string,
    r#" """one two three""" "#,
    Stmt::String("one two three".into())
);
parse_test!(
    triple_single_string,
    r#"'''one two three'''"#,
    Stmt::String("one two three".into())
);
parse_test!(
    triple_grave_string,
    r#"```one two three```"#,
    Stmt::String("one two three".into())
);
parse_test!(
    triple_string_with_breaks,
    r#""""
one
two
three
""""#,
    Stmt::String("\none\ntwo\nthree\n".into())
);

////
// if

parse_test!(
    basic_if,
    r#"
if true
    "something"
"#,
    Stmt::If(vec![(
        Stmt::Bool(true),
        vec![Stmt::String("something".into())]
    )])
);

parse_test!(
    basic_if_else,
    r#"
if true
    "something"
else
    "something else"
"#,
    Stmt::If(vec![
        (Stmt::Bool(true), vec![Stmt::String("something".into())]),
        (
            Stmt::Bool(true),
            vec![Stmt::String("something else".into())]
        ),
    ])
);

parse_test!(
    basic_if_with_elses,
    r#"
if i > 0
    "greater"
else if i < 0
    "lesser"
else
    "cero"
"#,
    Stmt::If(vec![
        (
            Stmt::Call(">".into(), vec![Stmt::Word("i".into()), Stmt::Number(0)]),
            vec![Stmt::String("greater".into())]
        ),
        (
            Stmt::Call("<".into(), vec![Stmt::Word("i".into()), Stmt::Number(0)]),
            vec![Stmt::String("lesser".into())]
        ),
        (Stmt::Bool(true), vec![Stmt::String("cero".into())]),
    ])
);

parse_test!(
    and_exprs,
    "true and false",
    Stmt::Call("and".into(), vec![Stmt::Bool(true), Stmt::Bool(false)])
);

parse_test!(
    or_exprs,
    "true or false",
    Stmt::Call("or".into(), vec![Stmt::Bool(true), Stmt::Bool(false)])
);

parse_test!(
    and_or_exprs,
    "true and false or false",
    Stmt::Call(
        "and".into(),
        vec![
            Stmt::Bool(true),
            Stmt::Call("or".into(), vec![Stmt::Bool(false), Stmt::Bool(false)])
        ]
    )
);

////
// for

parse_test!(
    basic_for,
    r#"
for v in abc
    print(v)
"#,
    Stmt::For(
        None,
        "v".into(),
        Box::new(Stmt::Word("abc".into())),
        vec![Stmt::Call("print".into(), vec![Stmt::Word("v".into())])]
    )
);

parse_test!(
    basic_for_with_index,
    r#"
for i, v in abc
    print("{i}: {v}")
"#,
    Stmt::For(
        Some("i".into()),
        "v".into(),
        Box::new(Stmt::Word("abc".into())),
        vec![Stmt::Call(
            "print".into(),
            vec![Stmt::Call(
                "concat".into(),
                vec![
                    Stmt::Word("i".into()),
                    Stmt::String(": ".into()),
                    Stmt::Word("v".into())
                ]
            )]
        )]
    )
);

parse_test!(
    basic_break,
    r#"
for i, v in abc
    print("{i}: {v}")
    if i > 20
        break
"#,
    Stmt::For(
        Some("i".into()),
        "v".into(),
        Box::new(Stmt::Word("abc".into())),
        vec![
            Stmt::Call(
                "print".into(),
                vec![Stmt::Call(
                    "concat".into(),
                    vec![
                        Stmt::Word("i".into()),
                        Stmt::String(": ".into()),
                        Stmt::Word("v".into())
                    ]
                )]
            ),
            Stmt::If(vec![(
                Stmt::Call(">".into(), vec![Stmt::Word("i".into()), Stmt::Number(20)]),
                vec![Stmt::Word("break".into())]
            )])
        ]
    )
);

parse_test!(
    basic_continue,
    r#"
for i, v in abc
    if i < 20
        continue
    print("{i}: {v}")
"#,
    Stmt::For(
        Some("i".into()),
        "v".into(),
        Box::new(Stmt::Word("abc".into())),
        vec![
            Stmt::If(vec![(
                Stmt::Call("<".into(), vec![Stmt::Word("i".into()), Stmt::Number(20)]),
                vec![Stmt::Word("continue".into())]
            )]),
            Stmt::Call(
                "print".into(),
                vec![Stmt::Call(
                    "concat".into(),
                    vec![
                        Stmt::Word("i".into()),
                        Stmt::String(": ".into()),
                        Stmt::Word("v".into())
                    ]
                )]
            )
        ]
    )
);

parse_test!(
    basic_for_list,
    r#"
for x in [1,2,3]
    print(x * 10)
"#,
    Stmt::For(
        None,
        "x".into(),
        Box::new(Stmt::List(vec![
            Stmt::Number(1),
            Stmt::Number(2),
            Stmt::Number(3)
        ])),
        vec![Stmt::Call(
            "print".into(),
            vec![Stmt::Call(
                "*".into(),
                vec![Stmt::Word("x".into()), Stmt::Number(10)]
            )]
        )]
    )
);

parse_test!(
    basic_for_map,
    r#"
for k, v in { first: 'Bilbo', last: 'Swaggins' }
    print("{k}: {v}")
"#,
    Stmt::For(
        Some("k".into()),
        "v".into(),
        Box::new(Stmt::Map(vec![
            ("first".to_string(), Stmt::String("Bilbo".into())),
            ("last".to_string(), Stmt::String("Swaggins".into())),
        ])),
        vec![Stmt::Call(
            "print".into(),
            vec![Stmt::Call(
                "concat".into(),
                vec![
                    Stmt::Word("k".into()),
                    Stmt::String(": ".into()),
                    Stmt::Word("v".into())
                ]
            )]
        )]
    )
);

////
// while

// parse_test!(
//     basic_while,
//     r#"
// while i > 0
//     do-something()
//     then-do-something-else()
// "#,
//     Stmt::None
// );

////
// def

parse_test!(
    basic_def,
    r#"
def greet(name)
    print("Hi there, {name}!")
"#,
    Stmt::Assign(
        "greet".into(),
        Box::new(Stmt::Fn(
            vec!["name".into()],
            vec![Stmt::Call(
                "print".into(),
                vec![Stmt::Call(
                    "concat".into(),
                    vec![
                        Stmt::String("Hi there, ".into()),
                        Stmt::Word("name".into()),
                        Stmt::String("!".into())
                    ]
                )]
            )]
        )),
        false
    )
);

parse_test!(
    basic_def_with_args,
    r#"
def greet(title, name)
    print("Hi there, {title}. {name}!")
"#,
    Stmt::Assign(
        "greet".into(),
        Box::new(Stmt::Fn(
            vec!["title".into(), "name".into()],
            vec![Stmt::Call(
                "print".into(),
                vec![Stmt::Call(
                    "concat".into(),
                    vec![
                        Stmt::String("Hi there, ".into()),
                        Stmt::Word("title".into()),
                        Stmt::String(". ".into()),
                        Stmt::Word("name".into()),
                        Stmt::String("!".into())
                    ]
                )]
            )]
        )),
        false
    )
);

parse_test!(
    def_op,
    r#"
def <<(a, b)
    if type(a) == 'list'
        append(a, b)
    return a
"#,
    Stmt::Assign(
        "<<".into(),
        Box::new(Stmt::Fn(
            vec!["a".into(), "b".into()],
            vec![
                Stmt::If(vec![(
                    Stmt::Call(
                        "==".into(),
                        vec![
                            Stmt::Call("type".into(), vec![Stmt::Word("a".into())]),
                            Stmt::Word("list".into())
                        ]
                    ),
                    vec![Stmt::Call(
                        "append".into(),
                        vec![Stmt::Word("a".into()), Stmt::Word("b".into())]
                    )]
                )]),
                Stmt::Return(Box::new(Stmt::Word("a".into())))
            ]
        )),
        false
    )
);

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

// parse_test!(
//     call_with_whitespace,
//     "something(a,
//         b,
//         true,
//         123
//     )",
//     Stmt::Call(
//         "something".into(),
//         vec![
//             Stmt::Word("a".into()),
//             Stmt::Word("b".into()),
//             Stmt::Bool(true),
//             Stmt::Number(123)
//         ]
//     )
// );

////
// op

parse_test!(
    basic_op,
    "2 + 2",
    Stmt::Call("+".into(), vec![Stmt::Number(2), Stmt::Number(2)])
);

parse_test!(
    eq_op,
    "2 == 2",
    Stmt::Call("==".into(), vec![Stmt::Number(2), Stmt::Number(2)])
);

parse_test!(
    neq_op,
    "2 != 2",
    Stmt::Call("!=".into(), vec![Stmt::Number(2), Stmt::Number(2)])
);

parse_test!(
    chained_ops,
    "2 + 20 * 10 - 5",
    Stmt::Call(
        "-".into(),
        vec![
            Stmt::Call(
                "+".into(),
                vec![
                    Stmt::Number(2),
                    Stmt::Call("*".into(), vec![Stmt::Number(20), Stmt::Number(10),])
                ]
            ),
            Stmt::Number(5)
        ]
    )
);

////
// tag

// parse_test!(basic_tag, "<some-tag></some-tag>", Stmt::None);
// parse_test!(basic_closed_tag, "<some-tag/>", Stmt::None);

// TODO:

// <div.with.many.classes> My <em.big>my</>!

// <form GET="/search">
//   <input:text@query/>
//   <input:submit/>

// <form POST="/info">
//   <h3> Your Information
//    <label> <input@name:text placeholder="Name..."/>
//    <label> <input@age:text placeholder="Age..."/>
//    <br/>
//    <input:submit/>
//    <input:reset/>

// <ul#menu>
//     <li.item#burger> Burger
//     <li.item#fries> Fries
//     <li.item#milkshake> Milkshake

// <#my-id> Just some <.bold>cool</> "content."
