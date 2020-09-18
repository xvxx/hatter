use hatter::{scan, Syntax::*};

macro_rules! scan {
    ($code:expr) => {
        scan($code).unwrap()
    };
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
                    println!("Computed tokens:");
                    for (x, token) in nodes.iter().enumerate() {
                        let (bold, clear) = if x == i  { ("\x1b[1;91m", "\x1b[0m") } else { ("","") };
                        println!("  {}({:?}, {}){}", bold, token.kind, token.literal(), clear);
                    }
                }
                println!("        left=want, right=got");
                assert_eq!($kind, node.kind);
                #[allow(unused_assignments)]
                { i += 1; }
            )+
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
    Special(';'),
    Special(';'),
    Number,
    Special(';'),
);

////
// Strings

scan_test!(basic_string, r#""just a regular string""#, String(true));
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

scan_test!(bracket_is_op, "<", Op);
scan_test!(bracket_followed_by_word, "<div>", Op, Word, Op);

////
// Indents

scan_test!(
    basic_indent,
    r#"
if 2 > 1
    true
else
    false
"#,
    Special(';'),
    Word,
    Number,
    Op,
    Number,
    Semi,
    Indent,
    Word,
    Dedent,
    Word,
    Indent,
    Word,
    Dedent
);
