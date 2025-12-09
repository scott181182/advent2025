use std::cmp::Reverse;

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

use common::run;
use common::space::octree::Octree;
use common::space::vector::Vector3i;
use priority_queue::PriorityQueue;

mod input;
use crate::input::{Input, parse_input};

fn solve_part2(points: Input) -> i64 {
    let ot = Octree::from_vec(points.clone());
    let mut nearest_iters = points
        .iter()
        .map(|p| (p, ot.closest_points(p.clone())))
        .collect::<HashMap<_, _>>();
    let mut edge_map: HashMap<Vector3i, HashSet<Vector3i>> = points
        .iter()
        .map(|p| (p.clone(), vec![p.clone()].into_iter().collect()))
        .collect();
    let mut closest_map: PriorityQueue<(&Vector3i, &Vector3i), Reverse<usize>> = points
        .iter()
        .map(|p| {
            let (q, d) = nearest_iters.get_mut(p).unwrap().next().unwrap();
            ((p, q), Reverse(d))
        })
        .collect();

    let mut main_graph = HashSet::default();
    let mut last_connection = None;

    while main_graph.len() < points.len() {
        // Closest edges will be the top two node matches.
        let ((a, b), _) = closest_map.pop().unwrap();
        let _ = closest_map.pop().unwrap();

        edge_map.get_mut(a).unwrap().insert(b.clone());
        edge_map.get_mut(b).unwrap().insert(a.clone());

        let a_next = nearest_iters.get_mut(a).unwrap().next().unwrap();
        let b_next = nearest_iters.get_mut(b).unwrap().next().unwrap();

        closest_map.push((a, a_next.0), Reverse(a_next.1));
        closest_map.push((b, b_next.0), Reverse(b_next.1));

        // Insert into main graph
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
