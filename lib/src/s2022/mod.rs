mod e01;
mod e02;
mod e03;
mod e04;
mod e05;
mod e06;
mod e07;
mod e08;
mod e09;
mod e10;
mod e11;
mod e12;
mod e13;

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
	register.insert(Puzzle::from("2022.08.1"), &e08::basic);
	register.insert(Puzzle::from("2022.08.2"), &e08::complex);
	register.insert(Puzzle::from("2022.09.1"), &e09::basic);
	register.insert(Puzzle::from("2022.09.2"), &e09::complex);
	register.insert(Puzzle::from("2022.10.1"), &e10::basic);
	register.insert(Puzzle::from("2022.10.2"), &e10::complex);
	register.insert(Puzzle::from("2022.11.1"), &e11::basic);
	register.insert(Puzzle::from("2022.11.2"), &e11::complex);
	register.insert(Puzzle::from("2022.12.1"), &e12::basic);
	register.insert(Puzzle::from("2022.12.2"), &e12::complex);
	register.insert(Puzzle::from("2022.13.1"), &e13::basic);
	register.insert(Puzzle::from("2022.13.2"), &e13::complex);

	register
}
