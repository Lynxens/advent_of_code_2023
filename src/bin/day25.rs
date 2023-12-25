use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use itertools::Itertools;

fn main() {
    let input = parse(include_str!("../../data/day25/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
}

type WiringDiagram = Vec<HashSet<usize>>;

fn parse(raw_input: &str) -> WiringDiagram
{
    let mut map: HashMap<&str, HashSet<&str>> = raw_input
        .lines()
        .map(|l| {
            let (component, connections) = l.split_once(": ").unwrap();

            (component, connections.split(' ').collect::<HashSet<&str>>())
        })
        .collect();

    map.clone()
        .into_iter()
        .for_each(|(n, connections)| {
            for c in connections {
                if let Some(cn) = map.get_mut(&c) {
                    cn.insert(n);
                } else {
                    map.insert(c, HashSet::from([n]));
                }
            }
        });

    map
        .values()
        .map(|connections| connections
            .iter()
            .map(|&c| map.keys().position(|&k| c == k).unwrap())
            .collect::<HashSet<usize>>()
        )
        .collect()
}

fn puzzle_1(data: &WiringDiagram) -> usize {

    let mut edge_frequencies: HashMap<(usize, usize), usize> = Default::default();

    for (from, to) in (0..data.len()).tuple_combinations().take(10000) {
        for (a, b) in find_shortest_path(data, from, to).unwrap().visited.into_iter().tuple_windows() {
            let edge: (usize, usize) = (min(a, b), max(a, b));

            if let Some(count) = edge_frequencies.get_mut(&edge) {
                *count += 1;
            } else {
                edge_frequencies.insert(edge, 1);
            }
        }
    }

    let removed_edges: Vec<(usize, usize)> = edge_frequencies
        .iter()
        .sorted_by(|&(_, count_a), &(_, count_b)| count_b.cmp(count_a))
        .take(3)
        .map(|(&e, _)| e)
        .collect();

    let mut group_1 = HashSet::new();
    let mut queue = VecDeque::from([removed_edges[0].0]);

    while let Some(node) = queue.pop_front() {
        if !group_1.insert(node) {
            continue;
        }

        if let Some(connections) = data.get(node) {
            if connections.len() == 2 {
                continue;
            }

            for &connection in connections {
                if !group_1.contains(&connection) && !removed_edges.contains(&(min(node, connection), max(node, connection))) {
                    queue.push_back(connection);
                }
            }
        }
    }

    group_1.len() * (data.len() - group_1.len())
}

fn find_shortest_path(graph: &[HashSet<usize>], start: usize, end: usize) -> Option<Path> {
    let mut distance_map = DistanceMap::new(graph.len());
    let mut heap = BinaryHeap::new();

    graph[start]
        .iter()
        .for_each(|&edge| {
            let path = Path::from_edge(edge, start);

            distance_map.add_path(&path);
            heap.push(path);
        });

    while let Some(path) = heap.pop() {
        if path.position == end {
            return Some(path);
        }

        if path.cost > distance_map.get_cost(&path) {
            continue;
        }

        graph[path.position]
            .iter()
            .for_each(|&edge| {
                let next_path = path.add_edge(edge);

                if next_path.cost > distance_map.get_cost(&next_path) {
                    return;
                }

                distance_map.add_path(&next_path);
                heap.push(next_path);
            })
    }

    None
}


#[derive(Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    position: usize,
    visited: Vec<usize>,
}

impl Path {
    fn from_edge(edge: usize, from: usize) -> Self {
        Self {
            cost: 1,
            position: edge,
            visited: Vec::from([from, edge]),
        }
    }

    fn add_edge(&self, edge: usize) -> Self {
        let mut visited= self.visited.clone();
        visited.push(edge);

        Self {
            cost: self.cost + 1,
            position: edge,
            visited,
        }
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DistanceMap {
    distances: Vec<usize>,
}

impl DistanceMap {
    fn new(graph_size: usize) -> Self {
        Self {
            distances: vec![usize::MAX; graph_size],
        }
    }

    fn get_cost(&self, path: &Path) -> usize {
        self.distances[path.position]
    }

    fn add_path(&mut self, path: &Path) {
        self.distances[path.position] = path.cost;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day25/input_example.txt"));

        assert_eq!(puzzle_1(&input), 54);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day25/input.txt"));

        assert_eq!(puzzle_1(&input), 552682);
    }
}