use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::cmp::Reverse;

use common::run_with_args;
use common::space::vector::Vector3i;
use priority_queue::PriorityQueue;

mod input;
mod octree;
use crate::input::{Input, parse_args, parse_input};
use crate::octree::Octree;

const TOP_COMPONENTS: usize = 3;

fn calculate_edge_map(
    connection_count: usize,
    points: Vec<Vector3i>,
) -> HashMap<Vector3i, HashSet<Vector3i>> {
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

    for _ in 0..connection_count {
        let ((a, b), _) = closest_map.pop().unwrap();
        let _ = closest_map.pop().unwrap();
        // Closest edges will be the top two node matches.

        edge_map.get_mut(a).unwrap().insert(b.clone());
        edge_map.get_mut(b).unwrap().insert(a.clone());

        let a_next = nearest_iters.get_mut(a).unwrap().next().unwrap();
        let b_next = nearest_iters.get_mut(b).unwrap().next().unwrap();

        closest_map.push((a, a_next.0), Reverse(a_next.1));
        closest_map.push((b, b_next.0), Reverse(b_next.1));
    }

    edge_map
}

fn resolve_forest(edge_map: &HashMap<Vector3i, HashSet<Vector3i>>) -> Vec<HashSet<Vector3i>> {
    let mut visited: HashSet<Vector3i> = HashSet::default();
    let mut forests: Vec<HashSet<Vector3i>> = vec![];

    for start_point in edge_map.keys() {
        if visited.contains(start_point) {
            continue;
        }

        let mut stack: Vec<&Vector3i> = vec![start_point];
        let mut current_forest: HashSet<Vector3i> = HashSet::default();

        while let Some(point) = stack.pop() {
            if visited.contains(point) {
                continue;
            }
            visited.insert(point.clone());
            current_forest.insert(point.clone());

            for neighbor in edge_map.get(point).unwrap() {
                if !visited.contains(neighbor) {
                    stack.push(neighbor);
                }
            }
        }

        forests.push(current_forest);
    }

    forests
}

fn solve_part1(connection_count: usize, points: Input) -> i64 {
    let edge_map = calculate_edge_map(connection_count, points);
    let mut forests = resolve_forest(&edge_map);
    forests.sort_by_key(|g| g.len());

    forests
        .into_iter()
        .rev()
        .take(TOP_COMPONENTS)
        .map(|g| g.len() as i64)
        .product()
}

fn main() {
    run_with_args(parse_args, parse_input, solve_part1);
}
