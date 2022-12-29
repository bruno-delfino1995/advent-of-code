mod e01;

use crate::prelude::*;

pub fn solutions() -> Register {
	let mut register = Register::new();

	register.insert(Puzzle::from("2022.01.1"), &e01::basic);
	register.insert(Puzzle::from("2022.01.2"), &e01::complex);

	register
}
