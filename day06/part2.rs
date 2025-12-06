use common::run;

use crate::input::{Expression, Input, Operation};

mod input;

fn split_at_indices<'a>(line: &'a str, indices: &[usize]) -> impl Iterator<Item = &'a str> {
    indices
        .iter()
        .enumerate()
        .map(|(i, idx)| {
            let end = if i < indices.len() - 1 {
                indices[i + 1] - 1
            } else {
                line.len()
            };
            *idx..end
        })
        .map(|range| line.get(range).unwrap())
}

pub fn parse_input(input_str: &str) -> Input {
    let mut line_iter = input_str.lines();
    let op_line = line_iter.next_back().expect("Could not take operator line");
    let (op_indices, operations): (Vec<_>, Vec<_>) = op_line
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_ascii_whitespace())
        .map(|(i, c)| (i, Operation::try_from(c).unwrap()))
        .unzip();

    let mut line_iters = line_iter
        .map(|line| split_at_indices(line, &op_indices))
        .collect::<Vec<_>>();

    operations
        .into_iter()
        .map(|operation| {
            let lines = line_iters
                .iter_mut()
                .map(Iterator::next)
                .map(Option::unwrap)
                .collect::<Vec<_>>();

            // All lines have the same length.
            let max_len = lines[0].len();

            let inputs = (0..max_len)
                .rev()
                .map(|idx| {
                    let digits = lines
                        .iter()
                        .map(|line| line.get(idx..(idx + 1)).unwrap_or(""))
                        .filter(|c| c != &" ")
                        .collect::<Vec<_>>()
                        .join("");
                    digits.parse().unwrap()
                })
                .collect();

            Expression { operation, inputs }
        })
        .collect()
}

fn solve_part2(input: Input) -> i64 {
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
    run(parse_input, solve_part2);
}
