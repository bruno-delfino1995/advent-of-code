pub use crate::{Input, Puzzle, Register, Solution, Solutions};
pub use std::io::BufRead;

pub fn lines(input: Input) -> Box<dyn Iterator<Item = String>> {
	let iter = input.lines().map(|r| r.unwrap());

	Box::new(iter)
}
