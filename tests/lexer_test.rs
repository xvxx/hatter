use hatter::{scan, Syntax::*, Token};

macro_rules! scan {
    ($code:expr) => {
        scan($code).unwrap()
    };
}

fn print_nodes(i: usize, nodes: &[Token]) {
    println!("Computed tokens:");
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

macro_rules! scan_test {
    ($name:ident, $code:expr, $($kind:expr,)+) => {
        #[test]
        fn $name() {
            let nodes = scan!($code);
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
        scan_test!($name, $code, $($kind,)+);
    }
}

////
// Numbers

scan_test!(positive_number, "100", Number);
scan_test!(negative_number, "-400", Number);
scan_test!(underscore_number, "1_422_200", Number);
scan_test!(negative_underscore_number, "-1_422_200", Number);
scan_test!(binary_number, "0b101", Number);
scan_test!(octal_number, "0o755", Number);
scan_test!(hex_number, "0xdeadbeef", Number);
scan_test!(float_number, "3.14", Number);
scan_test!(negative_float_number, "-1230.3552", Number);

////
// Comments

scan_test!(basic_comment, "123 # Something something.", Number);
scan_test!(
    another_basic_comment,
    "456\n# Something something.\n789",
    Number,
    Semi,
    Number,
);

////
// Strings

scan_test!(basic_string, r#""just a regular string""#, String(false));
scan_test!(
    interpolated_string,
    r#""just checking that {2 + 2} = 4""#,
    String(true)
);
scan_test!(single_quote_string, r#"'single quote'"#, String(false));
scan_test!(
    escaped_single_quote_string,
    r#"'what\'s up'"#,
    String(false)
);
scan_test!(grave_string, r#"`one two three`"#, String(false));
scan_test!(triple_string, r#" """one two three""" "#, String(false));
scan_test!(
    triple_single_string,
    r#"'''one two three'''"#,
    String(false)
);
scan_test!(triple_grave_string, r#"```one two three```"#, String(false));
scan_test!(
    triple_string_with_breaks,
    r#""""
one
two
three
""""#,
    String(false)
);

////
// Words

scan_test!(simple_word, "sup", Word);
scan_test!(title_word, "Yep", Word);
scan_test!(underscore_word, "some_thing", Word);
scan_test!(underscore_word2, "_some_other_thing_", Word);
scan_test!(dash_word, "some-thing", Word);
scan_test!(dash_word2, "some-thing-", Word);
scan_test!(numbered_word, "one123", Word);
scan_test!(word_with_brackets, "word-{with brackets}-in-it", Word);
scan_test!(
    word_ending_with_brackets,
    "word-ending-{with brackets}",
    Word
);
scan_test!(word_ending_with_question, "word-ending-in?", Word);
scan_test!(and_is_a_word, "and", Word);

////
// Ops

scan_test!(simple_op, "+", Op);
scan_test!(lonley_dash_is_an_op, "-", Op);
scan_test!(two_char_op, ":=", Op);
scan_test!(gt_op, ">", Op);
scan_test!(random_op, "*@$%^$*", Op);
scan_test!(dot_op, ".", Op);
scan_test!(double_dot_op, "..", Op);
scan_test!(question_op, "?", Op);
scan_test!(bang_op, "!", Op);
scan_test!(gt_is_op, "4 > 5", Number, Op, Number);
scan_test!(lt_is_op, "4 < 5", Number, Op, Number);
scan_test!(lt_with_word_is_caret, "<ding>", LCaret, Word, RCaret);
scan_test!(lt_with_word_is_caret_2, "<ding >", LCaret, Word, RCaret);

////
// Tags

scan_test!(angle_is_op, "<", Op);
scan_test!(
    angle_followed_by_word_is_not_op,
    "<div>",
    LCaret,
    Word,
    RCaret
);
scan_test!(angle_id, "<#id>", LCaret, Op, Word, RCaret);
scan_test!(angle_class, "<.class>", LCaret, Op, Word, RCaret);
scan_test!(angle_type, "<:type>", LCaret, Colon, Word, RCaret);
scan_test!(angle_name, "<@name>", LCaret, Op, Word, RCaret);
scan_test!(
    angle_interpolated,
    "<div class=something-{2 + 2}-classy>",
    LCaret,
    Word,
    Word,
    Op,
    Word,
    RCaret
);
scan_test!(
    angle_js,
    "<span onclick=(alert('He he he!'))>Something</span>",
    LCaret,
    Word,
    Word,
    Op,
    JS,
    RCaret,
    Word,
    LCaret,
    Op,
    Word,
    RCaret,
);
scan_test!(
    angle_input_type,
    "<input:type>",
    LCaret,
    Word,
    Colon,
    Word,
    RCaret
);
scan_test!(
    angle_input_name,
    "<input@name>",
    LCaret,
    Word,
    Op,
    Word,
    RCaret
);

////
// Indents

#[rustfmt::skip]
scan_test!(
    basic_indent,
    r#"
if 2 > 1
    true
else
    false
"#,
    Word, Number, Op, Number, Indent, Word, Semi, Dedent, Word,
    Indent, Word, Semi, Dedent
);

#[rustfmt::skip]
scan_test!(
    less_basic_indent,
    r#"
def thing()
    if true
        other-thing()
    else if false
        if true-again?
            other-other-thing()
    else
        for x in loop
            print(x)
done()
"#,
    Word, Word, LParen, RParen,                             // def thing()
        Indent, Word, Word,                                 //   if true
            Indent, Word, LParen, RParen, Semi,             //     other-thing()
        Dedent, Word, Word, Word,                           //   else if false
            Indent, Word, Word,                             //     if true-again?
                Indent, Word, LParen, RParen, Semi,         //       other-other-thing()
        Dedent, Dedent, Word,                               //   else
            Indent, Word, Word, Word, Word,                 //     for x in loop
                Indent, Word, LParen, Word, RParen, Semi,   //       print(x)
    Dedent, Dedent, Dedent, Word, LParen, RParen            // done()
);

#[rustfmt::skip]
scan_test!(
    no_indent_for_operators,
    r#"
abc()
    .to-lowercase()
    .to-uppercase()
    + 20
    if true
        yay
"#,
    Word, LParen, RParen,
    Op, Word, LParen, RParen,
    Op, Word, LParen, RParen,
    Op, Number,
    Indent, Word, Word, Indent, Word, Semi,
    Dedent, Dedent,
);

scan_test!(
    tag_isnt_an_operator,
    r#"
if true
    <b> Something
"#,
    Word,
    Word,
    Indent,
    LCaret,
    Word,
    RCaret,
    Word,
    Semi,
    Dedent
);
