extern crate parser_rs;

use parser_rs::Parser;

#[test]
fn consume_char() {
    let a = "abc";
    let mut parser = Parser::new(a);
    assert_eq!(parser.consume(), 'a');
    assert_eq!(parser.consume(), 'b');
    assert_eq!(parser.consume(), 'c');
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
