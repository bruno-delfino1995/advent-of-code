use crate::prelude::*;

solution!("0.0.0", sample(input) {
	lines(input).collect::<Vec<String>>().join("\n")
});

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	fn input() -> Input {
		input!(
			r#"
			an intricate input that
			has preceding spaces
			removed when constructing
		"#
		)
	}

	#[test]
	fn helpers() {
		assert_eq!(sample::solution(input()), all(input()).trim_end())
	}
}
