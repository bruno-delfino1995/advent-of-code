use std::fmt;

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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
	row: usize,
	col: usize,
}

impl Position {
	pub fn new(row: usize, col: usize) -> Self {
		Self { row, col }
	}

	pub fn row(&self) -> usize {
		self.row
	}

	pub fn col(&self) -> usize {
		self.col
	}

	pub fn to_linear(self, size: usize) -> usize {
		let row = self.row * size;
		let col = self.col;

		row + col
	}

	pub fn from_linear(idx: usize, size: usize) -> Self {
		let row = idx / size;
		let col = idx - row * size;

		Position { row, col }
	}

	pub fn mirror(&self, size: usize, axis: Axis) -> Self {
		let mirror = |n| (n as isize - (size - 1) as isize).unsigned_abs();

		match axis {
			Column => Position {
				row: self.row,
				col: mirror(self.col),
			},
			Row => Position {
				row: mirror(self.row),
				col: self.col,
			},
		}
	}
}

impl fmt::Debug for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.col, self.row)
	}
}
