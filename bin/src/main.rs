use lib;

use clap::Parser;

use std::fmt::Display;
use std::fs::File;
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

	let path = {
		let mut root = PathBuf::from("in");
		root.push(puzzle.year());
		root.push(format!("{:0>2}.{}.txt", puzzle.day(), puzzle.phase()));

		root
	};

	let reader = File::open(path)
		.map(BufReader::new)
		.map_err(|_| String::from("Input not found"))
		.unwrap_or_else(exit);

	let output = solutions
		.run(&puzzle, Box::new(reader))
		.unwrap_or_else(exit);

	println!("{}", output)
}

fn exit<T: Display, R>(err: T) -> R {
	eprintln!("{}", err);
	process::exit(1)
}

