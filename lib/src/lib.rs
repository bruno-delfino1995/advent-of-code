mod prelude;
mod puzzle;
mod s2022;

use std::{collections::HashMap, io::BufRead};

pub use self::puzzle::Puzzle;

pub type Input = Box<dyn BufRead>;
pub type Solution = dyn Fn(Input) -> String;
pub type Register = HashMap<Puzzle, &'static Solution>;
pub struct Solutions(Register);

impl Solutions {
	fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn load() -> Self {
		let mut solutions = Solutions::new();

		solutions.register(s2022::solutions());

		solutions
	}

	fn register(&mut self, mut solutions: Register) {
		for (k, v) in solutions.drain() {
			self.0.insert(k, v);
		}
	}

	pub fn run(&self, puzzle: &Puzzle, input: Box<dyn BufRead>) -> Result<String, String> {
		self.0
			.get(puzzle)
			.ok_or_else(|| String::from("Solution not found"))
			.map(|func| func(input))
	}
}

#[cfg(test)]
#[macro_export]
macro_rules! input {
	($contents:literal) => {{
		use indoc::indoc;
		use std::io::Cursor;

		let contents = indoc! { $contents };

		Box::new(Cursor::new(contents))
	}};
}
