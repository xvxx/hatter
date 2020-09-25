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
scan_test!(number_range, "1..101", Number, Op, Number);
scan_test!(number_range_inclusive, "500..=600", Number, Op, Number);
scan_test!(number_range_spaced, "500 ..= 600", Number, Op, Number);

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
scan_test!(escaped_single_quote_string, "'what\\'s up'", String(false));
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

#[test]
fn empty_string_test() {
    let lits = vec![
        r#""""#,
        r#"''"#,
        r#"``"#,
        r#""""""""#,
        r#"''''''"#,
        r#"``````"#,
    ];
    for lit in lits {
        assert_eq!("", scan!(lit)[0].literal());
    }
}

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
scan_test!(or_is_a_word, "or", Word);
scan_test!(not_is_a_word, "not", Word);
scan_test!(return_is_a_keyword, "return", Return);
scan_test!(break_is_a_word, "break", Word);
scan_test!(continue_is_a_word, "continue", Word);

////
// Ops

scan_test!(simple_op, "+", Op);
scan_test!(lonley_dash_is_an_op, "-", Op);
scan_test!(two_char_op, ":=", Op);
scan_test!(two_char_op2, "==", Op);
scan_test!(two_char_op3, "!=", Op);
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
scan_test!(slash_is_an_op, "23 / 24", Number, Op, Number);
scan_test!(but_not_in_a_tag, "<cat/>", LCaret, Word, Slash, RCaret);
scan_test!(or_a_close_tag, "</>", LCaret, Slash, RCaret);
scan_test!(or_a_closing_tag, "</div>", LCaret, Slash, Word, RCaret);

////
// Tags

scan_test!(angle_is_op, "<", Op);
scan_test!(angle_id, "<#id>", LCaret, Op, Word, RCaret);
scan_test!(angle_class, "<.class>", LCaret, Op, Word, RCaret);
scan_test!(angle_type, "<:type>", LCaret, Op, Word, RCaret);
scan_test!(angle_name, "<@name>", LCaret, Op, Word, RCaret);

#[rustfmt::skip]
mod skipfmt {
    use super::*;

    scan_test!(angle_followed_by_word_is_not_op,
        "<div>",
        LCaret, Word, RCaret
    );

    scan_test!(angle_interpolated,
        "<div class=something-{2 + 2}-classy>",
        LCaret, Word, Word, Equal, Word, RCaret
    );

    scan_test!(angle_interpolated_id_full,
        "<div#{cool}>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_interpolated_id_partial,
        "<div#page-{id}>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_interpolated_class_full,
        "<div.{cool}>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_interpolated_class_partial,
        "<div.page-{id}>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_interpolated_type_full,
        "<input:{cool}>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_interpolated_type_partial,
        "<input:page-{id}>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_interpolated_name_full,
        "<input@{cool}>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_interpolated_name_partial,
        "<input@page-{id}>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_interpolated_attr_name_partial,
        "<div my-{name}-name=true/>",
        LCaret, Word, Word, Equal, Word, Slash, RCaret
    );

    scan_test!(angle_interpolated_attr_name_full,
        "<div {name}=true/>",
        LCaret, Word, Word, Equal, Word, Slash, RCaret
    );

    scan_test!(angle_interpolated_attr_value_partial,
        "<div data-name=some-{thing}/>",
        LCaret, Word, Word, Equal, Word, Slash, RCaret
    );

    scan_test!(angle_interpolated_attr_value_full,
        "<div data-name={true}/>",
        LCaret, Word, Word, Equal, Word, Slash, RCaret
    );

    scan_test!(angle_js,
        "<span onclick=(alert('He he he!'))>Something</span>",
        LCaret, Word, Word, Equal, JS, RCaret, Word, LCaret,
        Slash, Word, RCaret,
    );

    scan_test!(angle_input_type,
        "<input:type>",
        LCaret, Word, Op, Word, RCaret
    );

    scan_test!(angle_input_name,
        "<input@name>",
        LCaret, Word, Op, Word, RCaret
    );
}

////
// Tag

scan_test!(basic_tag, "<b> Hey there", LCaret, Word, RCaret, Word, Word);

#[rustfmt::skip]
mod skipfmt_tag {
    use super::*;

    scan_test!(
        self_closing_tag,
        "<end/> ok",
        LCaret, Word, Slash, RCaret, Word
    );

    scan_test!(
        nested_tag,
        "<b> Hey <i> there",
        LCaret, Word, RCaret, Word, LCaret, Word, RCaret, Word
    );

    scan_test!(
        close_shortcut,
        "<b> Hey <i> there </> fren ",
        LCaret,
        Word, RCaret, Word, LCaret, Word, RCaret, Word, LCaret, Slash, RCaret, Word
    );

    scan_test!(
        basic_id_shortcut,
        "<b#shout> Hey yo",
        LCaret, Word, Op, Word, RCaret, Word, Word
    );

    scan_test!(
        basic_class_shortcut,
        "<span.clear> Welcome ",
        LCaret, Word, Op, Word, RCaret, Word
    );

    scan_test!(
        basic_classes,
        "<div.main.markdown> Yada yada... ",
        LCaret, Word, Op, Word, Op, Word, RCaret, Word, Word, Op
    );

    scan_test!(
        basic_name_shortcut,
        "<input@text/>",
        LCaret, Word, Op, Word, Slash, RCaret
    );

    scan_test!(
        basic_type_shortcut,
        "<input:submit/> ",
        LCaret, Word, Op, Word, Slash, RCaret
    );

