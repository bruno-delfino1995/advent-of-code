use crate::prelude::*;

pub fn first(input: Box<dyn BufRead>) -> String {
	let mut max: usize = 0;
	let mut acc = 0;

	for line in lines(input) {
		if line.is_empty() {
			acc = 0;
			continue
		}

		let calories: usize = line.parse().unwrap();
		acc += calories;

		if acc > max {
			max = acc;
		}
	}

	max.to_string()
}

#[cfg(test)]
mod test {
	use crate::input;
	use super::*;

	#[test]
	fn example() {
		let input = input!(r#"
			1000
			2000
			3000

			4000

			5000
			6000

			7000
			8000
			9000

			10000
		"#);


		assert_eq!(first(input), "24000");
	}
}
