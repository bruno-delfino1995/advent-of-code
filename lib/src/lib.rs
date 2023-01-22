mod common;
mod prelude;
mod puzzle;
mod s2022;
mod sample;

use linkme::distributed_slice;
use std::{collections::HashMap, io::BufRead};

pub use self::puzzle::Puzzle;

pub type Input = Box<dyn BufRead>;
pub type Solution = fn(Input) -> String;
pub type Register = HashMap<Puzzle, &'static Solution>;
pub struct Solutions(Register);

#[distributed_slice]
pub(crate) static SOLUTIONS: [(&str, fn(Input) -> String)] = [..];

impl Solutions {
	pub fn load() -> Self {
		let mut register = HashMap::new();

		for (puzzle, solution) in SOLUTIONS {
			let puzzle = Puzzle::from(*puzzle);

			if register.contains_key(&puzzle) {
				panic!("there can't be multiple solutions for the same puzzle");
			}

			register.insert(puzzle, solution);
		}

		Self(register)
	}

	pub fn run(&self, puzzle: &Puzzle, input: Box<dyn BufRead>) -> Result<String, String> {
		self.0
			.get(puzzle)
			.ok_or_else(|| String::from("Solution not found"))
			.map(|func| func(input))
	}
}

#[macro_export]
macro_rules! solution {
	($p:literal, $f:ident($i:ident) $b:tt ) => {
		mod $f {
			use linkme::distributed_slice;

			use super::*;
			use $crate::SOLUTIONS;

			#[distributed_slice(SOLUTIONS)]
			static SOLUTION: (&str, fn(Input) -> String) = ($p, solution);
			pub fn solution($i: Input) -> String {
				$b
			}
		}
	};
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
