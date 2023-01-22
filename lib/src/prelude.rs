pub use crate::{solution, Input, Puzzle};
pub use std::io::BufRead;

pub fn lines(input: Input) -> Box<dyn Iterator<Item = String>> {
	let iter = input.lines().map(|r| r.unwrap());

	Box::new(iter)
}

pub fn all(mut input: Input) -> String {
	let mut buf = String::new();
	input.read_to_string(&mut buf).unwrap();

	buf
}
