use itertools::Itertools;
use std::convert::TryInto;
use std::{collections::HashSet, str::FromStr};

use crate::prelude::*;

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

	fn items(&self) -> HashSet<char> {
		let len = self.left.len() + self.right.len() - 1;
		let mut items = HashSet::with_capacity(len);

		items.extend(self.left.iter());
		items.extend(self.right.iter());

		items
	}
}

struct Group([Rucksack; 3]);

impl Group {
	fn badge(&self) -> Item {
		let [ref first, ref second, ref third] = self.0;

		let shared = HashSet::from_iter(first.items().intersection(&second.items()).copied());

		let items = third.items();
		let badge = shared.intersection(&items).last().unwrap();

		Item(*badge)
	}
}

pub fn basic(input: Input) -> String {
	lines(input)
		.map(|line| Rucksack::from_str(&line).unwrap().overlap().value())
		.sum::<usize>()
		.to_string()
}

pub fn complex(input: Input) -> String {
	lines(input)
		.map(|line| Rucksack::from_str(&line).unwrap())
		.chunks(3)
		.into_iter()
		.map(|group| {
			let boxed = group.collect_vec().try_into().unwrap();

			Group(boxed).badge().value()
		})
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

	#[test]
	fn second_example() {
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

		assert_eq!(complex(input), "70");
	}
}
