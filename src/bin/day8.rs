use std::collections::{HashMap};
use nom::Slice;
use advent_of_code_2023::lcm;

fn main() {
    let input = parse(include_str!("../../data/day8/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

fn parse(raw_input: &str) -> (Vec<usize>, HashMap<&str, [&str; 2]>) {
    let (first_line, rest) = raw_input.split_once("\n\n").unwrap();

    let directions = first_line
        .chars()
        .map(|c| match c {
            'L' => Ok(0),
            'R' => Ok(1),
            _ => Err("Unexpect char"),
        }.expect("Unexpected char"))
        .collect();

    let node_map = HashMap::from_iter(rest
        .lines()
        .map(|l| {
            let (node, options) = l.split_once(" = ").unwrap();
            let (left, right) = options.slice(1..options.len() - 1).split_once(", ").unwrap();
            (node, [left, right])
        }));

    (
        directions,
        node_map,
    )
}

fn puzzle_1((directions, node_map): &(Vec<usize>, HashMap<&str, [&str; 2]>)) -> i128 {
    directions
        .iter()
        .cycle()
        .scan(("AAA", 0), | (node, steps), &direction | {
            *node = node_map.get(node).unwrap()[direction];
            *steps += 1;
            Some((*node, *steps))
        })
        .find(|&(node, _) | node == "ZZZ")
        .unwrap()
        .1
}

fn puzzle_2((directions, node_map): &(Vec<usize>, HashMap<&str, [&str; 2]>)) -> i128 {
    node_map
        .clone()
        .into_keys()
        .filter(|&n| n.ends_with('A'))
        .filter_map(|n| {
            let mut steps = 0;
            let mut node = n;
            let mut endings = HashMap::new();

            for (instruction_index, &direction) in directions.iter().enumerate().cycle() {
                node = node_map.get(node).unwrap()[direction];
                steps += 1;

                if node.ends_with('Z') {
                    match endings.get(&instruction_index) {
                        None => endings.insert(instruction_index, steps),
                        Some(&steps_to_ending) => return Some(steps - steps_to_ending),
                    };
                }
            }

            None
        })
        .reduce(|least_common_multiple, cycle_length| {
            lcm(least_common_multiple, cycle_length)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day8/input_example.txt"));

        assert_eq!(puzzle_1(&input), 6);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day8/input.txt"));

        assert_eq!(puzzle_1(&input), 18023);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day8/input_example_2.txt"));

        assert_eq!(puzzle_2(&input), 6);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day8/input.txt"));

        assert_eq!(puzzle_2(&input), 14449445933179);
    }
}