use std::env;
use std::fmt::Display;
use std::process;
use std::str::FromStr;

use lib::Puzzle;
use lib::Solutions;

fn main() {
	let solutions = Solutions::load();
	let args: Vec<String> = env::args().collect();

	// FIXME: It's unsafely trusting the args length
	match args[1].as_str() {
		"list" => list(solutions),
		"exec" => solve(solutions, &args[2]),
		_ => exit("Unsupported")
	}
}

fn list(_solutions: Solutions) {
	exit("Not implemented")
}

fn solve(solutions: Solutions, puzzle: &str) {
	let puzzle = Puzzle::from_str(puzzle).unwrap_or_default();

	let reader = std::io::stdin().lock();

	let output = solutions
		.run(&puzzle, Box::new(reader))
		.unwrap_or_else(exit);

	println!("{}", output)
}

fn exit<T: Display, R>(err: T) -> R {
	eprintln!("{}", err);
	process::exit(1)
}
