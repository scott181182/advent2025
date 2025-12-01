mod input;

use common::run;
use input::{parse_input, Input, DIAL_MOD, DIAL_START};

fn solve_part1(input: Input) -> i64 {
	let mut dial_position = DIAL_START;
	let mut count = 0;

	for step in input {
		dial_position = (dial_position + step) % DIAL_MOD;
		if dial_position == 0 {
			count += 1;
		}
	}
	count
}

fn main() {
	run(parse_input, solve_part1);
}
