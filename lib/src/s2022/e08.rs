use std::collections::HashSet;

use itertools::Itertools;

use crate::prelude::*;

type Position = (usize, usize);
type Height = usize;

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

	fn at(&self, position: Position) -> Option<&Height> {
		let row = position.0 * self.size;
		let column = position.1;

		self.grid.get(row + column)
	}

	fn scan_from(
		&self,
		direction: Direction,
	) -> Box<dyn Iterator<Item = (Position, &Height)> + '_> {
		let from_left = |n| {
			let row = n / self.size;
			let col = n - row * self.size;

			(row, col)
		};

		let invert = |n| (n as isize - (self.size - 1) as isize).unsigned_abs();

		let as_pos: Box<dyn Fn(usize) -> Position> = match direction {
			Direction::Up => Box::new(move |n: usize| -> Position {
				let (row, col) = from_left(n);

				(col, row)
			}),
			Direction::Right => Box::new(move |n: usize| -> Position {
				let (row, col) = from_left(n);

				(row, invert(col))
			}),
			Direction::Down => Box::new(move |n: usize| -> Position {
				let (row, col) = from_left(n);

				(invert(col), row)
			}),
			Direction::Left => Box::new(from_left),
		};

		let positions = 0..(self.size * self.size);

		let iter = positions
			.into_iter()
			.map(as_pos)
			.map(|pos| (pos, self.at(pos).unwrap()));

		Box::new(iter)
	}

	fn visible_from_outside(&self) -> HashSet<Position> {
		use Direction::*;

		let by_col: fn(&(Position, &Height)) -> usize = |((_, c), _)| *c;
		let by_row: fn(&(Position, &Height)) -> usize = |((r, _), _)| *r;
		let scans = [
			(Up, by_col),
			(Right, by_row),
			(Down, by_col),
			(Left, by_row),
		];

		let mut can_see = HashSet::new();

		for (direction, grouper) in scans {
			for (_, group) in &self.scan_from(direction).group_by(grouper) {
				let (_, visible) = group.fold(
					(-1, HashSet::new()),
					|(largest, mut visible), (pos, &height)| {
						let height = height as isize;

						if height > largest {
							visible.insert(pos);
						}

						(std::cmp::max(largest, height), visible)
					},
				);

				can_see.extend(visible);
			}
		}

		can_see
	}
}

pub fn basic(input: Input) -> String {
	let forest = lines(input)
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
		.unwrap();

	forest.visible_from_outside().len().to_string()
}

pub fn complex(_input: Input) -> String {
	String::from("unimplemented")
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
			3
		"#
		);

		assert_eq!(complex(input), "2")
	}
}
