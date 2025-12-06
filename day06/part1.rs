use common::run;

use crate::input::{Expression, Input, Operation};

mod input;

pub fn parse_input(input_str: &str) -> Input {
    let mut line_iter = input_str.lines();
    let op_line = line_iter.next_back().expect("Could not take operator line");
    let mut line_iters = line_iter
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse::<usize>)
                .map(Result::unwrap)
        })
        .collect::<Vec<_>>();
    let operations = op_line
        .split_ascii_whitespace()
        .map(str::parse::<Operation>)
        .map(Result::unwrap);

    operations
        .into_iter()
        .map(|operation| {
            let inputs = line_iters
                .iter_mut()
                .map(Iterator::next)
                .map(Option::unwrap)
                .collect();

            Expression { operation, inputs }
        })
        .collect()
}

fn solve_part1(input: Input) -> i64 {
    input
        .into_iter()
        .map(|expr| {
            expr.inputs
                .into_iter()
                .reduce(|acc, val| expr.operation.call(acc, val))
                .unwrap()
        })
        .sum::<usize>() as i64
}

fn main() {
    run(parse_input, solve_part1);
}
