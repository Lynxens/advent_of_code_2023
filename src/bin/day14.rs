use std::collections::HashMap;
use itertools::Itertools;
use advent_of_code_2023::*;

fn main() {
    let input = parse(include_str!("../../data/day14/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

fn parse(raw_input: &str) -> Vec<Vec<char>>
{
    raw_input
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn puzzle_1(data: &[Vec<char>]) -> usize {
    let mut platform = transpose(data);
    tilt_platform(&mut platform);

    calculate_load(&platform)
}

fn puzzle_2(data: &[Vec<char>]) -> usize {
    let (cycle, offset) = find_cycle(data);

    cycle[(1000000000 - offset) % cycle.len()]
}

fn find_cycle(data: &[Vec<char>]) -> (Vec<usize>, usize) {
    let mut platform = transpose(data); // West -> North
    let mut load_to_cycle_index: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut cycle: Vec<usize> = Vec::new();
    let mut cycle_index: usize = 0;

    loop {
        cycle_platform(&mut platform);

        let load = calculate_load(&platform);

        if let Some(cycle_indices) = load_to_cycle_index.get_mut(&load) {
            cycle_indices.push(cycle_index);

            if cycle_indices.len() == 3 {
                let diffs: Vec<usize> = cycle_indices
                    .iter()
                    .tuple_windows()
                    .map(|(&la, &lb)| lb - la)
                    .collect();

                if diffs.iter().all_equal() && diffs[0] != 1 {
                    return (cycle[cycle_indices[0]..cycle_indices[1]].to_vec(), cycle_indices[0] + 1);
                }
            }
        } else {
            load_to_cycle_index.insert(load, Vec::from([cycle_index]));
        }

        cycle.push(load);
        cycle_index += 1;
    }
}

fn cycle_platform(platform: &mut Vec<Vec<char>>) {
    tilt_platform(platform);

    *platform = transpose(platform); // North -> West
    tilt_platform(platform);

    *platform = reverse_rows(&transpose(platform)); // West -> South
    tilt_platform(platform);

    *platform = reverse_rows(&transpose(&reverse_rows(platform))); // South -> East
    tilt_platform(platform);

    *platform = transpose(&reverse_rows(platform)); // East -> North
}

fn tilt_platform(platform: &mut [Vec<char>]) {
    platform
        .iter_mut()
        .for_each(|line| {
            // Bubble sort rocks
            for i in 0..line.len() {
                for j in 0..line.len() - i - 1 {
                    if line[j] == '.' && line[j + 1] == 'O' {
                        line.swap(j, j + 1);
                    }
                }
            }
        });
}

fn calculate_load(platform: &[Vec<char>]) -> usize {
    platform
        .iter()
        .map(|line| line
            .iter()
            .zip((1..=platform.len()).rev())
            .filter_map(|(&c, i)| match c {
                'O' => Some(i),
                _ => None,
            })
            .sum::<usize>()
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day14/input_example.txt"));

        assert_eq!(puzzle_1(&input), 136);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day14/input.txt"));

        assert_eq!(puzzle_1(&input), 110274);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day14/input_example.txt"));

        assert_eq!(puzzle_2(&input), 64);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day14/input.txt"));

        assert_eq!(puzzle_2(&input), 90982);
    }
}