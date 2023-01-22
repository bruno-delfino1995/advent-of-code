use std::{
	fmt,
	ops::{Add, Sub},
};

use Axis::*;
use Direction::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Axis {
	Column,
	Row,
}

impl From<Direction> for Axis {
	fn from(value: Direction) -> Self {
		match value {
			Left | Right => Column,
			Up | Down => Row,
		}
	}
}

pub const DIRECTIONS: [Direction; 4] = [Up, Right, Down, Left];

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
struct Motion {
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
			Up => Motion::new(0, -1),
			Right => Motion::new(1, 0),
			Down => Motion::new(0, 1),
			Left => Motion::new(-1, 0),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Area {
	height: usize,
	width: usize,
}

impl From<(usize, usize)> for Area {
	fn from((h, w): (usize, usize)) -> Self {
		Area {
			height: h,
			width: w,
		}
	}
}

impl From<usize> for Area {
	fn from(value: usize) -> Self {
		Area {
			height: value,
			width: value,
		}
	}
}

impl Area {
	pub fn height(&self) -> usize {
		self.height
	}

	pub fn width(&self) -> usize {
		self.width
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
	row: usize,
	col: usize,
}

impl From<(usize, usize)> for Coord {
	fn from((r, c): (usize, usize)) -> Self {
		Coord { row: r, col: c }
	}
}

impl Coord {
	pub fn new(row: usize, col: usize) -> Self {
		Self { row, col }
	}

	pub fn row(&self) -> usize {
		self.row
	}

	pub fn col(&self) -> usize {
		self.col
	}

	pub fn to_linear(self, area: Area) -> usize {
		let row = self.row * area.width;
		let col = self.col;

		row + col
	}

	pub fn from_linear(idx: usize, grid: Area) -> Self {
		let row = idx / grid.width;
		let col = idx - row * grid.width;

		Coord { row, col }
	}

	pub fn mirror(&self, area: Area, axis: Axis) -> Self {
		let mirror = |n, size| (n as isize - (size - 1) as isize).unsigned_abs();

		match axis {
			Column => Coord {
				row: self.row,
				col: mirror(self.col, area.width),
			},
			Row => Coord {
				row: mirror(self.row, area.height),
				col: self.col,
			},
		}
	}

	pub fn walk(&self, area: Area, dir: Direction) -> Option<Self> {
		let new = *self + dir;

		if new.within(area) && new != *self {
			Some(new)
		} else {
			None
		}
	}

	pub fn within(&self, area: Area) -> bool {
		self.row < area.height && self.col < area.width
	}
}

impl fmt::Debug for Coord {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.row, self.col)
	}
}

impl Add<Direction> for Coord {
	type Output = Self;

	fn add(self, rhs: Direction) -> Self::Output {
		let motion: Motion = rhs.into();

		let row = self.row.saturating_add_signed(motion.x);
		let col = self.col.saturating_add_signed(motion.y);

		(row, col).into()
	}
}

impl Sub<Direction> for Coord {
	type Output = Self;

	fn sub(self, rhs: Direction) -> Self::Output {
		let motion: Motion = rhs.into();

		let row = self.row.saturating_add_signed(-motion.x);
		let col = self.col.saturating_add_signed(-motion.y);

		(row, col).into()
	}
}
