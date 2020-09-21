use hatter::{parse, scan, Stmt, Tag};

macro_rules! bx {
    ($code:expr) => {
        Box::new($code)
    };
}

macro_rules! tag {
    ($name:expr) => {
        Tag::new(Stmt::Word($name.into()))
    };
}

macro_rules! boo {
    ($boo:expr) => {
        Stmt::Bool($boo)
    };
}

macro_rules! word {
    ($code:expr) => {
        Stmt::Word($code.into())
    };
}

macro_rules! string {
    ($code:expr) => {
        Stmt::String($code.into())
    };
}

macro_rules! num {
    ($num:expr) => {
        Stmt::Number($num)
    };
}

macro_rules! call {
    ($name:expr, $($arg:expr),+) => {
        Stmt::Call($name.into(), vec![$($arg),+])
    };
    ($name:expr) => {
        Stmt::Call($name.into(), vec![])
    };
}

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

parse_test!(true_bool, "true", boo!(true));
parse_test!(false_bool, "false", boo!(false));

////
// number

parse_test!(positive_number, "100", num!(100));
parse_test!(negative_number, "-400", num!(-400));
parse_test!(underscore_number, "1_422_200", num!(1_422_200));
parse_test!(negative_underscore_number, "-1_422_200", num!(-1_422_200));
parse_test!(binary_number, "0b101", num!(5));
parse_test!(octal_number, "0o755", num!(493));
parse_test!(hex_number, "0xdeadbeef", num!(3735928559));
// parse_test!(float_number, "3.14", num!(3));
// parse_test!(negative_float_number, "-1230.3552", num!(-1230));

////
// string

