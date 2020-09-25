use hatter::{compile, Stmt, Tag};

macro_rules! bx {
    ($code:expr) => {
        Box::new($code)
    };
}

macro_rules! tag {
    ($name:expr) => {
        Tag::new(Stmt::String($name.into()))
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

macro_rules! xparse_test {
    ($name:ident, $code:expr, $($stmt:expr),+) => {
        #[allow(unused)]
        fn $name() {}
    };
    ($name:ident, $code:expr, $($stmt:expr,)+) => {
        xparse_test!($name, $code, $($stmt),+)
    }
}

macro_rules! parse_test {
    ($name:ident, $code:expr, $($stmt:expr,)+) => {
        #[test]
        fn $name() {
            let nodes = compile($code).unwrap();
            let mut i = 0;
            $(
                let node = nodes.get(i).unwrap();
                if node != &$stmt {
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
parse_test!(number_range, "1..101", call!("..", num!(1), num!(101)));
parse_test!(
    number_range_inclusive,
    "500..=600",
    call!("..=", num!(500), num!(600))
);
parse_test!(
    number_range_spaced,
    "500 ..= 600",
    call!("..=", num!(500), num!(600))
);

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

parse_test!(one_word_is_just_a_word, "one", word!("one"),);
parse_test!(
    two_words_are_two_words,
    "one two",
    word!("one"),
    word!("two")
);
parse_test!(two_words_in_tag_are_a_string, "<b> one two", {
    let mut tag = tag!("b");
    tag.set_body(vec![Stmt::String("one two".into())]);
    Stmt::Tag(tag)
});
parse_test!(
    two_words_with_op_in_tag_are_a_string,
    "<b> One and... two!",
    {
        let mut tag = tag!("b");
        tag.set_body(vec![Stmt::String("One and... two!".into())]);
        Stmt::Tag(tag)
    }
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
    "true && false",
    call!("&&", boo!(true), boo!(false))
);

parse_test!(
    or_exprs,
    "true || false",
    call!("||", boo!(true), boo!(false))
);

parse_test!(
    and_or_exprs,
    "true && false || false",
    call!("&&", boo!(true), call!("||", boo!(false), boo!(false)))
);

parse_test!(
    basic_if_oneline,
    r#"if true do "something""#,
    Stmt::If(vec![(boo!(true), vec![string!("something")])])
);

parse_test!(
    basic_if_else_oneline,
    r#"if true do "something" else do "something else""#,
    Stmt::If(vec![
        (boo!(true), vec![string!("something")]),
        (boo!(true), vec![string!("something else")]),
    ])
);

parse_test!(
    basic_if_else_if_oneline,
    r#"if true do "something" else if true do "ahh" else do "something else""#,
    Stmt::If(vec![
        (boo!(true), vec![string!("something")]),
        (boo!(true), vec![string!("ahh")]),
        (boo!(true), vec![string!("something else")]),
    ])
);

parse_test!(
    basic_if_else_if_mixed,
    r#"if true do "something" else if true
    "ahh"
else do "something else""#,
    Stmt::If(vec![
        (boo!(true), vec![string!("something")]),
        (boo!(true), vec![string!("ahh")]),
        (boo!(true), vec![string!("something else")]),
    ])
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
    basic_def_with_whitespace_args,
    r#"
def greet(
    title
    name, suffix
)
    print("Hi there, {title}. {name}!")
"#,
    Stmt::Assign(
        "greet".into(),
        bx!(Stmt::Fn(
            vec!["title".into(), "name".into(), "suffix".into()],
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

parse_test!(
    fn_literal_oneline,
    "z := fn(x) return x + 1\nz(2)",
    Stmt::Assign(
        "z".into(),
        bx!(Stmt::Fn(
            vec!["x".into()],
            vec![Stmt::Return(bx!(call!("+", word!("x"), num!(1))))]
        )),
        false
    ),
    call!("z", num!(2))
);

parse_test!(
    fn_literal,
    r#"
z := fn(x)
    return x + 1
z(2)"#,
    Stmt::Assign(
        "z".into(),
        bx!(Stmt::Fn(
            vec!["x".into()],
            vec![Stmt::Return(bx!(call!("+", word!("x"), num!(1))))]
        )),
        false
    ),
    call!("z", num!(2))
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
    call!(
        "greet",
        Stmt::Args(vec![
            ("name".into(), "Bob".into()),
            ("title".into(), "Dr".into())
        ])
    )
);

////
// op

parse_test!(basic_op, "2 + 2", call!("+", num!(2), num!(2)));

parse_test!(eq_op, "2 == 2", call!("==", num!(2), num!(2)));

parse_test!(neq_op, "2 != 2", call!("!=", num!(2), num!(2)));

parse_test!(
    basic_chained_ops,
    "1 + 2 * 3",
    call!("+", num!(1), call!("*", num!(2), num!(3)))
);

parse_test!(
    basic_reverse_chained_ops,
    "1 * 2 + 3",
    call!("+", call!("*", num!(1), num!(2)), num!(3))
);

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
    let tag = tag!("some-tag");
    Stmt::Tag(tag)
});

parse_test!(self_closing_tag, "<some-tag/>", {
    let mut tag = tag!("some-tag");
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(another_self_closing_tag, "<end/>", {
    let mut tag = tag!("end");
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(basic_tag, "<b> Hey there", {
    let mut tag = tag!("b");
    tag.set_body(vec![string!("Hey there")]);
    Stmt::Tag(tag)
});

parse_test!(nested_tag, "<b> Hey <i> there", {
    let mut b = tag!("b");
    let mut i = tag!("i");
    i.set_body(vec![word!("there")]);
    b.set_body(vec![word!("Hey"), i.into()]);
    Stmt::Tag(b)
});

parse_test!(close_shortcut, "<b> Hey <i> there </> fren ", {
    let mut i = tag!("i");
    i.set_body(vec![word!("there")]);
    let mut b = tag!("b");
    b.set_body(vec![word!("Hey"), i.into(), word!("fren")]);
    Stmt::Tag(b)
});

parse_test!(basic_id_shortcut, "<b#shout> Hey yo", {
    let mut b = tag!("b");
    b.set_id(string!("shout"));
    b.set_body(vec![string!("Hey yo")]);
    Stmt::Tag(b)
});

parse_test!(basic_class_shortcut, "<span.clear> Welcome ", {
    let mut tag = tag!("span");
    tag.add_class(string!("clear"));
    tag.set_body(vec![word!("Welcome")]);
    Stmt::Tag(tag)
});

parse_test!(basic_classes, "<div.main.markdown> Yada yada... ", {
    let mut tag = tag!("div");
    tag.add_class(string!("main"));
    tag.add_class(string!("markdown"));
    tag.set_body(vec![string!("Yada yada...")]);
    Stmt::Tag(tag)
});

parse_test!(basic_name_shortcut, "<input@text/>", {
    let mut tag = tag!("input");
    tag.add_attr(string!("name"), string!("text"));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(basic_type_shortcut, "<input:submit/> ", {
    let mut tag = tag!("input");
    tag.add_attr(string!("type"), string!("submit"));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(mixed_shortcuts, "<div#main.markdown/> ", {
    let mut tag = tag!("div");
    tag.set_id(string!("main"));
    tag.add_class(string!("markdown"));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(mixed_input_shortcuts, "<input#focused:text@search  /> ", {
    let mut tag = tag!("input");
    tag.set_id(string!("focused"));
    tag.add_attr(string!("type"), string!("text"));
    tag.add_attr(string!("name"), string!("search"));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(basic_attribute, r#"<div data-id="45">"#, {
    let mut tag = tag!("div");
    tag.add_attr(string!("data-id"), string!("45"));
    Stmt::Tag(tag)
});

parse_test!(
    basic_attributes,
    r#"<input name="thing" placeholder="Other..."/>"#,
    {
        let mut tag = tag!("input");
        tag.add_attr(string!("name"), string!("thing"));
        tag.add_attr(string!("placeholder"), string!("Other..."));
        tag.close();
        Stmt::Tag(tag)
    }
);

parse_test!(js_attributes, "<div onclick=(alert('lol'))>Click me", {
    let mut tag = tag!("div");
    tag.add_attr(
        string!("onclick"),
        string!("(function(e){ (alert('lol')) })(event);"),
    );
    tag.set_body(vec![string!("Click me")]);
    Stmt::Tag(tag)
});

parse_test!(simple_code_attributes, "<div data-id=page.id>", {
    let mut tag = tag!("div");
    tag.add_attr(string!("data-id"), call!(".", word!("page"), string!("id")));
    Stmt::Tag(tag)
});

parse_test!(
    exprs_in_shorthand,
    "<div #cool={true} .class1={page.id} .class{num}={page.id} .{all} #{any} :{cool} data-curly={page.id} data-expr={page.id} data-math={2 + 2} data-wow='wow!'>",
    {
        let mut tag = tag!("div");
        tag.set_id(call!("when", boo!(true), string!("cool")));
        tag.add_class(call!(
            "when",
            call!(".", word!("page"), string!("id")),
            string!("class1")
        ));
        tag.add_class(call!(
            "when",
            call!(".", word!("page"), string!("id")),
            call!("concat", string!("class"), word!("num"))
        ));
        tag.add_class(word!("all"));
        tag.set_id(word!("any"));
        tag.add_attr(string!("type"), word!("cool"));
        tag.add_attr(
            string!("data-curly"),
            call!(".", word!("page"), string!("id")),
        );
        tag.add_attr(
            string!("data-expr"),
            call!(".", word!("page"), string!("id")),
        );
        tag.add_attr(
            string!("data-math"),
            call!("+", num!(2), num!(2))
        );
        tag.add_attr(
            string!("data-wow"),
            string!("wow!")
        );
        Stmt::Tag(tag)
    }
);

parse_test!(shorthand_conditionals, "<div#id=has-id>", {
    let mut tag = tag!("div");
    tag.set_id(call!("when", word!("has-id"), string!("id")));
    Stmt::Tag(tag)
});

parse_test!(code_expr_attributes, "<div data-value={2 + 3}>", {
    let mut tag = tag!("div");
    tag.add_attr(string!("data-value"), call!("+", num!(2), num!(3)));
    Stmt::Tag(tag)
});

parse_test!(code_expr_attribute_names, "<div data-{2 + 2}=true>", {
    let mut tag = tag!("div");
    tag.add_attr(
        call!("concat", string!("data-"), call!("+", num!(2), num!(2))),
        boo!(true),
    );
    Stmt::Tag(tag)
});

parse_test!(angle_interpolated_id_full, "<div#{cool}>", {
    let mut tag = tag!("div");
    tag.set_id(word!("cool"));
    Stmt::Tag(tag)
});

parse_test!(angle_interpolated_id_partial, "<div#page-{id}>", {
    let mut tag = tag!("div");
    tag.set_id(call!("concat", string!("page-"), word!("id")));
    Stmt::Tag(tag)
});

parse_test!(angle_interpolated_class_full, "<div.{cool}>", {
    let mut tag = tag!("div");
    tag.add_class(word!("cool"));
    Stmt::Tag(tag)
});

parse_test!(angle_interpolated_class_partial, "<div.page-{id}>", {
    let mut tag = tag!("div");
    tag.add_class(call!("concat", string!("page-"), word!("id")));
    Stmt::Tag(tag)
});

parse_test!(angle_interpolated_type_full, "<input:{cool}>", {
    let mut tag = tag!("input");
    tag.add_attr(string!("type"), word!("cool"));
    Stmt::Tag(tag)
});

parse_test!(angle_interpolated_type_partial, "<input:page-{id}>", {
    let mut tag = tag!("input");
    tag.add_attr(
        string!("type"),
        call!("concat", string!("page-"), word!("id")),
    );
    Stmt::Tag(tag)
});

parse_test!(angle_interpolated_name_full, "<input@{cool}>", {
    let mut tag = tag!("input");
    tag.add_attr(string!("name"), word!("cool"));
    Stmt::Tag(tag)
});

parse_test!(angle_interpolated_name_partial, "<input@page-{id}>", {
    let mut tag = tag!("input");
    tag.add_attr(
        string!("name"),
        call!("concat", string!("page-"), word!("id")),
    );
    Stmt::Tag(tag)
});

parse_test!(
    angle_interpolated_attr_name_partial,
    "<div my-{name}-name=true/>",
    {
        let mut tag = tag!("div");
        tag.add_attr(
            call!("concat", string!("my-"), word!("name"), string!("-name")),
            boo!(true),
        );
        tag.close();
        Stmt::Tag(tag)
    }
);

parse_test!(angle_interpolated_attr_name_full, "<div {name}=true/>", {
    let mut tag = tag!("div");
    tag.add_attr(word!("name"), boo!(true));
    tag.close();
    Stmt::Tag(tag)
});

parse_test!(
    angle_interpolated_attr_value_partial,
    "<div data-name=some-{thing}/>",
    {
        let mut tag = tag!("div");
        tag.add_attr(
            string!("data-name"),
            call!("concat", string!("some-"), word!("thing")),
        );
        tag.close();
        Stmt::Tag(tag)
    }
);

parse_test!(
    angle_interpolated_attr_value_full,
    "<div data-name={true}/>",
    {
        let mut tag = tag!("div");
        tag.add_attr(string!("data-name"), boo!(true));
        tag.close();
        Stmt::Tag(tag)
    }
);

parse_test!(
    tag_everything,
    "<div#id.class1.class-2=is-it? :why-not @sure onclick=(alert(`it's ${2 + 2}`)) data-id=123 data-{value}=compute(value) />",
    {
        let mut tag = tag!("div");
        tag.set_id(string!("id"));
        tag.add_class(string!("class1"));
        tag.add_class(call!("when", word!("is-it?"), string!("class-2")));
        tag.add_attr(string!("type"), string!("why-not"));
        tag.add_attr(string!("name"), string!("sure"));
        tag.add_attr(string!("onclick"), string!("(function(e){ (alert(`it's ${2 + 2}`)) })(event);"));
        tag.add_attr(string!("data-id"), num!(123));
        tag.add_attr(call!("concat", string!("data-"), word!("value")), call!("compute", word!("value")));
        tag.close();
        Stmt::Tag(tag)
    }
);

parse_test!(
    random_tag,
    r#"<#my-id> Just some <.bold>cool</> "content.""#,
    {
        let mut tag = tag!("div");
        tag.set_id(string!("my-id"));
        let mut bold = tag!("div");
        bold.add_class(string!("bold"));
        bold.set_body(vec![word!("cool")]);
        tag.set_body(vec![string!("Just some"), bold.into(), string!("content.")]);
        Stmt::Tag(tag)
    }
);

parse_test!(
    tag_with_many_classes,
    "<div.with.many.classes> My <em.big>my</>'!'",
    {
        let mut tag = tag!("div");
        tag.add_class(string!("with"));
        tag.add_class(string!("many"));
        tag.add_class(string!("classes"));
        let mut em = tag!("em");
        em.add_class(string!("big"));
        em.set_body(vec![word!("my")]);
        tag.set_body(vec![word!("My"), em.into(), string!("!")]);
        Stmt::Tag(tag)
    }
);

parse_test!(
    indented_form,
    r#"
<form GET="/search">
  <input:text@query/>
  <input:submit/>
"#,
    {
        let mut form = tag!("form");
        form.add_attr(string!("GET"), string!("/search"));
        let mut text = tag!("input");
        text.add_attr(string!("type"), string!("text"));
        text.add_attr(string!("name"), string!("query"));
        text.close();
        let mut btn = tag!("input");
        btn.add_attr(string!("type"), string!("submit"));
        btn.close();
        form.set_body(vec![text.into(), btn.into()]);
        Stmt::Tag(form)
    }
);

parse_test!(
    full_form,
    r#"
<form POST="/info">
   <h3> Your Information
   <label> <input@name:text placeholder="Name..."/>
   <label> <input@age:text placeholder="Age..."/>
   <br/>
   <input:submit/>
   <input:reset/>
"#,
    {
        let mut form = tag!("form");
        form.add_attr(string!("POST"), string!("/info"));

        let mut h3 = tag!("h3");
        h3.set_body(vec![string!("Your Information")]);

        let mut lbl1 = tag!("label");
        let mut name = tag!("input");
        name.add_attr(string!("name"), string!("name"));
        name.add_attr(string!("type"), string!("text"));
        name.add_attr(string!("placeholder"), string!("Name..."));
        name.close();
        lbl1.set_body(vec![name.into()]);

        let mut lbl2 = tag!("label");
        let mut age = tag!("input");
        age.add_attr(string!("name"), string!("age"));
        age.add_attr(string!("type"), string!("text"));
        age.add_attr(string!("placeholder"), string!("Age..."));
        age.close();
        lbl2.set_body(vec![age.into()]);

        let mut br = tag!("br");
        br.close();

        let mut submit = tag!("input");
        submit.add_attr(string!("type"), string!("submit"));
        submit.close();

        let mut reset = tag!("input");
        reset.add_attr(string!("type"), string!("reset"));
        reset.close();

        form.set_body(
            vec![h3, lbl1, lbl2, br, submit, reset]
                .into_iter()
                .map(|x| Stmt::Tag(x))
                .collect(),
        );

        Stmt::Tag(form)
    }
);

parse_test!(
    nested_tag_list,
    r#"
<ul#menu>
    <li.item#burger> Burger
    <li.item#fries> Fries
    <li.item#shake> Milkshake
"#,
    {
        let mut ul = tag!("ul");
        ul.set_id(string!("menu"));

        let mut burger = tag!("li");
        burger.add_class(string!("item"));
        burger.set_id(string!("burger"));
        burger.set_body(vec![word!("Burger")]);

        let mut fries = tag!("li");
        fries.add_class(string!("item"));
        fries.set_id(string!("fries"));
        fries.set_body(vec![word!("Fries")]);

        let mut shake = tag!("li");
        shake.add_class(string!("item"));
        shake.set_id(string!("shake"));
        shake.set_body(vec![word!("Milkshake")]);

        ul.set_body(vec![burger.into(), fries.into(), shake.into()]);

        Stmt::Tag(ul)
    }
);

xparse_test!(
    nested_nested_tags,
    r#"
<div> <ul>
    <li> Fun
<span> Top-level
"#,
    {
        let mut div = tag!("div");
        let mut ul = tag!("ul");
        let mut li = tag!("li");
        li.set_body(vec![word!("Fun")]);
        ul.set_body(vec![li.into()]);
        div.set_body(vec![ul.into()]);
        Stmt::Tag(div)
    },
    {
        let mut span = tag!("span");
        span.set_body(vec![word!("Top-level")]);
        Stmt::Tag(span)
    }
);
