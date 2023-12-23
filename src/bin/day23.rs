use std::cmp::Ordering;
use std::collections::BinaryHeap;
use advent_of_code_2023::{von_neumann_compass};

fn main() {
    let input = parse(include_str!("../../data/day23/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn parse(raw_input: &str) -> Vec<Vec<char>> {
    raw_input
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn puzzle_1(data: &[Vec<char>]) -> usize {
    let graph = build_graph(data, false);

    find_longest_path(&graph, 1, graph.len() - 2)
}

fn puzzle_2(data: &[Vec<char>]) -> usize {
    let graph = build_graph(data, true);

    find_longest_path(&graph, 1, graph.len() - 2)
}

fn build_graph(data: &[Vec<char>], ignore_slopes: bool) -> Vec<Vec<Edge>> {
    let height = data.len();
    let width = data[0].len();
    let mut graph: Vec<Vec<Edge>> = Vec::with_capacity(width * height);

    for i in 0..height {
        for j in 0..width {
            let mut edges = Vec::new();

            match (data[i][j], ignore_slopes) {
                ('#', _) => {},
                ('^', false) => edges.push(Edge {position: (i - 1) * width + j, cost: 1}),
                ('v', false) => edges.push(Edge {position: (i + 1) * width + j, cost: 1}),
                ('<', false) => edges.push(Edge {position: i * width + (j - 1), cost: 1}),
                ('>', false) => edges.push(Edge {position: i * width + (j + 1), cost: 1}),
                _ => edges.extend(Vec::from_iter(
                    von_neumann_compass((i, j), height, width)
                        .into_iter()
                        .filter_map(|(_, (ny, nx))| {
                            return if data[ny][nx] == '#' {
                                None
                            } else {
                                Some(Edge {position: ny * width + nx, cost: 1})
                            }
                        })
                )),
            }

            graph.push(edges);
        }
    }

    for i in 0..graph.len() {
        if graph[i].len() == 0 || graph[i].len() == 2 {
            continue;
        }

        for j in 0..graph[i].len() {
            let mut cost = 1;
            let mut last_pos = i;
            let mut pos = graph[i][j].position;

            // Search for next intersection
            while graph[pos].len() == 2 {
                let new_pos = if graph[pos][0].position != last_pos { graph[pos][0].position } else { graph[pos][1].position };
                last_pos = pos;
                pos = new_pos;
                cost += 1;
            }

            graph[i][j].position = pos;
            graph[i][j].cost = cost;
        }
    }

    graph
}

fn find_longest_path(graph: &[Vec<Edge>], start: usize, end: usize) -> usize {
    let mut distance_map = DistanceMap::new(graph.len());
    let mut heap = BinaryHeap::new();

    graph[start]
        .iter()
        .for_each(|edge| {
            let path = Path::from_edge(edge);

            distance_map.add_path(&path);
            heap.push(path);
        });

    while let Some(path) = heap.pop() {
        if path.position == end {
            continue;
        }

        // if path.cost < distance_map.get_cost(&path) {
        //     continue;
        // }

        graph[path.position]
            .iter()
            .for_each(|edge| {
                if path.visited.contains(&edge.position) {
                    return;
                }

                let next_path = path.add_edge(edge);

                // if next_path.cost < distance_map.get_cost(&next_path) {
                //     return;
                // }

                distance_map.add_path(&next_path);
                heap.push(next_path);
            })
    }

    distance_map.distances[end]
}

struct Edge {
    position: usize,
    cost: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    position: usize,
    visited: Vec<usize>,
}

impl Path {
    fn from_edge(edge: &Edge) -> Self {
        Self {
            cost: edge.cost,
            position: edge.position,
            visited: Vec::from([edge.position]),
        }
    }

    fn add_edge(&self, edge: &Edge) -> Self {
        let mut visited= Vec::from_iter(self.visited.iter().map(|&p| p));
        visited.push(edge.position);

        Self {
            cost: self.cost + edge.cost,
            position: edge.position,
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
            distances: vec![0; graph_size],
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
        let input = parse(include_str!("../../data/day23/input_example.txt"));

        assert_eq!(puzzle_1(&input), 94);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day23/input.txt"));

        assert_eq!(puzzle_1(&input), 2318);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day23/input_example.txt"));

        assert_eq!(puzzle_2(&input), 154);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day23/input.txt"));

        assert_eq!(puzzle_2(&input), 6426);
    }
}