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
	too_worried: bool,
}

impl Monkey {
	fn throw(&mut self, divisors_product: isize) -> Option<(usize, isize)> {
		let item = self.items.pop_front();

		item.map(|mut worry| {
			// As we only test divisibility, reducing a number to the modulus of a divisor's multiple
			// guarantees that the result will be divisible by the original divisor. If the monkeys
			// have the following divisors [3, 2, 5, 6] (product = 180) and the first monkey's current
			// item is 8675, the check will still be false whether we the raw number against or if we
			// use the modulus of it against 180 (8675 % 180 => 35 % 3 = 2 <=> 8675 % 3 = 2)
			worry %= divisors_product;

			let new = self.operation.call(worry);

			if self.too_worried {
				new
			} else {
				new / 3
			}
		})
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
		sequence::{delimited, preceded, terminated, tuple},
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

	fn parse_monkey(s: &str, too_worried: bool) -> IResult<&str, Monkey> {
		let (rem, _) = terminated(parse_index, newline)(s)?;
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
				too_worried,
			},
		))
	}

	pub(super) fn monkeys(s: &str, too_worried: bool) -> IResult<&str, Vec<Monkey>> {
		separated_list1(newline, |s| parse_monkey(s, too_worried))(s)
	}
}

#[derive(Debug)]
struct KeepAway {
	monkeys: Vec<RefCell<Monkey>>,
	divisors_product: isize,
}

impl KeepAway {
	fn new(monkeys: Vec<Monkey>) -> Self {
		let monkeys: Vec<RefCell<Monkey>> = monkeys.into_iter().map(RefCell::new).collect();
		let divisors_product = monkeys.iter().map(|m| m.borrow().divisor).product();

		Self {
			monkeys,
			divisors_product,
		}
	}

	fn round(&mut self) -> Inspections {
		let mut inspections = Inspections::new(self.monkeys.len());

		for (i, monkey) in self.monkeys.iter().enumerate() {
			while let Some((receiver, worry)) = monkey.borrow_mut().throw(self.divisors_product) {
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

fn run(mut input: Input, rounds: usize, too_worried: bool) -> Inspections {
	let contents = {
		let mut buf = String::new();
		input.read_to_string(&mut buf).unwrap();

		buf
	};

	let monkeys = all_consuming(|s| parser::monkeys(s, too_worried))(&contents)
		.finish()
		.unwrap()
		.1;

	let mut game = KeepAway::new(monkeys);
	std::iter::repeat(0)
		.take(rounds)
		.map(|_| game.round())
		.reduce(|acc, el| acc.merge(el))
		.unwrap()
}

pub fn basic(input: Input) -> String {
	let mut inspections = run(input, 20, false).take();
	inspections.sort_by(|a, b| b.cmp(a));

	let res = inspections[0] * inspections[1];

	res.to_string()
}

pub fn complex(input: Input) -> String {
	let mut inspections = run(input, 10_000, true).take();
	inspections.sort_by(|a, b| b.cmp(a));

	let res = inspections[0] * inspections[1];

	res.to_string()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	fn input() -> Input {
		input!(
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
		)
	}

	#[test]
	fn first_example() {
		assert_eq!(basic(input()), "10605")
	}

	#[test]
	fn second_example() {
		assert_eq!(complex(input()), "2713310158")
	}
}
