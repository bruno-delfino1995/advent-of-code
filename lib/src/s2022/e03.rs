use std::{collections::HashSet, str::FromStr};

use crate::prelude::*;

#[derive(Debug)]
struct Item(char);

impl Item {
	const LOWERCASE_DIFF: usize = 'a' as usize - 1;
	const UPPERCASE_DIFF: usize = 'A' as usize - 27;

	fn value(&self) -> usize {
		let letter = self.0;

		if letter.is_lowercase() {
			letter as usize - Self::LOWERCASE_DIFF
		} else {
			letter as usize - Self::UPPERCASE_DIFF
		}
	}
}

type Compartment = HashSet<char>;
#[derive(Debug)]
struct Rucksack {
	left: Compartment,
	right: Compartment,
}

impl FromStr for Rucksack {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (left, right) = s.split_at(s.len() / 2);

		Ok(Rucksack {
			left: HashSet::from_iter(left.chars()),
			right: HashSet::from_iter(right.chars()),
		})
	}
}

impl Rucksack {
	fn overlap(&self) -> Item {
		let overlap = self.left.intersection(&self.right);

		let item = overlap.last().unwrap();

		Item(*item)
	}
}

pub fn basic(input: Input) -> String {
	lines(input)
		.map(|line| Rucksack::from_str(&line).unwrap().overlap().value())
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
			vJrwpWtwJgWrhcsFMMfFFhFp
			jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
			PmmdzqPrVvPwwTWBwg
			wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
			ttgJtRGJQctTZtZT
			CrZsJsPPZsGzwwsLwLmpwMDw
		"#
		);

		assert_eq!(basic(input), "157");
	}
}
