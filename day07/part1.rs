use common::run;

mod input;
use crate::input::{Element, Input, parse_input};

pub fn run_beam(manifold: &mut Input) {
    for i in 0..manifold.height {
        for j in 0..manifold.width {
            let cell = &manifold[(i, j)];

            if cell != &Element::Empty {
                continue;
            }

            let is_downwind = i > 0
                && (manifold[(i - 1, j)] == Element::Beam
                    || manifold[(i - 1, j)] == Element::Start);
            let is_split_from_left = i > 0
                && j > 0
                && manifold[(i, j - 1)] == Element::Splitter
                && manifold[(i - 1, j - 1)] == Element::Beam;
            let is_split_from_right = i > 0
                && j < manifold.width - 1
                && manifold[(i, j + 1)] == Element::Splitter
                && manifold[(i - 1, j + 1)] == Element::Beam;

            if is_downwind || is_split_from_left || is_split_from_right {
                manifold[(i, j)] = Element::Beam;
            }
        }
    }
}

fn count_splits(manifold: &Input) -> usize {
    manifold
        .windows((2, 1))
        .map(|w| {
            if w[(1, 0)] == Element::Splitter && w[(0, 0)] == Element::Beam {
                1
            } else {
                0
            }
        })
        .sum()
}

fn solve_part1(mut manifold: Input) -> i64 {
    run_beam(&mut manifold);
    count_splits(&manifold) as i64
}

fn main() {
    run(parse_input, solve_part1);
}
