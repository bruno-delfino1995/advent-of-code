use chrono::{DateTime, Datelike, Local};
use regex::{Regex, RegexSet};

use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Puzzle {
	year: u16,
	day: u8,
	phase: u8,
}

impl Puzzle {
	pub fn year(&self) -> String {
		self.year.to_string()
	}

	pub fn day(&self) -> String {
		self.day.to_string()
	}

	pub fn phase(&self) -> String {
		self.phase.to_string()
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
			phase: 1,
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
		let pattern = find_pattern(s).ok_or("invalid pattern")?;
		let matches = find_matches(&pattern, s);

		let current_puzzle = Puzzle::default();

		let year = parse_year(matches.get("year"), &current_puzzle)?;
		let day = parse_day(matches.get("day"), &year, &current_puzzle)?;
		let phase = parse_phase(matches.get("phase"), &current_puzzle)?;

		Ok(Puzzle { year, day, phase })
	}
}

fn find_pattern(s: &str) -> Option<String> {
	let set = RegexSet::new([
		r"^(?P<year>\d{4}).(?P<day>\d{1,2}).(?P<phase>\d)$",
		r"^(?P<year>\d{4}).(?P<day>\d{1,2})$",
		r"^(?P<day>\d{1,2}).(?P<phase>\d)$",
		r"^(?P<day>\d{1,2})$",
	])
	.unwrap();

	let patterns: Vec<String> = set
		.matches(s)
		.into_iter()
		.map(|i| set.patterns()[i].to_owned())
		.collect();

	patterns.first().map(|p| p.to_string())
}

fn find_matches(p: &str, s: &str) -> HashMap<String, String> {
	let re = Regex::new(p).unwrap();

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

			let name = String::from("phase");
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

fn parse_phase(p: Option<&String>, Puzzle { phase: cp, .. }: &Puzzle) -> Result<u8, &'static str> {
	match p {
		None => Ok(*cp),
		Some(p) => {
			let p = p.parse::<u8>().map_err(|_| "unable to parse phase")?;

			if p == 0 {
				return Err("phase starts at 1");
			}

			if p > 2 {
				return Err("phase stops at 2");
			}

			Ok(p)
		}
	}
}