    scan_test!(
        mixed_shortcuts,
        "<div#main.markdown/> ",
        LCaret, Word, Op, Word, Op, Word, Slash, RCaret
    );

    scan_test!(
        mixed_input_shortcuts,
        "<input#focused:text@search  /> ",
        LCaret, Word, Op, Word, Op, Word, Op, Word, Slash, RCaret
    );

    scan_test!(
        basic_attribute,
        r#"<div data-id="45">"#,
        LCaret, Word, Word, Equal, String(false), RCaret
    );

    scan_test!(
        basic_attributes,
        r#"<input name="thing" placeholder="Other..."/>"#,
        LCaret, Word, Word, Equal, String(false), Word, Equal,
        String(false), Slash, RCaret,
    );

    scan_test!(
        js_attributes,
        "<div onclick=(alert('lol'))>Click me",
        LCaret, Word, Word, Equal, JS, RCaret, Word, Word
    );

    scan_test!(
        simple_code_attributes,
        "<div data-id=page.id>",
        LCaret, Word, Word, Equal, Word, Op, Word, RCaret
    );

    scan_test!(
        shorthand_conditionals,
        "<div#id=has-id>",
        LCaret, Word, Op, Word, Equal, Word, RCaret,
    );

    scan_test!(
        code_expr_attributes,
        "<div data-value={2 + 3}>",
        LCaret, Word, Word, Equal, Word, RCaret
    );

    scan_test!(
        tag_everything,
        "<div#id.class1.class-2=is-it? :why-not @sure onclick=(alert(`it's ${2 + 2}`)) data-id=123 data-{value}=compute(value) />",
        LCaret, // <
        Word,   // div
        Op,     // #
        Word,   // id
        Op,     // .
        Word,   // class1
        Op,     // .
        Word,   // class-2
        Equal,  // =
        Word,   // is-it?
        Op,     // :
        Word,   // why-not
        Op,     // @
        Word,   // sure
        Word,   // onclick
        Equal,  // =
        JS,     // js
        Word,   // data-id
        Equal,  // =
        Number, // 123
        Word,   // data-{value}
        Equal,  // =
        Word,   // compute
        LParen, // (
        Word,   // value
        RParen, // )
        Slash,  // /
        RCaret // >
    );
}

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
    If, Number, Op, Number, Indent, Word, Semi, Dedent, Else,
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
    Def, Word, LParen, RParen,                             // def thing()
        Indent, If, Word,                                 //   if true
            Indent, Word, LParen, RParen, Semi,             //     other-thing()
        Dedent, Else, If, Word,                           //   else if false
            Indent, If, Word,                             //     if true-again?
                Indent, Word, LParen, RParen, Semi,         //       other-other-thing()
        Dedent, Dedent, Else,                               //   else
            Indent, For, Word, In, Word,                 //     for x in loop
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
    Indent, If, Word, Indent, Word, Semi,
    Dedent, Dedent,
);

scan_test!(
    tag_isnt_an_operator,
    r#"
if true
    <b> Something
"#,
    If,
    Word,
    Indent,
    LCaret,
    Word,
    RCaret,
    Word,
    Semi,
    Dedent
);

#[rustfmt::skip]
scan_test!(
    indented_comments,
    r#"
if true
    # Something
    "ok"
# ok?
else
    "something else"
        # nice..

# all done
"all done"
                # yup
"yup"
"#,
    If, Word,
        Indent, String(false), Semi,
    Dedent, Else,
        Indent, String(false), Semi,
    Dedent, String(false), Semi, String(false)
);

#[rustfmt::skip]
mod skipfmt_indent {
use super::*;

    scan_test!(
        ignore_whitespace_in_parens,
        r#"
add(
    1,
    2,    3
)
"#,
        Word, LParen, Semi, Number, Comma, Semi, Number, Comma, Number, Semi, RParen,
    );

    scan_test!(
        newlines_are_commas_in_fn_calls,
        r#"
add(
    1
    2,    3
    4
)
"#,
        Word, LParen, Semi, Number, Semi, Number, Comma, Number, Semi, Number, Semi, RParen,
    );

    scan_test!(
        comments_dont_eat_indents,
        r#"
if true # this should work
    print("Told ya")
"#,
        If, Word, Indent, Word, LParen, String(false), RParen, Semi, Dedent,
    );

    scan_test!(basic_indented_tag, r#"
<ul>
    <li> Kewl
    <li> Very!
"#,
    LCaret, Word, RCaret,
        Indent, LCaret, Word, RCaret, Word, Semi,
        LCaret, Word, RCaret, Word, Op, Semi,
    Dedent
    );

    scan_test!(basic_nested_tag, r#"
<main> <ul>
    <li> Kewl <b> beans
    <li> Very!
"#,
        LCaret, Word, RCaret,
        LCaret, Word, RCaret,
            Indent, LCaret, Word, RCaret, Word, LCaret, Word, RCaret, Word, Semi,
            LCaret, Word, RCaret, Word, Op, Semi,
        Dedent
    );

    scan_test!(nested_tag_and_code, r#"
<main> <ul> for bean in beans
    <li> Kewl <b> bean
    <li> Very: <b> if 2 > 1
        <em> Nuty!
"#,
        LCaret, Word, RCaret,
        LCaret, Word, RCaret,
        For, Word, In, Word,
            Indent, LCaret, Word, RCaret, Word, LCaret, Word, RCaret, Word, Semi,
            LCaret, Word, RCaret, Word, Colon, LCaret, Word, RCaret, If, Number, Op, Number,
                Indent, LCaret, Word, RCaret, Word, Op, Semi,
            Dedent,
        Dedent
    );

}
