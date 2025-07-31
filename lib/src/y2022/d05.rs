use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};
use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Debug)]
struct Container(char);

#[derive(Debug)]
struct Command {
	amount: usize,
	from: usize,
	to: usize,
}

#[derive(Debug)]
struct Piles(Vec<VecDeque<Container>>);

impl Piles {
	fn apply(&mut self, command: Command) {
		let from = self.0.get_mut(command.from).unwrap();
		let elems = from.drain(0..command.amount).collect_vec();
		let to = self.0.get_mut(command.to).unwrap();

		for el in elems {
			to.push_front(el);
		}
	}

	fn apply_retain(&mut self, command: Command) {
		let from = self.0.get_mut(command.from).unwrap();
		let mut elems = from.drain(0..command.amount).collect_vec();
		elems.reverse();
		let to = self.0.get_mut(command.to).unwrap();

		for el in elems {
			to.push_front(el);
		}
	}

	fn heads(&self) -> Vec<char> {
		self.0
			.iter()
			.filter_map(|pile| pile.front())
			.map(|c| c.0)
			.collect()
	}
}

mod parser {
	use super::*;
	use nom::{
		branch::alt,
		bytes::complete::{tag, take},
		character::complete::digit1,
		combinator::{map, map_res},
		multi::separated_list0,
		sequence::{delimited, preceded, tuple},
		IResult,
	};

	fn parse_slot(s: &str) -> IResult<&str, Option<Container>> {
		let container = map(delimited(tag("["), take(1usize), tag("]")), |s: &str| {
			Some(Container(s.chars().next().unwrap()))
		});

		let space = map(tag("   "), |_| None);

		alt((container, space))(s)
	}

	pub(super) fn ship(s: &str) -> IResult<&str, Vec<Option<Container>>> {
		separated_list0(tag(" "), parse_slot)(s)
	}

	fn parse_number(s: &str) -> IResult<&str, usize> {
		map_res(digit1, str::parse)(s)
	}

	fn parse_column(s: &str) -> IResult<&str, usize> {
		map(parse_number, |n| n - 1)(s)
	}

	pub(super) fn command(s: &str) -> IResult<&str, Command> {
		let amount = preceded(tag("move "), parse_number);
		let from = preceded(tag(" from "), parse_column);
		let to = preceded(tag(" to "), parse_column);

		map(tuple((amount, from, to)), |(amount, from, to)| Command {
			amount,
			from,
			to,
		})(s)
	}
}

fn transpose<T>(v: Vec<Vec<Option<T>>>) -> Vec<VecDeque<T>> {
	assert!(!v.is_empty());
	let len = v[0].len();
	let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
	(0..len)
		.map(|_| {
			iters
				.iter_mut()
				.filter_map(|n| n.next().unwrap())
				.collect::<VecDeque<T>>()
		})
		.collect()
}

fn parse(input: Input) -> (Piles, Vec<Command>) {
	let mut lines = lines(input);

	let crates = (&mut lines)
		.map_while(|l| {
			all_consuming(parser::ship)(&l)
				.finish()
				.ok()
				.map(|(_, v)| v)
		})
		.collect_vec();
	let piles = Piles(transpose(crates));

	assert!(lines.next().is_some());

	let commands = lines
		.map(|l| parser::command(&l).ok().map(|(_, c)| c).unwrap())
		.collect_vec();

	(piles, commands)
}

solution!("y2022d05p1", basic(input) {
	let (mut piles, commands) = parse(input);

	for c in commands {
		piles.apply(c);
	}

	piles.heads().into_iter().collect()
});

solution!("y2022d05p2", complex(input) {
	let (mut piles, commands) = parse(input);

	for c in commands {
		piles.apply_retain(c);
	}

	piles.heads().into_iter().collect()
});

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			    [D]
			[N] [C]
			[Z] [M] [P]
			 1   2   3

			move 1 from 2 to 1
			move 3 from 1 to 3
			move 2 from 2 to 1
			move 1 from 1 to 2
		"#
		);

		assert_eq!(basic::solution(input), "CMZ");
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			    [D]
			[N] [C]
			[Z] [M] [P]
			 1   2   3

			move 1 from 2 to 1
			move 3 from 1 to 3
			move 2 from 2 to 1
			move 1 from 1 to 2
		"#
		);

		assert_eq!(complex::solution(input), "MCD");
	}
}
