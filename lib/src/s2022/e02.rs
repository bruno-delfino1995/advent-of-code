use std::{
	cmp::Ordering::{self, *},
	str::FromStr,
};

use crate::prelude::*;

#[derive(Eq, PartialEq)]
enum Move {
	Paper = 2,
	Rock = 1,
	Scissors = 3,
}

impl Move {
	fn beats(&self, other: &Move) -> bool {
		use Move::*;

		matches!(
			(self, other),
			(Paper, Rock) | (Rock, Scissors) | (Scissors, Paper)
		)
	}

	fn move_to(&self, planned: &Outcome) -> Move {
		use Move::*;
		use Outcome::*;

		let mut moves = [Paper, Rock, Scissors].into_iter();

		match planned {
			Win => moves.find(|m| m > self),
			Lose => moves.find(|m| m < self),
			Draw => moves.find(|m| m == self),
		}
		.unwrap()
	}

	fn score(&self) -> usize {
		use Move::*;

		match self {
			Paper => 2,
			Rock => 1,
			Scissors => 3,
		}
	}
}

impl Ord for Move {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.beats(other) {
			Greater
		} else if other.beats(self) {
			Less
		} else {
			Equal
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
	fn outcome(&self) -> Outcome {
		use Outcome::*;

		match self.ours.cmp(&self.theirs) {
			Less => Lose,
			Equal => Draw,
			Greater => Win,
		}
	}

	fn score(&self) -> usize {
		let outcome = self.outcome();
		let shape = &self.ours;

		outcome.score() + shape.score()
	}
}

enum Outcome {
	Win,
	Draw,
	Lose,
}

impl Outcome {
	fn score(&self) -> usize {
		use Outcome::*;

		match self {
			Win => 6,
			Draw => 3,
			Lose => 0,
		}
	}
}

impl FromStr for Outcome {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Outcome::*;

		match s {
			"X" => Ok(Lose),
			"Y" => Ok(Draw),
			"Z" => Ok(Win),
			_ => Err(String::from("Invalid")),
		}
	}
}

peg::parser! {
	grammar parser() for str {
		rule theirs() -> Move
			= n:$("A" / "B" / "C") {? Move::from_str(n).map_err(|_| "invalid") }

		rule ours() -> Move
			= n:$("X" / "Y" / "Z") {? Move::from_str(n).map_err(|_| "invalid") }

		rule plan() -> Outcome
			= n:$("X" / "Y" / "Z") {? Outcome::from_str(n).map_err(|_| "invalid") }

		pub(super) rule plain() -> Round
			=	t:theirs() " " o:ours() { Round { theirs: t, ours: o } }

		pub(super) rule strategy() -> Round
			=	t:theirs() " " p:plan() {
				let o = t.move_to(&p);

				Round { theirs: t, ours: o }
			}
	}
}

pub fn basic(input: Input) -> String {
	lines(input)
		.map(|line| {
			let round = parser::plain(&line).expect("invalid round");

			round.score()
		})
		.sum::<usize>()
		.to_string()
}

pub fn complex(input: Input) -> String {
	lines(input)
		.map(|line| {
			let round = parser::strategy(&line).expect("invalid round");

			round.score()
		})
		.sum::<usize>()
		.to_string()
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

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			A Y
			B X
			C Z
		"#
		);

		assert_eq!(complex(input), "12");
	}
}