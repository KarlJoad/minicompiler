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

    for character in input.chars() {
	/* Bring the Op and Token enum variants into this scope's namespace. */
	use Op::*;
	use Token::*;

	/* Match on the various possible tokens we can be given and that we choose
	 * to accept. */
	match character {
	    // Skip any whitespace
	    ' ' => continue,

	    // Terminating characters for input. Stop lexing here.
	    ';' | '\n' => {
		result.push(EOF);
		break;
	    }

	    // Math Operations
	    '+' => result.push(Operation(Add)),
	    '-' => result.push(Operation(Sub)),
	    '*' => result.push(Operation(Mul)),
	    '/' => result.push(Operation(Div)),

	    // Parentheses
	    '(' => result.push(LeftParen),
	    ')' => result.push(RightParen),

	    // Numbers
	    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
		/* We take the number character, convert it to an unsigned byte,
		 * then subtract the upper half-byte 3, and convert resulting byte
		 * back to a number. */
		let num: i64 = (character as u8 - '0' as u8) as i64;

		/* Number parsing is done with a stack-based manipulation system. */

		/* If result is empty, push this number to result and continue
		 * lexing the input. */
		if result.len() == 0 {
		    result.push(Number(num));
		    continue;
		}

		/* If we receive another number, pop the last item in result and
		 * save it as last. */
		let last = result.pop().unwrap();
		/* pop() -> Option<T>. Use unwrap() to get value inside Option. */

		match last {
		    /* If last is a number, multiply that number by 10 and add the
		     * current number to it. */
		    Number(i) => {
			result.push(Number((i * 10) + num));
		    }
		    /* Otherwise push the node back into result and push the current
		     * number to result as well. */
		    _ => {
			result.push(last);
			result.push(Number(num));
		    }
		}
	    }

	    // Anything and everything else is malformed input.
	    _ => return Err(BadInput),
	}
    }

    Ok(result)
}

fn main() {
    println!("Hello, world!");
}
