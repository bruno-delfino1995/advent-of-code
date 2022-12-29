use std::{
	cmp::Ordering::{self, *},
	str::FromStr,
};

use crate::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
enum Move {
	Paper = 2,
	Rock = 1,
	Scissors = 3,
}

impl Ord for Move {
	fn cmp(&self, other: &Self) -> Ordering {
		use Move::*;

		match (self, other) {
			(Paper, Paper) => Equal,
			(Paper, Rock) => Greater,
			(Paper, Scissors) => Less,
			(Rock, Paper) => Less,
			(Rock, Rock) => Equal,
			(Rock, Scissors) => Greater,
			(Scissors, Paper) => Greater,
			(Scissors, Rock) => Less,
			(Scissors, Scissors) => Equal,
		}
	}
}

impl PartialOrd for Move {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl FromStr for Move {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Move::*;

		match s {
			"A" | "X" => Ok(Rock),
			"B" | "Y" => Ok(Paper),
			"C" | "Z" => Ok(Scissors),
			_ => Err(String::from("invalid")),
		}
	}
}

struct Round {
	theirs: Move,
	ours: Move,
}

impl Round {
	fn score(&self) -> u8 {
		let outcome: Outcome = self.into();
		let shape = &self.ours;

		outcome as u8 + *shape as u8
	}
}

#[repr(u8)]
enum Outcome {
	Win = 6,
	Draw = 3,
	Lose = 0,
}

impl From<&Round> for Outcome {
	fn from(value: &Round) -> Self {
		use Outcome::*;

		match value.ours.cmp(&value.theirs) {
			Less => Lose,
			Equal => Draw,
			Greater => Win,
		}
	}
}

peg::parser! {
	grammar parser() for str {
		rule theirs() -> Move
			= n:$("A" / "B" / "C") {? Move::from_str(n).map_err(|_| "invalid")}

		rule ours() -> Move
			= n:$("X" / "Y" / "Z") {? Move::from_str(n).map_err(|_| "invalid")}

		pub(super) rule parse() -> Round
			=	t:theirs() " " o:ours() { Round { theirs: t, ours: o } }
	}
}

pub fn basic(input: Input) -> String {
	lines(input)
		.map(|line| {
			let round = parser::parse(&line).expect("invalid round");

			round.score() as usize
		})
		.sum::<usize>()
		.to_string()
}

pub fn complex(input: Input) -> String {
	String::from("unimplemented")
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			A Y
			B X
			C Z
		"#
		);

		assert_eq!(basic(input), "15");
	}
}
