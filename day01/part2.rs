mod input;

use common::run;
use input::{parse_input, Input, DIAL_MOD, DIAL_START};

fn rotate_counting_zeros(a: i64, b: i64, m: i64) -> (i64, i64) {
	let result = (a + b).rem_euclid(m);
	let mut passes = (a + b).div_euclid(m).abs();

	if a == 0 && b < 0 {
		passes -= 1;
	}
	if result == 0 && b < 0 {
		passes += 1;
	}

	(result, passes)
}

fn solve_part2(input: Input) -> i64 {
	let mut dial_position = DIAL_START;
	let mut count = 0;
	for step in input {
		let (new_pos, passes) = rotate_counting_zeros(dial_position, step, DIAL_MOD);
		dial_position = new_pos;
		count += passes;
	}
	count
}

fn main() {
	run(parse_input, solve_part2);
}