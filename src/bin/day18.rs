use itertools::Itertools;
use nom::Slice;
use advent_of_code_2023::{Direction};
use advent_of_code_2023::Direction::{East, North, South, West};

fn main() {
    let input = parse(include_str!("../../data/day18/input.txt"), false);
    let input_hex = parse(include_str!("../../data/day18/input.txt"), true);

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input_hex));
}

#[derive(Copy, Clone)]
struct Instruction {
    direction: Direction,
    size: i64,
}

fn parse(raw_input: &str, use_hex: bool) -> Vec<Instruction>
{
    raw_input
        .lines()
        .map(|l| {
            if use_hex {
                let hex = l.split(' ').last().unwrap();
                let (size, direction) = hex.slice(2..hex.len() - 1).split_at(hex.len() - 4);

                Instruction {
                    direction: match direction {
                        "3" => North,
                        "2" => West,
                        "1" => South,
                        _ => East
                    },
                    size: i64::from_str_radix(size, 16).expect("Failed to convert size from hex to decimal"),
                }
            } else {
                let (direction, size, _) = l.splitn(3, ' ').collect_tuple().unwrap();

                Instruction {
                    direction: match direction {
                        "U" => North,
                        "D" => South,
                        "L" => West,
                        _ => East
                    },
                    size: size.parse::<i64>().unwrap(),
                }
            }
        })
        .collect()
}

fn puzzle_1(data: &[Instruction]) -> i64 {
    measure_trench_from_instructions(data)
}

fn puzzle_2(data: &[Instruction]) -> i64 {
    measure_trench_from_instructions(data)
}

fn measure_trench_from_instructions(instructions: &[Instruction]) -> i64 {
    instructions
        .iter()
        .fold((1, 0, 0), | (area, y, x), i | { // Start at 1 to include the origin point
            match i.direction {
                North => (area, y - i.size, x), // Do nothing
                South => (area + i.size, y + i.size, x), // Add the vertical line
                West => (area + (i.size * y), y, x + i.size), // Add the size of the area below the two coordinates
                East => (area - (i.size * (y - 1)), y, x - i.size), // Subtract the size of the area below the two coordinates without the top horizontal line
            }
        }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day18/input_example.txt"), false);

        assert_eq!(puzzle_1(&input), 62);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day18/input.txt"), false);

        assert_eq!(puzzle_1(&input), 49061);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day18/input_example.txt"), true);

        assert_eq!(puzzle_2(&input), 952408144115);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day18/input.txt"), true);

        assert_eq!(puzzle_2(&input), 92556825427032);
    }
}