use std::cmp::Ordering;
use std::fmt;

use crate::prelude::*;

use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};

#[derive(Eq, PartialEq, Clone)]
enum Data {
	Number(u8),
	List(Vec<Data>),
}

impl fmt::Debug for Data {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Number(n) => write!(f, "{}", n),
			Self::List(data) => {
				let contents = data
					.iter()
					.map(|d| format!("{:?}", d))
					.collect_vec()
					.join(",");
				write!(f, "[{}]", contents)
			}
		}
	}
}

impl Ord for Data {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Self::Number(a), Self::Number(b)) => a.cmp(b),
			(Self::List(left), Self::List(right)) => {
				let mut left = left.iter();
				let mut right = right.iter();

				loop {
					let a = left.next();
					let b = right.next();

					let order = match (a, b) {
						(None, None) => return Ordering::Equal,
						(None, Some(_)) => return Ordering::Less,
						(Some(_), None) => return Ordering::Greater,
						(Some(a), Some(b)) => a.cmp(b),
					};

					if !order.is_eq() {
						return order;
					}
				}
			}
			(Self::Number(_), _) => {
				let list = Self::List(vec![self.clone()]);

				list.cmp(other)
			}
			(_, Self::Number(_)) => {
				let list = Self::List(vec![other.clone()]);

				self.cmp(&list)
			}
		}
	}
}

impl PartialOrd for Data {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Packet(Vec<Data>);

impl fmt::Debug for Packet {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let contents = self
			.0
			.iter()
			.map(|d| format!("{:?}", d))
			.collect_vec()
			.join(",");
		write!(f, "[{}]", contents)
	}
}

#[derive(Debug, PartialEq, Eq)]
struct Pair(Packet, Packet);

impl Pair {
	fn ordered(&self) -> bool {
		let left = &self.0;
		let right = &self.1;

		matches!(left.cmp(right), Ordering::Less | Ordering::Equal)
	}
}

mod parser {
	use super::*;
	use nom::{
		branch::alt,
		bytes::complete::tag,
		character::complete::{newline, u8},
		combinator::map,
		multi::separated_list0,
		sequence::{delimited, terminated, tuple},
		IResult,
	};

	fn parse_data(s: &str) -> IResult<&str, Data> {
		alt((
			map(u8, Data::Number),
			map(
				delimited(tag("["), separated_list0(tag(","), parse_data), tag("]")),
				Data::List,
			),
		))(s)
	}

	fn parse_packet(s: &str) -> IResult<&str, Packet> {
		map(
			delimited(tag("["), separated_list0(tag(","), parse_data), tag("]")),
			Packet,
		)(s)
	}

	fn parse_pair(s: &str) -> IResult<&str, Pair> {
		map(
			tuple((
				terminated(parse_packet, newline),
				terminated(parse_packet, newline),
			)),
			|(l, r)| Pair(l, r),
		)(s)
	}

	pub(super) fn pairs(s: &str) -> IResult<&str, Vec<Pair>> {
		separated_list0(newline, parse_pair)(s)
	}
}

pub fn basic(mut input: Input) -> String {
	let contents = {
		let mut buf = String::new();
		input.read_to_string(&mut buf).unwrap();

		buf
	};

	let pairs = all_consuming(parser::pairs)(&contents).finish().unwrap().1;

	pairs
		.into_iter()
		.map(|p| p.ordered())
		.enumerate()
		.filter_map(|(i, p)| if p { Some(i + 1) } else { None })
		.sum::<usize>()
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
			[1,1,3,1,1]
			[1,1,5,1,1]

			[[1],[2,3,4]]
			[[1],4]

			[9]
			[[8,7,6]]

			[[4,4],4,4]
			[[4,4],4,4,4]

			[7,7,7,7]
			[7,7,7]

			[]
			[3]

			[[[]]]
			[[]]

			[1,[2,[3,[4,[5,6,7]]]],8,9]
			[1,[2,[3,[4,[5,6,0]]]],8,9]
		"#
		);

		assert_eq!(basic(input), "13")
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
