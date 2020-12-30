/* Mathematical operations we can perform.
 * To make things slightly easier on ourselves, and to leverage the strong
 * type system that Rust has, we have this enum derive some trait functionality. */
#[derive(Debug, Eq, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div
}

/* Tokens that we choose to accept. */
#[derive(Debug, Eq, PartialEq)]
enum Token {
    EOF,
    Number(i64),
    Operation(Op),
    LeftParen,
    RightParen
}

fn lex(input: &str) -> Vec<Token> {
/* struct representing malformed user input. */
#[derive(Debug, Eq, PartialEq)]
struct BadInput;

    todo!("Implement Lexing!");
}

fn main() {
    println!("Hello, world!");
}
