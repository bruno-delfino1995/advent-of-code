use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::{ops::RangeInclusive, str::FromStr};

use crate::prelude::*;

struct Assignment(RangeInclusive<usize>);

impl Assignment {
	fn contains(&self, other: &Self) -> bool {
		let below = self.0.start() <= other.0.start();
		let above = self.0.end() >= other.0.end();

		below && above
	}

	fn intersects(&self, other: &Self) -> bool {
		let ours: HashSet<usize> = HashSet::from_iter(self.0.clone());
		let theirs: HashSet<usize> = HashSet::from_iter(other.0.clone());

		!ours.intersection(&theirs).collect_vec().is_empty()
	}
}

impl FromStr for Assignment {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static! {
			static ref PATTERN: Regex = Regex::new(r"(?P<start>\d*)-(?P<end>\d*)").unwrap();
		}

		let captures = PATTERN.captures(s).unwrap();

		let start = captures.name("start").unwrap().as_str().parse().unwrap();
		let end = captures.name("end").unwrap().as_str().parse().unwrap();

		Ok(Assignment(start..=end))
	}
}

solution!("2022.4.1", basic(input) {
	lines(input)
		.map(|line| {
			let (first, second) = line
				.split(',')
				.map(|assign| Assignment::from_str(assign).unwrap())
				.collect_tuple()
				.unwrap();

			first.contains(&second) || second.contains(&first)
		})
		.filter(|&c| c)
		.collect_vec()
		.len()
		.to_string()
});

solution!("2022.4.2", complex(input) {
	lines(input)
		.map(|line| {
			let (first, second) = line
				.split(',')
				.map(|assign| Assignment::from_str(assign).unwrap())
				.collect_tuple()
				.unwrap();

			first.intersects(&second)
		})
		.filter(|&c| c)
		.collect_vec()
		.len()
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
			2-4,6-8
			2-3,4-5
			5-7,7-9
			2-8,3-7
			6-6,4-6
			2-6,4-8
		"#
		);

		assert_eq!(basic::solution(input), "2");
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			2-4,6-8
			2-3,4-5
			5-7,7-9
			2-8,3-7
			6-6,4-6
			2-6,4-8
		"#
		);

		assert_eq!(complex::solution(input), "4");
	}
}
