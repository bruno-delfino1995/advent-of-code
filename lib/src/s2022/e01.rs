use std::collections::BinaryHeap;

use crate::prelude::*;

fn parse(input: Input) -> BinaryHeap<usize> {
	let (calories, mut elves) = lines(input).fold(
		(0, BinaryHeap::new()),
		|(calories, mut elves), line| match line.parse::<usize>() {
			Ok(n) => (calories + n, elves),
			Err(_) => {
				elves.push(calories);
				(0, elves)
			}
		},
	);

	elves.push(calories);

	elves
}

pub fn basic(input: Input) -> String {
	let mut elves = parse(input);

	let max: usize = elves.pop().unwrap();

	max.to_string()
}

pub fn complex(input: Input) -> String {
	let elves = parse(input).into_sorted_vec();

	let max: usize = elves.into_iter().rev().take(3).sum();

	max.to_string()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			1000
			2000
			3000

			4000

			5000
			6000

			7000
			8000
			9000

			10000
		"#
		);

		assert_eq!(basic(input), "24000");
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			1000
			2000
			3000

			4000

			5000
			6000

			7000
			8000
			9000

			10000
		"#
		);

		assert_eq!(complex(input), "45000");
	}
}
