use crate::prelude::*;

enum Instruction {
	Noop,
	Add(isize),
}

struct Register(isize);

impl Register {
	fn new() -> Self {
		Self(1)
	}

	fn read(self, ins: Instruction) -> (Vec<isize>, Self) {
		match ins {
			Instruction::Noop => (vec![self.0], self),
			Instruction::Add(amount) => {
				let nvalue = self.0 + amount;
				let cycles = vec![self.0, self.0];

				(cycles, Self(nvalue))
			}
		}
	}

	fn close(self) -> Vec<isize> {
		vec![self.0]
	}
}

fn parse(input: &str) -> Instruction {
	if input.eq("noop") {
		return Instruction::Noop;
	}

	let amount = input.split(" ").last().and_then(|n| n.parse::<isize>().ok()).unwrap();
	Instruction::Add(amount)
}

pub fn basic(input: Input) -> String {
	let cycles: Box<dyn Iterator<Item = isize>> = Box::new(std::iter::empty());
	let reg = Register::new();

	let (iter, reg) =
		lines(input).map(|line| parse(&line)).fold((cycles, reg), |(cycles, reg), ins| {
			let (values, reg) = reg.read(ins);
			let iter = Box::new(cycles.chain(values.into_iter()));

			(iter, reg)
		});

	let mut iter = iter.chain(reg.close().into_iter()).enumerate().map(|(idx, value)| (idx + 1, value));
	let mut checks = std::iter::successors(Some(20), |n| Some(n + 40));
	let mut to_check = checks.next();
	let zipped =
		std::iter::from_fn(|| {
			let check = to_check.unwrap();
			match iter.next() {
				None => None,
				Some((idx, value)) if idx == check => {
					to_check = checks.next();

					Some((idx, value, true))
				},
				Some((idx, value)) => Some((idx, value, false)),
			}
		});

	zipped.filter(|(_, _, c)| *c)
		.map(|(idx, v, _)| v * (idx as isize))
		.take(6)
		.sum::<isize>()
		.to_string()
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
			addx 15
			addx -11
			addx 6
			addx -3
			addx 5
			addx -1
			addx -8
			addx 13
			addx 4
			noop
			addx -1
			addx 5
			addx -1
			addx 5
			addx -1
			addx 5
			addx -1
			addx 5
			addx -1
			addx -35
			addx 1
			addx 24
			addx -19
			addx 1
			addx 16
			addx -11
			noop
			noop
			addx 21
			addx -15
			noop
			noop
			addx -3
			addx 9
			addx 1
			addx -3
			addx 8
			addx 1
			addx 5
			noop
			noop
			noop
			noop
			noop
			addx -36
			noop
			addx 1
			addx 7
			noop
			noop
			noop
			addx 2
			addx 6
			noop
			noop
			noop
			noop
			noop
			addx 1
			noop
			noop
			addx 7
			addx 1
			noop
			addx -13
			addx 13
			addx 7
			noop
			addx 1
			addx -33
			noop
			noop
			noop
			addx 2
			noop
			noop
			noop
			addx 8
			noop
			addx -1
			addx 2
			addx 1
			noop
			addx 17
			addx -9
			addx 1
			addx 1
			addx -3
			addx 11
			noop
			noop
			addx 1
			noop
			addx 1
			noop
			noop
			addx -13
			addx -19
			addx 1
			addx 3
			addx 26
			addx -30
			addx 12
			addx -1
			addx 3
			addx 1
			noop
			noop
			noop
			addx -9
			addx 18
			addx 1
			addx 2
			noop
			noop
			addx 9
			noop
			noop
			noop
			addx -1
			addx 2
			addx -37
			addx 1
			addx 3
			noop
			addx 15
			addx -21
			addx 22
			addx -6
			addx 1
			noop
			addx 2
			addx 1
			noop
			addx -10
			noop
			noop
			addx 20
			addx 1
			addx 2
			addx 2
			addx -6
			addx -11
			noop
			noop
			noop
		"#
		);

		assert_eq!(basic(input), "13140")
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
