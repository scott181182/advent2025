use std::collections::{HashMap, HashSet};

use common::run_with_args;
use common::space::vector::Vector3i;
use indexmap::IndexMap;

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
    let mut edge_map: HashMap<Vector3i, HashSet<Vector3i>> = points
        .iter()
        .map(|p| (p.clone(), vec![p.clone()].into_iter().collect()))
        .collect();
    let mut closest_map: IndexMap<Vector3i, (&Vector3i, usize)> = points
        .iter()
        .map(|p| {
            (
                p.clone(),
                ot.find_closest_point(p, edge_map.get(p).unwrap()).unwrap(),
            )
        })
        .collect();

    for _ in 0..connection_count {
        closest_map.sort_by(|_, a, _, b| a.1.cmp(&b.1));

        let (a, b) = {
            let (a, (b, _)) = closest_map.first().unwrap();

            let a = (*a).clone();
            let b = (*b).clone();

            (a, b)
        };

        edge_map.get_mut(&a).unwrap().insert(b.clone());
        edge_map.get_mut(&b).unwrap().insert(a.clone());

        closest_map.insert(
            a.clone(),
            ot.find_closest_point(&a, edge_map.get(&a).unwrap())
                .unwrap(),
        );
        closest_map.insert(
            b.clone(),
            ot.find_closest_point(&b, edge_map.get(&b).unwrap())
                .unwrap(),
        );
    }

    edge_map
}

fn resolve_forest(edge_map: &HashMap<Vector3i, HashSet<Vector3i>>) -> Vec<HashSet<Vector3i>> {
    let mut visited: HashSet<Vector3i> = HashSet::new();
    let mut forests: Vec<HashSet<Vector3i>> = vec![];

    for start_point in edge_map.keys() {
        if visited.contains(start_point) {
            continue;
        }

        let mut stack: Vec<&Vector3i> = vec![start_point];
        let mut current_forest: HashSet<Vector3i> = HashSet::new();

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
