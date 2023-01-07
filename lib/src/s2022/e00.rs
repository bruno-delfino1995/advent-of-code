use crate::prelude::*;

pub fn basic(_input: Input) -> String {
	String::from("unimplemented")
}

pub fn complex(_input: Input) -> String {
	String::from("unimplemented")
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			1
		"#
		);

		assert_eq!(basic(input), "2")
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			3
		"#
		);

		assert_eq!(complex(input), "2")
	}
}
