use std::{collections::HashMap, fmt};

use itertools::Itertools;

use crate::{
	common::grid::{Area, Coord, DIRECTIONS},
	prelude::*,
};

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
	fn neighbors(&self, coord: Coord) -> Vec<Coord> {
		let idx = coord.to_linear(self.area);
		let height = &self.heights[idx];

		DIRECTIONS
			.into_iter()
			.filter_map(|d| coord.walk(self.area, d))
			.filter(|c| {
				let idx = c.to_linear(self.area);
				let other = &self.heights[idx];

				height.walkable(other)
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
		let mut checking = HashMap::with_capacity(len);
		checking.insert(map.source, 0);

		Self {
			map,
			checking,
			checked: HashMap::with_capacity(len),
		}
	}

	fn journey(mut self) -> usize {
		loop {
			if let Some(steps) = self.checked.get(&self.map.target) {
				return *steps;
			}

			self = self.climb();
		}
	}

	fn climb(mut self) -> Self {
		let mut next = HashMap::with_capacity(self.checking.len() * 4);

		for (check, steps) in self.checking {
			for neighbor in self.map.neighbors(check) {
				if self.checked.contains_key(&neighbor) {
					continue;
				}

				self.checked.insert(neighbor, steps + 1);
				next.insert(neighbor, steps + 1);
			}
		}

		self.checking = next;
		self
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
				},
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

pub fn basic(input: Input) -> String {
	let map = parse(input);
	let climbers = Climbers::new(map);

	climbers.journey().to_string()
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
			Sabqponm
			abcryxxl
			accszExk
			acctuvwj
			abdefghi
		"#
		);

		assert_eq!(basic(input), "31")
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
