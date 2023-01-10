use clap::Parser;

use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::process;

use std::{io::BufReader, path::PathBuf};

use lib::{Puzzle, Solutions};

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
	let solutions = Solutions::load();

	let args = Args::parse();
	let puzzle = args.puzzle.unwrap_or_default();

	let reader = open_input(&puzzle).unwrap_or_else(exit);

	let output = solutions
		.run(&puzzle, Box::new(reader))
		.unwrap_or_else(exit);

	println!("{}", output)
}

fn open_input(puzzle: &Puzzle) -> Result<Box<dyn BufRead>, String> {
	let day = puzzle.day();
	let year = puzzle.year();
	let phase = puzzle.phase();

	let asked_phase_path = {
		let mut root = PathBuf::from("in");
		root.push(year.to_string());
		root.push(format!("{:0>2}.{}.txt", day, phase));

		root
	};

	let previous_phase_path = {
		let mut root = PathBuf::from("in");
		root.push(year.to_string());
		root.push(format!("{:0>2}.{}.txt", day, phase.saturating_sub(1)));

		root
	};

	let day_path = {
		let mut root = PathBuf::from("in");
		root.push(year.to_string());
		root.push(format!("{:0>2}.txt", day));

		root
	};

	let reader = File::open(asked_phase_path)
		.or_else(|_| File::open(previous_phase_path))
		.or_else(|_| File::open(day_path))
		.map_err(|_| "input not found".to_string())
		.map(BufReader::new)?;

	Ok(Box::new(reader))
}

fn exit<T: Display, R>(err: T) -> R {
	eprintln!("{}", err);
	process::exit(1)
}
