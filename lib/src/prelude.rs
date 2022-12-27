pub use super::{Puzzle, Register, Solution, Solutions};
pub use std::io::BufRead;

pub fn lines(input: Box<dyn BufRead>) -> Box<dyn Iterator<Item = String>> {
	let iter = input.lines().map(|r| r.unwrap()).into_iter();

	Box::new(iter)
}
