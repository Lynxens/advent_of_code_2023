use std::cmp::max;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

fn main() {
    let input = parse(include_str!("../../data/day2/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

fn parse(raw_input: &str) -> Vec<Vec<Vec<(&str,i32)>>>
{
    raw_input
        .split('\n')
        .map(|game| {
            let (_, draws) = game
                .split_once(": ")
                .unwrap();

            draws
                .split("; ")
                .map(|subset| {
                    subset
                        .split(", ")
                        .map(|color_count| {
                            let (count, color) = color_count
                                .split_once(' ')
                                .unwrap();

                            (color, count.parse::<i32>().unwrap())
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn puzzle_1(data: &[Vec<Vec<(&str,i32)>>]) -> i32 {
    data
        .iter()
        .enumerate()
        .filter(|&(_, game)| {
            game
                .iter()
                .all(|draws| {
                    draws
                        .iter()
                        .all(|&(color, count)| {
                            count <= match color {
                                "red" => Ok(12),
                                "green" => Ok(13),
                                "blue" => Ok(14),
                                _ => Err(Error::new(ErrorKind::InvalidData,"Unexpected color"))
                            }.unwrap()
                        })
                })
        })
        .map(|(game_id, _)| (game_id + 1) as i32)
        .sum()
}

fn puzzle_2(data: &[Vec<Vec<(&str,i32)>>]) -> i32 {
    data
        .iter()
        .map(|subsets| {
            let mut color_counts = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);

            subsets
                .iter()
                .for_each(|subset| {
                    subset
                        .iter()
                        .for_each(| &(color, count) | {
                            color_counts.insert(color, max(*color_counts.get(color).unwrap(), count));
                        })
                });

            color_counts.values().product::<i32>()
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day2/input_example.txt"));

        assert_eq!(puzzle_1(&input), 8);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day2/input.txt"));

        assert_eq!(puzzle_1(&input), 1734);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day2/input_example.txt"));

        assert_eq!(puzzle_2(&input), 2286);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day2/input.txt"));

        assert_eq!(puzzle_2(&input), 70387);
    }
}