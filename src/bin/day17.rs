use std::cmp::{Ordering};
use std::collections::{BinaryHeap};
use advent_of_code_2023::{coordinate_steps_in_direction, Direction};
use advent_of_code_2023::Direction::{East, North, South, West};

fn main() {
    let input = parse(include_str!("../../data/day17/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn parse(raw_input: &str) -> Vec<Vec<usize>> {
    raw_input
        .lines()
        .map(|l| l
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        )
        .collect()
}

fn puzzle_1(data: &[Vec<usize>]) -> usize {
    let graph = build_graph(data, 1, 3);

    find_shortest_path(&graph, 0, graph.len() - 1).unwrap()
}

fn puzzle_2(data: &[Vec<usize>]) -> usize {
    let graph = build_graph(data, 4, 10);

    find_shortest_path(&graph, 0, graph.len() - 1).unwrap()
}

fn build_graph(data: &[Vec<usize>], min_steps: usize, max_steps: usize) -> Vec<Vec<Edge>> {
    let height = data.len();
    let width = data[0].len();
    let mut graph = Vec::with_capacity(width * height);

    for i in 0..height {
        for j in 0..width {
            let mut edges = Vec::new();

            for direction in [North, East, South, West] {
                let mut cost_sum = 0;
                for offset in 1..=max_steps {
                    if let Some((y, x)) = coordinate_steps_in_direction(&(i, j), offset, &direction, height, width) {
                        cost_sum += data[y][x];

                        if offset >= min_steps {
                            edges.push(Edge { position: y * width + x, cost: cost_sum, direction })
                        }
                    } else {
                        break
                    }
                }
            }

            graph.push(edges);
        }
    }

    graph
}

fn find_shortest_path(graph: &[Vec<Edge>], start: usize, end: usize) -> Option<usize> {
    let mut distance_map = DistanceMap::new(graph.len());
    let mut heap = BinaryHeap::new();

    graph[start]
        .iter()
        .for_each(|edge| {
            let path = Path::from_edge(edge);

            heap.push(path);
            distance_map.add_path(&path);
        });

    while let Some(path) = heap.pop() {
        if path.position == end {
            return Some(path.cost);
        }

        if path.cost > distance_map.get_cost(&path) {
            continue;
        }

        graph[path.position]
            .iter()
            .for_each(|edge| {
                if edge.direction == path.direction || edge.direction == path.direction.opposite() {
                    return;
                }

                let next_path = path.add_edge(edge);

                if next_path.cost >= distance_map.get_cost(&next_path) {
                    return;
                }

                heap.push(next_path);
                distance_map.add_path(&next_path);
            })
    }

    None
}

struct Edge {
    position: usize,
    cost: usize,
    direction: Direction,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    position: usize,
    direction: Direction,
}

impl Path {
    fn from_edge(edge: &Edge) -> Self {
        Self {
            cost: edge.cost,
            position: edge.position,
            direction: edge.direction,
        }
    }

    fn add_edge(&self, edge: &Edge) -> Self {
        Self {
            cost: self.cost + edge.cost,
            position: edge.position,
            direction: edge.direction,
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
    distances: Vec<[usize; 4]>,
}

impl DistanceMap {
    fn new(graph_size: usize) -> Self {
        Self {
            distances: vec![[usize::MAX, usize::MAX, usize::MAX, usize::MAX]; graph_size],
        }
    }

    fn get_cost(&self, path: &Path) -> usize {
        self.distances[path.position][path.direction as usize]
    }

    fn add_path(&mut self, path: &Path) {
        self.distances[path.position][path.direction as usize] = path.cost;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day17/input_example.txt"));

        assert_eq!(puzzle_1(&input), 102);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day17/input.txt"));

        assert_eq!(puzzle_1(&input), 1004);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day17/input_example.txt"));

        assert_eq!(puzzle_2(&input), 94);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day17/input.txt"));

        assert_eq!(puzzle_2(&input), 1171);
    }
}