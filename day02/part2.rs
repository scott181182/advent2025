use std::collections::HashSet;

mod input;

use common::run;
use input::{parse_input, Input};

fn explore_patterns(start: usize, end: usize, pattern_length: usize) -> HashSet<usize> {
    let start_str = start.to_string();
    let start_length = start_str.len();
    let end_length = end.to_string().len();

    (start_length..=end_length)
        .filter(|&n| n % pattern_length == 0 && n > pattern_length)
        .map(|n| n / pattern_length)
        .flat_map(|pattern_repetitions| {
            let mut invalid_ids = vec![];

            let (mut candidate_pattern, mut candidate_int) = if pattern_repetitions * pattern_length == start_length {
                let pat = &start_str[..pattern_length];
                (pat.to_string(), pat.parse::<usize>().unwrap())
            } else {
                let pat = format!("1{}", "0".repeat(pattern_length - 1));
                (pat.clone(), pat.parse::<usize>().unwrap())
            };

            let mut candidate = candidate_pattern.repeat(pattern_repetitions).parse::<usize>().unwrap();

            while candidate <= end {
                if candidate >= start {
                    invalid_ids.push(candidate);
                }

                candidate_int += 1;
                candidate_pattern = candidate_int.to_string();
                if candidate_pattern.len() > pattern_length {
                    break;
                }
                candidate = candidate_pattern.repeat(pattern_repetitions).parse::<usize>().unwrap();
            }
            invalid_ids
        })
        .collect()
}

fn find_invalid_ids(start: usize, end: usize) -> HashSet<usize> {
    let end_str = end.to_string();
    let longest_pattern = end_str.len() / 2;

    (1..=longest_pattern).into_iter()
        .flat_map(|pattern_length| explore_patterns(start, end, pattern_length))
        .collect()
}

fn solve_part2(id_ranges: Input) -> i64 {
	id_ranges.into_iter()
		.flat_map(|(start, end)| find_invalid_ids(start, end))
		.sum::<usize>() as i64
}

fn main() {
	run(parse_input, solve_part2);
}
