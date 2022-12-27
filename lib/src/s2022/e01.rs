use crate::prelude::*;

pub fn first(input: Box<dyn BufRead>) -> String {
	lines(input).collect()
}
