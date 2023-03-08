use std::collections::BTreeSet;

use crate::common::plane::Direction;
use crate::{common::plane::Point, prelude::*};

use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};

#[derive(Debug)]
struct Trace(Vec<Point>);

impl Into<(Vec<isize>, Vec<isize>)> for Trace {
	fn into(self) -> (Vec<isize>, Vec<isize>) {
		self.0
			.into_iter()
			.tuple_windows()
			.flat_map(|(a, b)| {
				let [a, b] = {
					let mut points = [a, b];
					points.sort();

					points
				};

				if a.x() == b.x() {
					std::iter::repeat(a.x()).zip(a.y()..=b.y()).collect_vec()
				} else {
					(a.x()..=b.x()).zip(std::iter::repeat(a.y())).collect_vec()
				}
			})
			.fold((vec![], vec![]), |(mut xs, mut ys), (x, y)| {
				xs.push(x);
				ys.push(y);

				(xs, ys)
			})
	}
}

#[derive(Debug)]
struct Cave {
	points: BTreeSet<Point>,
	height: isize,
	floor: isize,
}

impl From<Vec<Trace>> for Cave {
	fn from(traces: Vec<Trace>) -> Self {
		let mut height = None;
		let mut points = BTreeSet::new();

		for trace in traces {
			let (xs, ys) = trace.into();
			let max = ys.iter().max();

			if height.is_none() || max > height.as_ref() {
				height = max.cloned();
			}

			let rocks: BTreeSet<_> = xs.into_iter().zip(ys.into_iter()).map(Point::from).collect();

			points.extend(rocks)
		}

		Cave {
			points,
			height: height.unwrap(),
			floor: 0
		}
	}
}

impl Cave {
	fn len(&self) -> usize {
		self.points.len()
	}

	fn drip(mut self) -> Self {
		// Generate a point at 500x(max y) and drop it until it sets or falls beyond (min y)
		let mut sand = Point::new(500, 0);
		if self.points.contains(&sand) {
			return self;
		}

		loop {
			if self.floor != 0 && sand.y() == self.floor - 1 {
				self.points.insert(sand);
				return self;
			}

			if sand.y() > self.height {
				return self;
			}
		
			// 0 is the top instead of the bottom, that's why down = up in terms of motions
			let down = sand + Direction::Up;
			let left = sand + Direction::Up + Direction::Left;
			let right = sand + Direction::Up + Direction::Right;
		
			match (self.points.contains(&down), self.points.contains(&left), self.points.contains(&right)) {
				_is_set @ (true, true, true) => {
					self.points.insert(sand);
					return self;
				},
				(false, _, _) => sand = down,
				(true, false, _) => sand = left,
				(true, true, false) => sand = right,
			}
		}
	}
}

mod parser {
	use super::*;
	use crate::common::parse::isize;
	use nom::{
		bytes::complete::tag,
		character::complete::newline,
		combinator::{eof, map},
		multi::{many_till, separated_list1},
		sequence::{separated_pair, terminated},
		IResult,
	};

	fn parse_point(s: &str) -> IResult<&str, Point> {
		map(separated_pair(isize, tag(","), isize), |(x, y)| {
			Point::new(x, y)
		})(s)
	}

	fn parse_trace(s: &str) -> IResult<&str, Trace> {
		terminated(
			map(separated_list1(tag(" -> "), parse_point), Trace),
			newline,
		)(s)
	}

	pub(super) fn cave(s: &str) -> IResult<&str, Cave> {
		map(many_till(parse_trace, eof), |(traces, _)| {
			Cave::from(traces)
		})(s)
	}
}

fn parse(input: Input) -> Cave {
	let contents = all(input);
	let parsed = all_consuming(parser::cave)(&contents).finish();

	parsed.ok().unwrap().1
}

solution!("2022.14.1", basic(input) {
	let mut cave = parse(input);

	let mut amount = 0;
	loop {
		let before = cave.len();
		cave = cave.drip();
		if cave.len() > before {
			amount += 1;
		} else {
			break;
		}
	};
	
	amount.to_string()
});

solution!("2022.14.2", complex(input) {
	let mut cave = parse(input);
	cave.floor = cave.height + 2;

	let mut amount = 0;
	loop {
		let before = cave.len();
		cave = cave.drip();
		if cave.len() > before {
			amount += 1;
		} else {
			break;
		}
	};
	
	amount.to_string()
});

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	fn input() -> Input {
		input!(
			r#"
			498,4 -> 498,6 -> 496,6
			503,4 -> 502,4 -> 502,9 -> 494,9
			"#
		)
	}

	#[test]
	fn first_example() {
		assert_eq!(basic::solution(input()), "24")
	}

	#[test]
	fn second_example() {
		assert_eq!(complex::solution(input()), "93")
	}
}
