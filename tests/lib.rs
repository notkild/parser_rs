extern crate parser_rs;

use parser_rs::Parser;

#[test]
fn consume_char() {
    let a = "abc";
    let mut parser = Parser::new(a);
    assert_eq!(parser.consume(), Some('a'));
    assert_eq!(parser.consume(), Some('b'));
    assert_eq!(parser.consume(), Some('c'));
    assert_eq!(parser.consume(), None);
}

#[test]
fn consume_while() {
    let s = "hello world";
    let mut parser = Parser::new(s);
    assert_eq!(parser.consume_while(|c| c != ' '), "hello");
}

#[test]
fn consume_until() {
    let s = "hello world from another world";
    let mut parser = Parser::new(s);
    assert_eq!(parser.consume_until('r'), "hello wo");
    parser.consume();
    assert_eq!(parser.consume_until('r'), "ld f");
}

#[test]
fn consume_until_str() {
    let s = "hello world from another world";
    let mut parser = Parser::new(s);
    assert_eq!(parser.consume_until_str("another"), "hello world from ");
}

#[test]
fn consume_until_strs() {
    let s = "hello world from another world";
    let mut parser = Parser::new(s);
    assert_eq!(parser.consume_until_strs(&["world", "another"]), "hello ");
    parser.consume_until(' ');
    assert_eq!(parser.consume_until_strs(&["world", "another"]), " from ");
}

#[test]
fn consume_until_end() {
    let s = "hello world";
    let mut parser = Parser::new(s);
    parser.consume();
    assert_eq!(parser.consume_until_end(), "ello world");
}

#[test]
fn consume_inside() {
    let s = "hello world \"from another world\"";
    let mut parser = Parser::new(s);
    assert_eq!(parser.consume_until('\"'), "hello world ");
    assert_eq!(parser.consume_inside('\"'), "from another world");

    let s = "\"from another world";
    let mut parser = Parser::new(s);
    assert_eq!(parser.consume_inside('\"'), "from another world");
}
