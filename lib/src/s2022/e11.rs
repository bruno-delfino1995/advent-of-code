use std::{cell::RefCell, collections::VecDeque};

use nom::{combinator::all_consuming, Finish};

use crate::prelude::*;

#[derive(Debug)]
enum Op {
	Mul(Term),
	Add(Term),
}

impl Op {
	fn call(&self, o: isize) -> isize {
		match self {
			Op::Mul(t) => o * t.eval(o),
			Op::Add(t) => o + t.eval(o),
		}
	}
}

#[derive(Debug)]
enum Term {
	Curr,
	Number(isize),
}

impl Term {
	fn eval(&self, o: isize) -> isize {
		match self {
			Term::Curr => o,
			Term::Number(n) => *n,
		}
	}
}

#[derive(Debug)]
struct Monkey {
	items: VecDeque<isize>,
	operation: Op,
	divisor: isize,
	on_true: usize,
	on_false: usize,
}

impl Monkey {
	fn throw(&mut self) -> Option<(usize, isize)> {
		let item = self.items.pop_front();

		item.map(|worry| self.operation.call(worry) / 3)
			.map(|worry| {
				let divisible = worry % self.divisor == 0;

				if divisible {
					(self.on_true, worry)
				} else {
					(self.on_false, worry)
				}
			})
	}

	fn receive(&mut self, item: isize) {
		self.items.push_back(item)
	}
}

mod parser {
	use crate::common::parse::{isize, usize};

	use super::*;
	use nom::{
		branch::alt,
		bytes::complete::tag,
		character::complete::{newline, one_of, space1},
		combinator::map,
		multi::{separated_list0, separated_list1},
		sequence::{delimited, preceded, tuple},
		IResult,
	};

	fn parse_index(s: &str) -> IResult<&str, usize> {
		delimited(tag("Monkey "), usize, tag(":"))(s)
	}

	fn parse_items(s: &str) -> IResult<&str, VecDeque<isize>> {
		map(
			preceded(tag("Starting items: "), separated_list0(tag(", "), isize)),
			VecDeque::from,
		)(s)
	}

	fn parse_operation(s: &str) -> IResult<&str, Op> {
		let parse_term = alt((map(tag("old"), |_| Term::Curr), map(isize, Term::Number)));

		let (rem, (op, t)) = preceded(
			tag("Operation: new = old"),
			tuple((preceded(space1, one_of("*+")), preceded(space1, parse_term))),
		)(s)?;

		let op = match op {
			'*' => Op::Mul(t),
			'+' => Op::Add(t),
			_ => unreachable!(),
		};

		Ok((rem, op))
	}

	fn parse_divisor(s: &str) -> IResult<&str, isize> {
		preceded(tag("Test: divisible by "), isize)(s)
	}

	fn parse_on_true(s: &str) -> IResult<&str, usize> {
		preceded(tag("If true: throw to monkey "), usize)(s)
	}

	fn parse_on_false(s: &str) -> IResult<&str, usize> {
		preceded(tag("If false: throw to monkey "), usize)(s)
	}

	fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
		let (rem, (_, _)) = tuple((parse_index, newline))(s)?;
		let (rem, items) = delimited(space1, parse_items, newline)(rem)?;
		let (rem, operation) = delimited(space1, parse_operation, newline)(rem)?;
		let (rem, divisor) = delimited(space1, parse_divisor, newline)(rem)?;
		let (rem, on_true) = delimited(space1, parse_on_true, newline)(rem)?;
		let (rem, on_false) = delimited(space1, parse_on_false, newline)(rem)?;

		Ok((
			rem,
			Monkey {
				items,
				operation,
				divisor,
				on_true,
				on_false,
			},
		))
	}

	pub(super) fn monkeys(s: &str) -> IResult<&str, Vec<Monkey>> {
		separated_list1(tag("\n"), parse_monkey)(s)
	}
}

#[derive(Debug)]
struct KeepAway {
	monkeys: Vec<RefCell<Monkey>>,
}

impl KeepAway {
	fn new(monkeys: Vec<Monkey>) -> Self {
		let monkeys = monkeys.into_iter().map(RefCell::new).collect();

		Self { monkeys }
	}

	fn round(&mut self) -> Inspections {
		let mut inspections = Inspections::new(self.monkeys.len());

		for (i, monkey) in self.monkeys.iter().enumerate() {
			while let Some((receiver, worry)) = monkey.borrow_mut().throw() {
				inspections.record(i);

				self.monkeys
					.get(receiver)
					.unwrap()
					.borrow_mut()
					.receive(worry)
			}
		}

		inspections
	}
}

#[derive(Debug)]
struct Inspections(Vec<usize>);

impl Inspections {
	fn new(len: usize) -> Self {
		Self(vec![0; len])
	}

	fn record(&mut self, idx: usize) {
		self.0[idx] += 1;
	}

	fn merge(self, other: Self) -> Self {
		let inspections = self
			.0
			.into_iter()
			.zip(other.0.into_iter())
			.map(|(a, b)| a + b)
			.collect();

		Self(inspections)
	}

	fn take(self) -> Vec<usize> {
		self.0
	}
}

pub fn basic(mut input: Input) -> String {
	let contents = {
		let mut buf = String::new();
		input.read_to_string(&mut buf).unwrap();

		buf
	};

	let monkeys = all_consuming(parser::monkeys)(&contents)
		.finish()
		.unwrap()
		.1;

	let mut game = KeepAway::new(monkeys);
	let mut inspections = std::iter::repeat(0)
		.take(20)
		.map(|_| game.round())
		.reduce(|acc, el| acc.merge(el))
		.unwrap()
		.take();
	inspections.sort_by(|a, b| b.cmp(a));

	let res = inspections[0] * inspections[1];

	res.to_string()
}

pub fn complex(_input: Input) -> String {
	todo!()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			Monkey 0:
				Starting items: 79, 98
				Operation: new = old * 19
				Test: divisible by 23
					If true: throw to monkey 2
					If false: throw to monkey 3

			Monkey 1:
				Starting items: 54, 65, 75, 74
				Operation: new = old + 6
				Test: divisible by 19
					If true: throw to monkey 2
					If false: throw to monkey 0

			Monkey 2:
				Starting items: 79, 60, 97
				Operation: new = old * old
				Test: divisible by 13
					If true: throw to monkey 1
					If false: throw to monkey 3

			Monkey 3:
				Starting items: 74
				Operation: new = old + 3
				Test: divisible by 17
					If true: throw to monkey 0
					If false: throw to monkey 1
		"#
		);

		assert_eq!(basic(input), "10605")
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			3
		"#
		);

		assert_eq!(complex(input), "2")
	}
}
