use chrono::{Datelike, Local};
use clap::Parser;
use regex::{Regex, RegexSet};
use std::{collections::HashMap, str::FromStr};

#[derive(Parser, Debug)]
#[command(name = "Advent of Code", about = "Solution to AOC puzzles")]
#[command(
	next_line_help = true,
	hide_possible_values = true,
	disable_help_subcommand = true,
	disable_version_flag = true,
	help_expected = true
)]
pub struct Args {
	#[arg(
		value_name = "puzzle",
		help = "Which puzzle should I run? (`year.day.phase` - YYYY.DD.P | YYYY.DD | DD.P | DD)"
	)]
	puzzle: Option<Puzzle>,
}

fn main() {
	let args = Args::parse();

	let puzzle = args.puzzle.unwrap_or_default();

	println!("{:?}", puzzle)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Puzzle(u16, u8, u8);

impl Default for Puzzle {
	fn default() -> Self {
		let today = Local::now();
		let is_december = today.month() == 12;

		let year = if is_december {
			today.year()
		} else {
			today.year() - 1
		};

		let day = if is_december { today.day() } else { 25 };

		Puzzle(year as u16, day as u8, 3)
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

		Ok(Puzzle(year, day, phase))
	}
}

fn find_pattern(s: &str) -> Option<String> {
	let set = RegexSet::new(&[
		r"^(?P<year>\d{4}).(?P<day>\d{1,2}).(?P<phase>\d)$",
		r"^(?P<year>\d{4}).(?P<day>\d{1,2})$",
		r"^(?P<day>\d{1,2}).(?P<phase>\d)$",
		r"^(?P<day>\d{1,2})$",
	])
	.unwrap();

	// let matches: Option<> = set.matches(&puzzle).into_iter().collect::<Vec<usize>>().first();
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

fn parse_year(y: Option<&String>, Puzzle(cy, _, _): &Puzzle) -> Result<u16, &'static str> {
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

fn parse_day(d: Option<&String>, y: &u16, Puzzle(cy, cd, _): &Puzzle) -> Result<u8, &'static str> {
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

fn parse_phase(p: Option<&String>, Puzzle(_, _, cp): &Puzzle) -> Result<u8, &'static str> {
	match p {
		None => Ok(*cp),
		Some(p) => {
			let p = p.parse::<u8>().map_err(|_| "unable to parse phase")?;

			if p == 0 {
				return Err("phase starts at 1");
			}

			if p > 3 {
				return Err("phase stops at 3 (3 = run all)");
			}

			Ok(p)
		}
	}
}
