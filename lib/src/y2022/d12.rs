use itertools::Itertools;
use std::{collections::HashMap, fmt};

use crate::{
	common::grid::{Area, Coord, Direction},
	prelude::*,
};
use Target::*;

#[derive(Clone, Copy)]
enum Target {
	Peak,
	Valley,
}

#[derive(Clone, Copy)]
struct Elevation(u8);

impl From<char> for Elevation {
	fn from(value: char) -> Self {
		Self(value as u8 - b'a')
	}
}

impl Elevation {
	fn walkable(&self, other: &Elevation) -> bool {
		other.0 <= self.0 + 1
	}

	fn as_char(&self) -> char {
		(self.0 + b'a') as char
	}
}

struct Heightmap {
	heights: Vec<Elevation>,
	area: Area,
	source: Coord,
	target: Coord,
}

impl fmt::Display for Heightmap {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let lines = (0..self.area.height())
			.into_iter()
			.map(|i| {
				(0..self.area.width())
					.map(|j| {
						let coord: Coord = (i, j).into();

						if coord == self.source {
							'S'
						} else if coord == self.target {
							'E'
						} else {
							let idx = coord.to_linear(self.area);
							let rep = &self.heights[idx];

							rep.as_char()
						}
					})
					.collect::<String>()
			})
			.collect_vec()
			.join("\n");

		writeln!(f, "{}", lines)
	}
}

impl Heightmap {
	fn at(&self, coord: Coord) -> Elevation {
		let idx = coord.to_linear(self.area);
		self.heights[idx]
	}

	fn neighbors(&self, coord: Coord, target: Target) -> Vec<Coord> {
		let height = self.at(coord);

		Direction::all()
			.into_iter()
			.filter_map(|d| coord.walk(self.area, d))
			.filter(|c| {
				let idx = c.to_linear(self.area);
				let other = self.heights[idx];

				match target {
					Peak => height.walkable(&other),
					Valley => other.walkable(&height),
				}
			})
			.collect()
	}

	fn len(&self) -> usize {
		self.heights.len()
	}
}

struct Climbers {
	map: Heightmap,
	checking: HashMap<Coord, usize>,
	checked: HashMap<Coord, usize>,
}

impl Climbers {
	fn new(map: Heightmap) -> Self {
		let len = map.len();

		Self {
			map,
			checking: HashMap::with_capacity(len),
			checked: HashMap::with_capacity(len),
		}
	}

	fn uphill(mut self) -> usize {
		self.checking.insert(self.map.source, 0);
		let target = self.map.target;

		loop {
			if let Some(coord) = self.walk(Peak, |(coord, _)| coord == target) {
				return *self.checked.get(&coord).unwrap();
			}
		}
	}

	fn downhill(mut self) -> usize {
		self.checking.insert(self.map.target, 0);

		loop {
			if let Some(coord) = self.walk(Valley, |(_, elevation)| elevation.as_char() == 'a') {
				return *self.checked.get(&coord).unwrap();
			}
		}
	}

	fn walk<F: Fn((Coord, Elevation)) -> bool>(
		&mut self,
		target: Target,
		is_destination: F,
	) -> Option<Coord> {
		let mut next = HashMap::with_capacity(self.checking.len() * 4);

		for (check, steps) in self.checking.iter() {
			for neighbor in self.map.neighbors(*check, target) {
				if self.checked.contains_key(&neighbor) {
					continue;
				}

				self.checked.insert(neighbor, steps + 1);
				next.insert(neighbor, steps + 1);

				if is_destination((neighbor, self.map.at(neighbor))) {
					return Some(neighbor);
				}
			}
		}

		self.checking = next;
		None
	}
}

fn parse(input: Input) -> Heightmap {
	let mut height = 0;
	let mut width = 0;
	let mut source = (0, 0);
	let mut target = (0, 0);
	let mut heights = Vec::new();

	for line in lines(input) {
		width = line.len();
		height += 1;

		let mut row = Vec::with_capacity(width);
		for (pos, elevation) in line.chars().enumerate() {
			match elevation {
				'S' => {
					source = (height - 1, pos);
					row.push(Elevation::from('a'));
				}
				'E' => {
					target = (height - 1, pos);
					row.push(Elevation::from('z'));
				}
				letter => {
					row.push(Elevation::from(letter));
				}
			}
		}

		heights.extend(row);
	}

	let size = (height, width);
	Heightmap {
		heights,
		area: size.into(),
		source: source.into(),
		target: target.into(),
	}
}

solution!("y2022d12p1", basic(input) {
	let map = parse(input);
	let climbers = Climbers::new(map);

	climbers.uphill().to_string()
});

solution!("y2022d12p2", complex(input) {
	let map = parse(input);
	let climbers = Climbers::new(map);

	climbers.downhill().to_string()
});

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			Sabqponm
			abcryxxl
			accszExk
			acctuvwj
			abdefghi
		"#
		);

		assert_eq!(basic::solution(input), "31")
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			Sabqponm
			abcryxxl
			accszExk
			acctuvwj
			abdefghi
		"#
		);

		assert_eq!(complex::solution(input), "29")
	}
}
