use common::grid::Grid;
use common::run;

mod input;
use crate::input::{Element, Input, parse_input};

pub fn run_beam(manifold: &Input, beam_grid: &mut Grid<usize>) {
    for i in 0..manifold.height {
        for j in 0..manifold.width {
            let cell = &manifold[(i, j)];

            if cell != &Element::Empty {
                continue;
            }

            let upstream = if i > 0 {
                beam_grid[(i - 1, j)] + (manifold[(i - 1, j)] == Element::Start) as usize
            } else {
                0
            };
            let left_split = if i > 0 && j > 0 && manifold[(i, j - 1)] == Element::Splitter {
                beam_grid[(i - 1, j - 1)]
            } else {
                0
            };
            let right_split =
                if i > 0 && j < manifold.width - 1 && manifold[(i, j + 1)] == Element::Splitter {
                    beam_grid[(i - 1, j + 1)]
                } else {
                    0
                };

            beam_grid[(i, j)] += upstream + left_split + right_split;
        }
    }
}

fn solve_part1(manifold: Input) -> i64 {
    let mut beam_grid: Grid<usize> = Grid::defaults((manifold.height, manifold.width));
    run_beam(&manifold, &mut beam_grid);

    eprintln!("{beam_grid:?}");
    beam_grid.row(beam_grid.height - 1).iter().sum::<usize>() as i64
}

fn main() {
    run(parse_input, solve_part1);
}
