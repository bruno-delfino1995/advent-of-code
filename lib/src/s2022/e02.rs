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

mod parser {
	use super::*;
	use nom::{
		bytes::complete::tag,
		character::complete::one_of,
		combinator::{map, map_res},
		sequence::tuple,
		IResult,
	};

	fn parse_move(s: &str) -> IResult<&str, Move> {
		map_res(one_of("ABCXYZ"), |s| Move::from_str(&s.to_string()))(s)
	}

	fn parse_plan(s: &str) -> IResult<&str, Outcome> {
		map_res(one_of("XYZ"), |s| Outcome::from_str(&s.to_string()))(s)
	}

	pub(super) fn plain(s: &str) -> IResult<&str, Round> {
		map(
			tuple((parse_move, tag(" "), parse_move)),
			|(theirs, _, ours)| Round { theirs, ours },
		)(s)
	}

	pub(super) fn strategy(s: &str) -> IResult<&str, Round> {
		map(
			tuple((parse_move, tag(" "), parse_plan)),
			|(theirs, _, planned)| {
				let ours = theirs.move_to(&planned);

				Round { theirs, ours }
			},
		)(s)
	}
}

solution!("2022.2.1", basic(input) {
	lines(input)
		.map(|line| {
			let (_, round) = parser::plain(&line).expect("invalid round");

			round.score()
		})
		.sum::<usize>()
		.to_string()
});

solution!("2022.2.2", complex(input) {
	lines(input)
		.map(|line| {
			let (_, round) = parser::strategy(&line).expect("invalid round");

			round.score()
		})
		.sum::<usize>()
		.to_string()
});

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

		assert_eq!(basic::solution(input), "15");
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

		assert_eq!(complex::solution(input), "12");
	}
}
