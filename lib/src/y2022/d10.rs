use itertools::Itertools;

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
}

fn parse(input: &str) -> Instruction {
	if input.eq("noop") {
		return Instruction::Noop;
	}

	let amount = input
		.split(' ')
		.last()
		.and_then(|n| n.parse::<isize>().ok())
		.unwrap();
	Instruction::Add(amount)
}

solution!("y2022d10p1", basic(input) {
	let cycles: Box<dyn Iterator<Item = isize>> = Box::new(std::iter::empty());
	let reg = Register::new();

	let (iter, _) =
		lines(input)
			.map(|line| parse(&line))
			.fold((cycles, reg), |(cycles, reg), ins| {
				let (values, reg) = reg.read(ins);
				let iter = Box::new(cycles.chain(values.into_iter()));

				(iter, reg)
			});

	let mut iter = iter.enumerate().map(|(idx, value)| (idx + 1, value));
	let mut checks = std::iter::successors(Some(20), |n| Some(n + 40));
	let mut to_check = checks.next();
	let zipped = std::iter::from_fn(|| {
		let check = to_check.unwrap();
		match iter.next() {
			None => None,
			Some((idx, value)) if idx == check => {
				to_check = checks.next();

				Some((idx, value, true))
			}
			Some((idx, value)) => Some((idx, value, false)),
		}
	});

	zipped
		.filter(|(_, _, c)| *c)
		.map(|(idx, v, _)| v * (idx as isize))
		.take(6)
		.sum::<isize>()
		.to_string()
});

solution!("y2022d10p2", complex(input) {
	let cycles: Box<dyn Iterator<Item = isize>> = Box::new(std::iter::empty());
	let reg = Register::new();

	let (iter, _) =
		lines(input)
			.map(|line| parse(&line))
			.fold((cycles, reg), |(cycles, reg), ins| {
				let (values, reg) = reg.read(ins);
				let iter = Box::new(cycles.chain(values.into_iter()));

				(iter, reg)
			});

	let columns = 40;
	iter.enumerate()
		.chunks(columns)
		.into_iter()
		.map(|iter| {
			let line = String::with_capacity(columns);

			iter.fold(line, |mut line, (cycle, x)| {
				let cursor = cycle % columns;
				let sprite = [x - 1, x, x + 1];

				if sprite.contains(&(cursor as isize)) {
					line.push('#');
				} else {
					line.push('.');
				}

				line
			})
		})
		.join("\n")
});

#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;
	use crate::input;

	fn input() -> Box<dyn BufRead> {
		input!(
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
		)
	}

	#[test]
	fn first_example() {
		assert_eq!(basic::solution(input()), "13140")
	}

	#[test]
	fn second_example() {
		let output = indoc! {"
			##..##..##..##..##..##..##..##..##..##..
			###...###...###...###...###...###...###.
			####....####....####....####....####....
			#####.....#####.....#####.....#####.....
			######......######......######......####
			#######.......#######.......#######....."
		};

		assert_eq!(complex::solution(input()), output)
	}
}
