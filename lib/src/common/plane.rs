use auto_ops::*;

use std::cmp::Ordering;
use std::fmt;
use std::ops::*;

use Axis::*;
use Direction::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Axis {
	X,
	Y,
}

impl From<Direction> for Axis {
	fn from(value: Direction) -> Self {
		match value {
			Left | Right => X,
			Up | Down => Y,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl fmt::Debug for Direction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let as_str = match self {
			Up => "up",
			Right => "right",
			Down => "down",
			Left => "left",
		};

		write!(f, "{}", as_str)
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Motion {
	x: isize,
	y: isize,
}

impl Motion {
	pub fn new(x: isize, y: isize) -> Self {
		Self { x, y }
	}
}

impl fmt::Debug for Motion {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}

impl From<Direction> for Motion {
	fn from(value: Direction) -> Self {
		match value {
			Up => Motion::new(0, 1),
			Right => Motion::new(1, 0),
			Down => Motion::new(0, -1),
			Left => Motion::new(-1, 0),
		}
	}
}

impl Add for Motion {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let x = self.x + rhs.x;
		let y = self.y + rhs.y;

		Self::new(x, y)
	}
}

impl Sub for Motion {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let x = self.x - rhs.x;
		let y = self.y - rhs.y;

		Self::new(x, y)
	}
}

impl Mul<isize> for Motion {
	type Output = Self;

	fn mul(self, rhs: isize) -> Self::Output {
		let x = self.x * rhs;
		let y = self.y * rhs;

		Motion::new(x, y)
	}
}

impl Add<Direction> for Motion {
	type Output = Self;

	fn add(self, rhs: Direction) -> Self::Output {
		let other: Motion = rhs.into();

		self + other
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
	x: isize,
	y: isize,
}

impl Point {
	pub fn origin() -> Self {
		Self { x: 0, y: 0 }
	}

	pub fn new(x: isize, y: isize) -> Self {
		Self { x, y }
	}

	pub fn x(&self) -> isize {
		self.x
	}

	pub fn y(&self) -> isize {
		self.y
	}
}

impl fmt::Debug for Point {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl From<Motion> for Point {
	fn from(value: Motion) -> Self {
		Point::new(value.x, value.y)
	}
}

impl From<(isize, isize)> for Point {
	fn from((x, y): (isize, isize)) -> Self {
		Point::new(x, y)
	}
}

impl Ord for Point {
	fn cmp(&self, other: &Self) -> Ordering {
		let x = self.x.cmp(&other.x);
		let y = self.y.cmp(&other.y);

		if x != Ordering::Equal { x } else { y }
	}
}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl_op_ex!(+ |lhs: &Point, rhs: &Point| -> Point {
	let x = lhs.x + rhs.x;
	let y = lhs.y + rhs.y;

	Point::new(x, y)
});

impl_op_ex!(+ |lhs: &Point, rhs: &Motion| -> Point {
	let point: Point = rhs.clone().into();
	lhs + point
});

impl_op_ex!(+ |lhs: &Point, rhs: &Direction| -> Point {
	let motion: Motion = rhs.clone().into();
	lhs + motion
});

impl_op_ex!(- |lhs: &Point, rhs: &Point| -> Point {
	let x = lhs.x - rhs.x;
	let y = lhs.y - rhs.y;

	Point::new(x, y)
});

impl_op_ex!(- |lhs: &Point, rhs: &Motion| -> Point {
	let point: Point = rhs.clone().into();
	lhs - point
});

impl_op_ex!(- |lhs: &Point, rhs: &Direction| -> Point {
	let motion: Motion = rhs.clone().into();
	lhs - motion
});
