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

/* struct representing malformed user input. */
#[derive(Debug, Eq, PartialEq)]
struct BadInput;

/* If we receive BadInput, then we need a way to display the output as an error
 * message using std::fmt::Display. */
impl std::fmt::Display for BadInput {
    /* fmt takes a reference to BadInput and a mutable reference to the formatter
     * and write!s the string to the formatter. */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	/* No semicolon needed, because it's a one-line function (an expression). */
	write!(f, "Invalid input. Check again.")
    }
}

/* Bad input can use the default stdlib implementation. */
impl std::error::Error for BadInput {}

fn lex(input: &str) -> Result<Vec<Token>, BadInput> {
    let mut result: Vec<Token> = Vec::new();

    for char in input.chars() {
	/* Bring the Op and Token enum variants into this scope's namespace. */
	use Op::*;
	use Token::*;

	/* Match on the various possible tokens we can be given and that we choose
	 * to accept. */
	match char {
	}
    }

    Ok(result);
}

fn main() {
    println!("Hello, world!");
}
