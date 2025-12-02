mod input;

use common::run;
use input::{parse_input, Input};

fn find_invalid_ids(start: usize, end: usize) -> Vec<usize> {
    let start_str = start.to_string();

    let mut candidate_half = if start_str.len() & 1 == 0 {
        start_str[..start_str.len() / 2].parse::<usize>().unwrap()
    } else {
        // Just an optimization to cut out the odd lengths
        10_usize.pow((start_str.len() / 2) as u32)
    };

    let mut invalid_ids = Vec::new();

    let mut candidate = format!("{}{}", candidate_half, candidate_half).parse::<usize>().unwrap();
    while candidate <= end {
        if candidate >= start {
            invalid_ids.push(candidate);
        }

        candidate_half += 1;
        candidate = format!("{}{}", candidate_half, candidate_half).parse::<usize>().unwrap();
    }

    invalid_ids
}

fn solve_part1(input: Input) -> i64 {
	input.into_iter()
		.flat_map(|(start, end)| find_invalid_ids(start, end))
		.sum::<usize>() as i64
}

fn main() {
	run(parse_input, solve_part1);
}
