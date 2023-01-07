use std::{cmp::Ordering::*, collections::HashSet, fmt};

use crate::prelude::*;
use Axis::*;
use Direction::*;

type Height = usize;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Axis {
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
	row: usize,
	col: usize,
}

impl Position {
	fn new(row: usize, col: usize) -> Self {
		Self { row, col }
	}

	fn to_linear(self, size: usize) -> usize {
		let row = self.row * size;
		let col = self.col;

		row + col
	}

	fn from_linear(idx: usize, size: usize) -> Self {
		let row = idx / size;
		let col = idx - row * size;

		Position { row, col }
	}

	fn mirror(&self, size: usize, axis: Axis) -> Self {
		let mirror = |n| (n as isize - (size - 1) as isize).unsigned_abs();

		match axis {
			X => Position {
				row: self.row,
				col: mirror(self.col),
			},
			Y => Position {
				row: mirror(self.row),
				col: self.col,
			},
		}
	}
}

impl fmt::Debug for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.row, self.col)
	}
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Debug)]
struct Forest {
	size: usize,
	grid: Vec<Height>,
}

impl Forest {
	fn new(size: usize) -> Self {
		Forest {
			size,
			grid: Vec::with_capacity(size * size),
		}
	}

	fn plant(&mut self, row: Vec<Height>) {
		self.grid.extend(row)
	}

	fn at(&self, pos: &Position) -> Option<Height> {
		let idx = pos.to_linear(self.size);

		self.grid.get(idx).copied()
	}

	fn borders(&self) -> Vec<(Position, Direction)> {
		let top = (0..self.size).map(|col| (Position::new(0, col), Down));
		let right = (0..self.size).map(|row| (Position::new(row, self.size - 1), Left));
		let bottom = top.clone().rev().map(|(p, _)| (p.mirror(self.size, Y), Up));
		let left = right
			.clone()
			.rev()
			.map(|(p, _)| (p.mirror(self.size, X), Right));

		top.chain(right).chain(bottom).chain(left).collect()
	}

	fn visible_for(&self, observer: &mut dyn Observer) -> Vec<(Position, Height)> {
		let pos = observer.location();
		let dir = observer.looking();

		let to_check: Box<dyn Iterator<Item = Position>> = match dir {
			Up => Box::new((0..=pos.row).rev().map(|row| Position::new(row, pos.col))),
			Right => Box::new((pos.col..self.size).map(|col| Position::new(pos.row, col))),
			Down => Box::new((pos.row..self.size).map(|row| Position::new(row, pos.col))),
			Left => Box::new((0..=pos.col).rev().map(|col| Position::new(pos.row, col))),
		};

		to_check
			.filter_map(|pos| {
				self.at(&pos).and_then(|height| {
					if observer.detects(&(pos, height)) {
						Some((pos, height))
					} else {
						None
					}
				})
			})
			.collect()
	}

	fn visible_from_outside(&self) -> HashSet<Position> {
		self.borders()
			.into_iter()
			.flat_map(|(origin, direction)| {
				let mut elf = Elf::new(origin, direction);

				self.visible_for(&mut elf)
			})
			.map(|(p, _)| p)
			.collect()
	}

	fn scenic_scores(&self) -> Vec<(Position, usize)> {
		const DIRECTIONS: [Direction; 4] = [Up, Right, Down, Left];

		self.grid
			.iter()
			.enumerate()
			.map(|(idx, h)| {
				let pos = Position::from_linear(idx, self.size);

				let score =
					DIRECTIONS
						.into_iter()
						.map(|d| {
							let mut tree_house = TreeHouse::new(*h, pos, d);

							self.visible_for(&mut tree_house).len()
						})
						.product();

				(pos, score)
			})
			.collect()
	}
}

trait Observer {
	fn location(&self) -> Position;

	fn looking(&self) -> Direction;

	fn detects(&mut self, tree: &(Position, Height)) -> bool;
}

struct Elf {
	origin: Position,
	direction: Direction,
	latest: Option<Height>,
}

impl Elf {
	fn new(pos: Position, dir: Direction) -> Self {
		Self {
			origin: pos,
			direction: dir,
			latest: None,
		}
	}
}

impl Observer for Elf {
	fn location(&self) -> Position {
		self.origin
	}

	fn looking(&self) -> Direction {
		self.direction
	}

	fn detects(&mut self, (_, height): &(Position, Height)) -> bool {
		let order = self.latest.map(|l| height.cmp(&l)).unwrap_or(Greater);

		match order {
			Less | Equal => false,
			Greater => {
				self.latest = Some(*height);

				true
			}
		}
	}
}

struct TreeHouse {
	height: Height,
	origin: Position,
	direction: Direction,
	proceed: bool,
}

impl TreeHouse {
	fn new(height: Height, pos: Position, dir: Direction) -> Self {
		Self {
			height,
			origin: pos,
			direction: dir,
			proceed: true,
		}
	}
}

impl Observer for TreeHouse {
	fn location(&self) -> Position {
		self.origin
	}

	fn looking(&self) -> Direction {
		self.direction
	}

	fn detects(&mut self, (pos, height): &(Position, Height)) -> bool {
		if pos == &self.origin {
			return false
		}

		if !self.proceed {
			return false
		}

		if height >= &self.height {
			self.proceed = false;
		}

		return true;
	}
}

fn parse(input: Input) -> Forest {
	lines(input)
		.fold(None, |acc, line| {
			let row: Vec<_> = line
				.chars()
				.map(|height| String::from(height).parse::<usize>().unwrap())
				.collect();

			match acc {
				None => {
					let mut forest = Forest::new(row.len());
					forest.plant(row);

					Some(forest)
				}
				Some(mut forest) => {
					forest.plant(row);

					Some(forest)
				}
			}
		})
		.unwrap()
}

pub fn basic(input: Input) -> String {
	let forest = parse(input);

	forest.visible_from_outside().len().to_string()
}

pub fn complex(input: Input) -> String {
	let forest = parse(input);

	forest.scenic_scores().iter().map(|(_, s)| s).max().unwrap().to_string()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			30373
			25512
			65332
			33549
			35390
		"#
		);

		assert_eq!(basic(input), "21")
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			30373
			25512
			65332
			33549
			35390
		"#
		);

		assert_eq!(complex(input), "8")
	}
}
