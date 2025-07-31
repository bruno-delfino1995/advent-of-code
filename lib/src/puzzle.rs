use std::collections::HashMap;
use std::str::FromStr;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;

use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Puzzle {
	year: u16,
	day: u8,
	part: u8,
}

impl Puzzle {
	pub fn year(&self) -> usize {
		self.year as usize
	}

	pub fn day(&self) -> usize {
		self.day as usize
	}

	pub fn part(&self) -> usize {
		self.part as usize
	}
}

impl Default for Puzzle {
	fn default() -> Self {
		let today = Local::now();
		Self::from(today)
	}
}

impl From<DateTime<Local>> for Puzzle {
	fn from(today: DateTime<Local>) -> Self {
		let is_december = today.month() == 12;

		let year = if is_december {
			today.year()
		} else {
			today.year() - 1
		};

		let day = if is_december { today.day() } else { 25 };

		Puzzle {
			year: year as u16,
			day: day as u8,
			part: 1,
		}
	}
}

impl From<&'static str> for Puzzle {
	fn from(value: &'static str) -> Self {
		Self::from_str(value).expect("invalid puzzle spec")
	}
}

impl FromStr for Puzzle {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let matches = find_matches(s);

		let current_puzzle = Puzzle::default();

		let year = parse_year(matches.get("year"), &current_puzzle)?;
		let day = parse_day(matches.get("day"), &year, &current_puzzle)?;
		let part = parse_part(matches.get("part"), &current_puzzle)?;

		Ok(Puzzle { year, day, part })
	}
}

fn find_matches(s: &str) -> HashMap<String, String> {
	let re = Regex::new(r"^y(?P<year>\d{4})d(?P<day>\d{2})p(?P<part>\d)$").unwrap();

	re.captures(s)
		.map(|c| {
			let mut caps: HashMap<String, String> = HashMap::with_capacity(3);

			let name = String::from("year");
			if let Some(value) = c.name(&name).map(|m| m.as_str().to_string()) {
				caps.insert(name, value);
			}

			let name = String::from("day");
			if let Some(value) = c.name(&name).map(|m| m.as_str().to_string()) {
				caps.insert(name, value);
			}

			let name = String::from("part");
			if let Some(value) = c.name(&name).map(|m| m.as_str().to_string()) {
				caps.insert(name, value);
			}

			caps
		})
		.unwrap_or_default()
}

fn parse_year(y: Option<&String>, Puzzle { year: cy, .. }: &Puzzle) -> Result<u16, &'static str> {
	match y {
		None => Ok(*cy),
		Some(y) => {
			let y = y.parse::<u16>().map_err(|_| "unable to parse year")?;

			if y < 2015 {
				return Err("year starts at 2015");
			}

			if y > *cy {
				return Err("future puzzles are unknown");
			}

			Ok(y)
		}
	}
}

fn parse_day(
	d: Option<&String>,
	y: &u16,
	Puzzle {
		year: cy, day: cd, ..
	}: &Puzzle,
) -> Result<u8, &'static str> {
	match d {
		None => Ok(*cd),
		Some(d) => {
			let d = d.parse::<u8>().map_err(|_| "unable to parse day")?;

			if d == 0 {
				return Err("day start at 1");
			}

			if d > 25 {
				return Err("day stops at 25");
			}

			if y >= cy && d > *cd {
				return Err("future puzzles are unknown");
			}

			Ok(d)
		}
	}
}

fn parse_part(p: Option<&String>, Puzzle { part: cp, .. }: &Puzzle) -> Result<u8, &'static str> {
	match p {
		None => Ok(*cp),
		Some(p) => {
			let p = p.parse::<u8>().map_err(|_| "unable to parse part")?;

			if p == 0 {
				return Err("part starts at 1");
			}

			if p > 2 {
				return Err("part stops at 2");
			}

			Ok(p)
		}
	}
}
