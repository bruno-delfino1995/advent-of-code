mod e01;
mod e02;
mod e03;
mod e04;
mod e05;
mod e06;
mod e07;

use crate::prelude::*;

pub fn solutions() -> Register {
	let mut register = Register::new();

	register.insert(Puzzle::from("2022.01.1"), &e01::basic);
	register.insert(Puzzle::from("2022.01.2"), &e01::complex);
	register.insert(Puzzle::from("2022.02.1"), &e02::basic);
	register.insert(Puzzle::from("2022.02.2"), &e02::complex);
	register.insert(Puzzle::from("2022.03.1"), &e03::basic);
	register.insert(Puzzle::from("2022.03.2"), &e03::complex);
	register.insert(Puzzle::from("2022.04.1"), &e04::basic);
	register.insert(Puzzle::from("2022.04.2"), &e04::complex);
	register.insert(Puzzle::from("2022.05.1"), &e05::basic);
	register.insert(Puzzle::from("2022.05.2"), &e05::complex);
	register.insert(Puzzle::from("2022.06.1"), &e06::basic);
	register.insert(Puzzle::from("2022.06.2"), &e06::complex);
	register.insert(Puzzle::from("2022.07.1"), &e07::basic);
	register.insert(Puzzle::from("2022.07.2"), &e07::complex);

	register
}
