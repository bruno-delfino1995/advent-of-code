use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn isize(s: &str) -> IResult<&str, isize> {
	map_res(digit1, str::parse)(s)
}

pub fn usize(s: &str) -> IResult<&str, usize> {
	map_res(digit1, str::parse)(s)
}
