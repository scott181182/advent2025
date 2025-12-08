use std::collections::{HashMap, HashSet};

use common::run;
use common::space::vector::Vector3i;
use indexmap::IndexMap;

mod input;
mod octree;
use crate::input::{Input, parse_input};
use crate::octree::Octree;

fn solve_part2(points: Input) -> i64 {
    let ot = Octree::from_vec(points.clone());
    let mut edge_map: HashMap<&Vector3i, HashSet<&Vector3i>> = points
        .iter()
        .map(|p| (p, vec![p].into_iter().collect()))
        .collect();
    let mut closest_map: IndexMap<&Vector3i, (&Vector3i, usize)> = points
        .iter()
        .map(|p| {
            (
                p,
                ot.find_closest_point(p, edge_map.get(p).unwrap()).unwrap(),
            )
        })
        .collect();

    let mut main_graph = HashSet::new();
    let mut last_connection = None;

    while main_graph.len() < points.len() {
        closest_map.sort_by(|_, a, _, b| a.1.cmp(&b.1));

        let (a, b) = {
            let (a, (b, _)) = closest_map.first().unwrap();

            let a = *a;
            let b = *b;

            (a, b)
        };

        edge_map.get_mut(&a).unwrap().insert(b);
        edge_map.get_mut(&b).unwrap().insert(a);

        closest_map.insert(
            a,
            ot.find_closest_point(a, edge_map.get(a).unwrap()).unwrap(),
        );
        closest_map.insert(
            b,
            ot.find_closest_point(b, edge_map.get(b).unwrap()).unwrap(),
        );

        main_graph.insert(a);
        main_graph.insert(b);

        last_connection = Some((a, b));
    }

    let last_connection = last_connection.unwrap();
    last_connection.0.x * last_connection.1.x
}

fn main() {
    run(parse_input, solve_part2);
}
