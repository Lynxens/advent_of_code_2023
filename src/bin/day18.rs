use std::cmp::Ordering;
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

struct Instruction {
    direction: Direction,
    size: i64,
}

type Trench = Vec<(i64, i64)>;

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
    let trench = dig_trench(data);
    measure_trench(&trench)
}

fn puzzle_2(data: &[Instruction]) -> i64 {
    let trench = dig_trench(data);
    measure_trench(&trench)
}

fn dig_trench(instructions: &[Instruction]) -> Trench {
    let mut coords: Vec<(i64, i64)> = Vec::new();
    let mut x = 0;
    let mut y = 0;

    coords.push((y, x));

    for instruction in instructions.iter() {
        match instruction.direction {
            North => y -= instruction.size,
            South => y += instruction.size,
            East => x += instruction.size,
            West => x -= instruction.size,
        };

        coords.push((y, x));
    }

    coords
}

fn measure_trench(trench: &Trench) -> i64 {
    let area: i64 = trench
        .iter()
        .tuple_windows()
        .map(|((y0, x0), (y1, x1))| {
            let dx = (x1 - x0).abs();
            let dy = (y1 - y0).abs();

            match (x1.cmp(&x0), y1.cmp(&y0)) {
                (Ordering::Greater, _) => dx * y0, // West: Add area below coordinates
                (Ordering::Less, _) => -dx * (y0 + 1), // East: Subtract area below coordinates and the horizontal line above
                (_, Ordering::Less) => -dy, // North: Subtract vertical line
                _ => 0, // South: Do nothing
            }
        })
        .sum::<i64>()
        .abs();

    area + 1 // +1 for origin point
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