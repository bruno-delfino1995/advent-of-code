use nom::{combinator::all_consuming, Finish};
use std::collections::HashSet;
use std::fmt;

use crate::common::plane::{Direction, Motion, Point};
use crate::prelude::*;
use Direction::*;

struct Knot {
	point: Point,
	tail: Option<Box<Knot>>,
}

impl fmt::Debug for Knot {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.point)?;

		match self.tail {
			Some(ref tail) => write!(f, " <=> {:?}", tail),
			None => Ok(()),
		}
	}
}

impl Knot {
	fn new() -> Self {
		Self {
			point: Point::origin(),
			tail: None,
		}
	}

	fn points(&self) -> Vec<Point> {
		let mut res = vec![];
		res.push(self.point);

		let mut curr = self;
		while let Some(ref tail) = curr.tail {
			res.push(tail.point);
			curr = tail;
		}

		res
	}

	fn link(mut self, tail: Knot) -> Self {
		self.tail = Some(Box::new(tail));
		self
	}

	fn go(&mut self, direction: Direction) {
		self.conduct(direction.into());
	}

	fn conduct(&mut self, motion: Motion) {
		let delta: Point = motion.into();
		self.point = self.point + delta;

		if self.tail.is_some() {
			let mut tail = self.tail.take().unwrap();
			tail.follow(self);
			self.tail = Some(tail);
		}
	}

	fn follow(&mut self, head: &Knot) {
		let tail = self.point;
		let head = head.point;
		let (x, y) = {
			let delta = head - tail;

			(delta.x(), delta.y())
		};

		#[allow(unused_variables)]
		match (x.unsigned_abs(), y.unsigned_abs()) {
			is_adjacent @ (c, r) if c <= 1 && r <= 1 => (),
			not_diagonal @ ((0, _) | (_, 0)) => self.conduct(Motion::new(x / 2, y / 2)),
			perfect_diagonal @ (c, r) if c == r => self.conduct(Motion::new(x / 2, y / 2)),
			far_in_y @ (c, r) if c < r => self.conduct(Motion::new(x, y / 2)),
			far_in_x @ (c, r) if c > r => self.conduct(Motion::new(x / 2, y)),
			_ => unreachable!("it's either linear or a diagonal"),
		}
	}
}

struct Rope {
	head: Knot,
}

impl fmt::Debug for Rope {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.head)
	}
}

impl Rope {
	fn new(size: usize) -> Self {
		let knots = std::iter::repeat_with(Knot::new).take(size.saturating_sub(2));

		let tail = Knot::new();
		let second = knots.fold(tail, |tail, knot| knot.link(tail));
		let head = Knot::new().link(second);

		Self { head }
	}

	fn points(&self) -> Vec<Point> {
		self.head.points()
	}

	fn go(mut self, direction: Direction) -> Self {
		self.head.go(direction);
		self
	}
}

mod parser {
	use super::*;
	use nom::{
		bytes::complete::{tag, take},
		character::complete::digit1,
		combinator::{map, map_res},
		sequence::separated_pair,
		IResult,
	};

	fn parse_direction(s: &str) -> IResult<&str, Direction> {
		map_res(take(1usize), |s: &str| match s {
			"U" => Ok(Up),
			"R" => Ok(Right),
			"D" => Ok(Down),
			"L" => Ok(Left),
			_ => Err("unknown direction"),
		})(s)
	}

	pub(super) fn directions(s: &str) -> IResult<&str, Vec<Direction>> {
		let digit = map_res(digit1, |d: &str| d.parse::<usize>());

		map(
			separated_pair(parse_direction, tag(" "), digit),
			|(dir, amount)| std::iter::repeat(dir).take(amount).collect(),
		)(s)
	}
}

fn parse(input: Input) -> Box<dyn Iterator<Item = Direction>> {
	let iter = lines(input)
		.filter_map(|l| {
			all_consuming(parser::directions)(&l)
				.finish()
				.ok()
				.map(|(_, v)| v)
		})
		.flatten();

	Box::new(iter)
}

fn solve(input: Input, len: usize) -> String {
	let rope = Rope::new(len);
	let points = {
		let mut set = HashSet::new();
		set.insert(Point::new(0, 0));

		set
	};

	let (_, points) = parse(input).fold((rope, points), |(rope, mut points), dir| {
		let rope = rope.go(dir);
		let point = rope.points().last().cloned().unwrap();

		points.insert(point);

		(rope, points)
	});

	points.len().to_string()
}

solution!("2022.9.1", basic(input) {
	solve(input, 2)
});

solution!("2022.9.2", complex(input) {
	solve(input, 10)
});

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			R 4
			U 4
			L 3
			D 1
			R 4
			D 1
			L 5
			R 2
		"#
		);

		assert_eq!(basic::solution(input), "13")
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			R 5
			U 8
			L 8
			D 3
			R 17
			D 10
			L 25
			U 20
		"#
		);

		assert_eq!(complex::solution(input), "36")
	}
}