parse_test!(
    basic_string,
    r#""just a regular string""#,
    string!("just a regular string")
);
parse_test!(
    interpolated_string,
    r#""just checking that {2 + 2} = 4""#,
    call!(
        "concat",
        string!("just checking that "),
        call!("+", num!(2), num!(2)),
        string!(" = 4")
    )
);
parse_test!(
    single_quote_string,
    "'single quote'",
    string!("single quote")
);
parse_test!(
    escaped_single_quote_string,
    r#"'what\'s up'"#,
    string!("what\\'s up")
);
parse_test!(grave_string, r#"`one two three`"#, string!("one two three"));
parse_test!(
    triple_string,
    r#" """one two three""" "#,
    string!("one two three")
);
parse_test!(
    triple_single_string,
    r#"'''one two three'''"#,
    string!("one two three")
);
parse_test!(
    triple_grave_string,
    r#"```one two three```"#,
    string!("one two three")
);
parse_test!(
    triple_string_with_breaks,
    r#""""
one
two
three
""""#,
    string!("\none\ntwo\nthree\n")
);

////
// list

parse_test!(
    basic_list,
    "[1,2,3]",
    Stmt::List(vec![num!(1), num!(2), num!(3)])
);

parse_test!(
    lists_ignore_whitespace,
    "[
        1,
2,
        3
                ]",
    Stmt::List(vec![num!(1), num!(2), num!(3)])
);

parse_test!(
    lists_treat_newlines_as_optional_commas,
    "
[
    1
    2
    3
]",
    Stmt::List(vec![num!(1), num!(2), num!(3)])
);

parse_test!(
    string_list,
    "[ 'one',
    'two','three']",
    Stmt::List(vec![string!("one"), string!("two"), string!("three")])
);

////
// map

parse_test!(
    basic_map,
    "{ one: 1, two: 2, three: 3 }",
    Stmt::Map(vec![
        ("one".to_string(), num!(1)),
        ("two".to_string(), num!(2)),
        ("three".to_string(), num!(3))
    ])
);

parse_test!(
    basic_map_spanning_newlines,
    "{
        one: 1
                two: 2, three:
        3}",
    Stmt::Map(vec![
        ("one".to_string(), num!(1)),
        ("two".to_string(), num!(2)),
        ("three".to_string(), num!(3))
    ])
);

////
// if

parse_test!(
    basic_if,
    r#"
if true
    "something"
"#,
    Stmt::If(vec![(boo!(true), vec![string!("something")])])
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
        (boo!(true), vec![string!("something")]),
        (boo!(true), vec![string!("something else")]),
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
        (call!(">", word!("i"), num!(0)), vec![string!("greater")]),
        (call!("<", word!("i"), num!(0)), vec![string!("lesser")]),
        (boo!(true), vec![string!("cero")]),
    ])
);

parse_test!(
    and_exprs,
    "true and false",
    call!("and", boo!(true), boo!(false))
);

parse_test!(
    or_exprs,
    "true or false",
    call!("or", boo!(true), boo!(false))
);

parse_test!(
    and_or_exprs,
    "true and false or false",
    call!("and", boo!(true), call!("or", boo!(false), boo!(false)))
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
        bx!(word!("abc")),
        vec![call!("print", word!("v"))]
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
        bx!(word!("abc")),
        vec![call!(
            "print",
            call!("concat", word!("i"), string!(": "), word!("v"))
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
        bx!(word!("abc")),
        vec![
            call!(
                "print",
                call!("concat", word!("i"), string!(": "), word!("v"))
            ),
            Stmt::If(vec![(
                call!(">", word!("i"), num!(20)),
                vec![word!("break")]
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
        bx!(word!("abc")),
        vec![
            Stmt::If(vec![(
                call!("<", word!("i"), num!(20)),
                vec![word!("continue")]
            )]),
            call!(
                "print",
                call!("concat", word!("i"), string!(": "), word!("v"))
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
        bx!(Stmt::List(vec![num!(1), num!(2), num!(3)])),
        vec![call!("print", call!("*", word!("x"), num!(10)))]
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
        bx!(Stmt::Map(vec![
            ("first".to_string(), string!("Bilbo")),
            ("last".to_string(), string!("Swaggins")),
        ])),
        vec![call!(
            "print",
            call!("concat", word!("k"), string!(": "), word!("v"))
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
def greet()
    print("HEY!!")
"#,
    Stmt::Assign(
        "greet".into(),
        bx!(Stmt::Fn(vec![], vec![call!("print", string!("HEY!!"))])),
        false
    )
);

parse_test!(
    basic_def_with_arg,
    r#"
def greet(name)
    print("Hi there, {name}!")
"#,
    Stmt::Assign(
        "greet".into(),
        bx!(Stmt::Fn(
            vec!["name".into()],
            vec![call!(
                "print",
                call!("concat", string!("Hi there, "), word!("name"), string!("!"))
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
        bx!(Stmt::Fn(
            vec!["title".into(), "name".into()],
            vec![call!(
                "print",
                call!(
                    "concat",
                    string!("Hi there, "),
                    word!("title"),
                    string!(". "),
                    word!("name"),
                    string!("!")
                )
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
        bx!(Stmt::Fn(
            vec!["a".into(), "b".into()],
            vec![
                Stmt::If(vec![(
                    call!("==", call!("type", word!("a")), string!("list")),
                    vec![call!("append", word!("a"), word!("b"))]
                )]),
                Stmt::Return(bx!(word!("a")))
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
    Stmt::Assign("a".into(), bx!(num!(123)), false)
);

////
// reassign

parse_test!(
    reassign,
    "a = 123",
    Stmt::Assign("a".into(), bx!(num!(123)), true)
);

////
// call

parse_test!(basic_call, "word(true)", call!("word", boo!(true)));

parse_test!(empty_call, "add()", call!("add"));

parse_test!(
    call_with_args,
    "something(a,b, true ,123)",
    call!("something", word!("a"), word!("b"), boo!(true), num!(123))
);

parse_test!(
    call_with_whitespace,
    "something(a,
        b,
        true,
        123
    )",
    call!("something", word!("a"), word!("b"), boo!(true), num!(123))
);

parse_test!(
    call_with_named_args,
    "greet(name: 'Bob', title: 'Dr')",
    call!("something", word!("a"), word!("b"), boo!(true), num!(123))
);

////
// op

parse_test!(basic_op, "2 + 2", call!("+", num!(2), num!(2)));

parse_test!(eq_op, "2 == 2", call!("==", num!(2), num!(2)));

parse_test!(neq_op, "2 != 2", call!("!=", num!(2), num!(2)));

parse_test!(
    chained_ops,
    "2 + 20 * 10 - 5",
    call!(
        "-",
        call!("+", num!(2), call!("*", num!(20), num!(10))),
        num!(5)
    )
);

////
// tag

parse_test!(reg_tag, "<some-tag></some-tag>", {
    let mut tag = tag!("some-tag");
    Stmt::Tag(tag)
});

parse_test!(self_closing_tag, "<some-tag/>", {
    let mut tag = tag!("some-tag");
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(basic_tag, "<b> Hey there", {
    let mut tag = tag!("b");
    tag.set_body(vec![word!("Hey"), word!("there")]);
    Stmt::Tag(tag)
});

parse_test!(self_closing_tag, "<end/>", {
    let mut tag = tag!("end");
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(nested_tag, "<b> Hey <i> there", {
    let mut b = tag!("b");
    let mut i = tag!("i");
    i.set_body(vec![word!("there")]);
    b.set_body(vec![word!("Hey"), Stmt::Tag(i)]);
    Stmt::Tag(b)
});

parse_test!(close_shortcut, "<b> Hey <i> there </> fren ", {
    let mut b = tag!("b");
    let mut i = tag!("i");
    i.set_body(vec![word!("there")]);
    b.set_body(vec![word!("Hey"), Stmt::Tag(i), word!("fren")]);
    Stmt::Tag(b)
});

parse_test!(basic_id_shortcut, "<b#shout> Hey yo", {
    let mut b = tag!("b");
    b.set_id(word!("shout"));
    b.set_body(vec![word!("Hey"), word!("yo")]);
    Stmt::Tag(b)
});

parse_test!(basic_class_shortcut, "<span.clear> Welcome ", {
    let mut tag = tag!("span");
    tag.add_class(word!("clear"));
    tag.set_body(vec![word!("Welcome")]);
    Stmt::Tag(tag)
});

parse_test!(basic_classes, "<div.main.markdown> Yada yada... ", {
    let mut tag = tag!("div");
    tag.add_class(word!("main"));
    tag.add_class(word!("markdown"));
    tag.set_body(vec![word!("Yada"), word!("yada")]);
    Stmt::Tag(tag)
});

parse_test!(basic_name_shortcut, "<input@text/>", {
    let mut tag = tag!("input");
    tag.add_attr(word!("name"), word!("text"));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(basic_type_shortcut, "<input:submit/> ", {
    let mut tag = tag!("input");
    tag.add_attr(word!("type"), word!("submit"));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(mixed_shortcuts, "<div#main.markdown/> ", {
    let mut tag = tag!("div");
    tag.set_id(word!("main"));
    tag.add_class(word!("markdown"));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(mixed_input_shortcuts, "<input#focused:text@search  /> ", {
    let mut tag = tag!("input");
    tag.set_id(word!("focused"));
    tag.add_attr(word!("type"), word!("text"));
    tag.add_attr(word!("name"), word!("search"));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(basic_attribute, r#"<div data-id="45">"#, {
    let mut tag = tag!("div");
    tag.add_attr(word!("data-id"), string!("45"));
    Stmt::Tag(tag)
});

parse_test!(
    basic_attributes,
    r#"<input name="thing" placeholder="Other..."/>"#,
    {
        let mut tag = tag!("input");
        tag.add_attr(word!("name"), string!("thing"));
        tag.add_attr(word!("placeholder"), string!("Other..."));
        tag.close();
        Stmt::Tag(tag)
    }
);

parse_test!(js_attributes, "<div onclick=(alert('lol'))>Click me", {
    let mut tag = tag!("div");
    tag.add_attr(word!("onclick"), string!("javascript..."));
    tag.set_body(vec![string!("Click me")]);
    Stmt::Tag(tag)
});

parse_test!(simple_code_attributes, "<div data-id=page.id>", {
    let mut tag = tag!("div");
    tag.add_attr(word!("data-id"), call!(".", word!("page"), word!("id")));
    Stmt::Tag(tag)
});

parse_test!(shorthand_conditionals, "<div#id=has-id>", {
    let mut tag = tag!("div");
    tag.set_id(call!("when", word!("has-id"), word!("id")));
    Stmt::Tag(tag)
});

parse_test!(code_expr_attributes, "<div data-value={2 + 3}>", {
    let mut tag = tag!("div");
    tag.add_attr(word!("data-value"), call!("+", num!(2), num!(3)));
    Stmt::Tag(tag)
});

parse_test!(
    tag_everything,
    "<div#id.class1.class-2=is-it? :why-not @sure onclick=(alert(`it's ${2 + 2}`)) data-id=123 data-{value}=compute(value) />",
    {
        let mut tag = tag!("div");
        tag.set_id(word!("id"));
        tag.add_class(word!("class1"));
        tag.add_class(call!("when", word!("is-it?"), word!("class-2")));
        tag.add_attr(word!("type"), word!("why-not"));
        tag.add_attr(word!("name"), word!("sure"));
        tag.add_attr(word!("onclick"), string!("JS..."));
        tag.add_attr(word!("data-id"), num!(123));
        tag.add_attr(word!("data-{value}"), call!("compute", word!("value")));
        tag.close();
        Stmt::Tag(tag)
    }
);

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
