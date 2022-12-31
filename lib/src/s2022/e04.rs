use std::{ops::RangeInclusive, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::prelude::*;

struct Assignment(RangeInclusive<usize>);

impl Assignment {
	fn contains(&self, other: &Self) -> bool {
		let below = self.0.start() <= other.0.start();
		let above = self.0.end() >= other.0.end();

		below && above
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

pub fn basic(input: Input) -> String {
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
}

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

		assert_eq!(basic(input), "2");
	}
}
