use std::{cmp::Ordering::*, collections::HashSet};

use crate::common::grid::{Axis, Coord, Direction};
use crate::prelude::*;
use Axis::*;
use Direction::*;

type Height = usize;

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

	fn at(&self, pos: &Coord) -> Option<Height> {
		let idx = pos.to_linear(self.size.into());

		self.grid.get(idx).copied()
	}

	fn borders(&self) -> Vec<(Coord, Direction)> {
		let top = (0..self.size).map(|col| (Coord::new(0, col), Down));
		let right = (0..self.size).map(|row| (Coord::new(row, self.size - 1), Left));
		let bottom = top
			.clone()
			.rev()
			.map(|(p, _)| (p.mirror(self.size.into(), Row), Up));
		let left = right
			.clone()
			.rev()
			.map(|(p, _)| (p.mirror(self.size.into(), Column), Right));

		top.chain(right).chain(bottom).chain(left).collect()
	}

	fn visible_for(&self, observer: &mut dyn Observer) -> Vec<(Coord, Height)> {
		let pos = observer.location();
		let dir = observer.looking();

		let to_check: Box<dyn Iterator<Item = Coord>> = match dir {
			Up => Box::new((0..=pos.row()).rev().map(|row| Coord::new(row, pos.col()))),
			Right => Box::new((pos.col()..self.size).map(|col| Coord::new(pos.row(), col))),
			Down => Box::new((pos.row()..self.size).map(|row| Coord::new(row, pos.col()))),
			Left => Box::new((0..=pos.col()).rev().map(|col| Coord::new(pos.row(), col))),
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

	fn visible_from_outside(&self) -> HashSet<Coord> {
		self.borders()
			.into_iter()
			.flat_map(|(origin, direction)| {
				let mut elf = Elf::new(origin, direction);

				self.visible_for(&mut elf)
			})
			.map(|(p, _)| p)
			.collect()
	}

	fn scenic_scores(&self) -> Vec<(Coord, usize)> {
		const DIRECTIONS: [Direction; 4] = [Up, Right, Down, Left];

		self.grid
			.iter()
			.enumerate()
			.map(|(idx, h)| {
				let pos = Coord::from_linear(idx, self.size.into());

				let score = DIRECTIONS
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
	fn location(&self) -> Coord;

	fn looking(&self) -> Direction;

	fn detects(&mut self, tree: &(Coord, Height)) -> bool;
}

struct Elf {
	origin: Coord,
	direction: Direction,
	latest: Option<Height>,
}

impl Elf {
	fn new(pos: Coord, dir: Direction) -> Self {
		Self {
			origin: pos,
			direction: dir,
			latest: None,
		}
	}
}

impl Observer for Elf {
	fn location(&self) -> Coord {
		self.origin
	}

	fn looking(&self) -> Direction {
		self.direction
	}

	fn detects(&mut self, (_, height): &(Coord, Height)) -> bool {
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
	origin: Coord,
	direction: Direction,
	proceed: bool,
}

impl TreeHouse {
	fn new(height: Height, pos: Coord, dir: Direction) -> Self {
		Self {
			height,
			origin: pos,
			direction: dir,
			proceed: true,
		}
	}
}

impl Observer for TreeHouse {
	fn location(&self) -> Coord {
		self.origin
	}

	fn looking(&self) -> Direction {
		self.direction
	}

	fn detects(&mut self, (pos, height): &(Coord, Height)) -> bool {
		if pos == &self.origin {
			return false;
		}

		if !self.proceed {
			return false;
		}

		if height >= &self.height {
			self.proceed = false;
		}

		true
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

	forest
		.scenic_scores()
		.iter()
		.map(|(_, s)| s)
		.max()
		.unwrap()
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
