use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Debug)]
struct Reader{
	tail: Option<char>,
	size: usize,
	data: VecDeque<(usize, char)>,
	seen: [usize; 26],
}

impl Reader {
	pub fn new(size: usize) -> Self {
		Self {
			size,
			tail: None,
			data: VecDeque::with_capacity(size),
			seen: [0; 26],
		}
	}

	const START: usize = 'a' as usize;

	pub fn read(&mut self, pair@(_, c): (usize, char)) {
		if self.is_full() {
			let idx = self.tail.unwrap() as usize - Self::START;
			self.seen[idx] = self.seen[idx].saturating_sub(1);
			self.data.pop_back();
			self.tail = self.data.back().map(|(_, c)| *c);
		} else if self.is_empty() {
			self.tail = Some(c);
		}

		let idx = c as usize - Self::START;
		self.seen[idx] = self.seen[idx] + 1;
		self.data.push_front(pair);

		return;
	}

	pub fn is_full(&self) -> bool {
		self.data.len() == self.size
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	pub fn distinct(&self) -> bool {
		self.seen.iter().all(|&n| n <= 1)
	}

	pub fn last_index(&self) -> usize {
		self.data.front().unwrap().0
	}
}

fn marker(message: &str, size: usize) -> usize {
	let mut collector = Reader::new(size);

	message.chars().enumerate().fold_while(0, |_, el| {
		collector.read(el);

		if collector.is_full() && collector.distinct() {
			Done(collector.last_index())
		} else {
			Continue(0)
		}
	}).into_inner()
}

pub fn basic(input: Input) -> String {
	let message: String = lines(input).collect();

	let start_at = marker(&message, 4) + 1;

	start_at.to_string()
}

pub fn complex(input: Input) -> String {
	let message: String = lines(input).collect();

	let start_at = marker(&message, 14) + 1;

	start_at.to_string()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let cases = [
			(input!("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7"),
			(input!("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5"),
			(input!("nppdvjthqldpwncqszvftbrmjlhg"), "6"),
			(input!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10"),
			(input!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11"),
		];

		for (input, result) in cases {
			assert_eq!(basic(input), result)
		}
	}

	#[test]
	fn second_example() {
		let cases = [
			(input!("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19"),
			(input!("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23"),
			(input!("nppdvjthqldpwncqszvftbrmjlhg"), "23"),
			(input!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29"),
			(input!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26"),
		];

		for (input, result) in cases {
			assert_eq!(complex(input), result)
		}
	}
}
