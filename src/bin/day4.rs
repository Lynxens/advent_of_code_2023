use std::collections::HashSet;
use regex::Regex;

fn main() {
    let input = parse(include_str!("../../data/day4/input.txt"));

    puzzle_1(input);
    puzzle_2(input);
}

fn parse(raw_input: &str) -> &str {
    raw_input
}

fn puzzle_1(data: &str) -> i32 {
    get_correct_counts(data)
        .iter()
        .filter_map(|count| match count {
            0 => None,
            _ => Some(2i32.pow((count - 1) as u32)),
        })
        .sum()
}

fn puzzle_2(data: &str) -> i32 {
    let mut card_counts = vec![1; data.lines().count()];

    get_correct_counts(data)
        .iter()
        .enumerate()
        .for_each(|(i, count)| {
            for j in i + 1 ..= i + count {
                card_counts[j] += card_counts[i]
            }
        });

    card_counts.iter().sum()
}

fn get_correct_counts(data: &str) -> Vec<usize> {
    Regex::new(r": +(?P<winning>(?:\d+ +)+\d+) +\| +(?P<received>(?:\d+ +)+\d+)")
        .unwrap()
        .captures_iter(data)
        .map(|c| {
            let winning_numbers: HashSet<i32> = HashSet::from_iter(
                c
                    .name("winning")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|number| number.parse::<i32>().unwrap())
            );

            let received_numbers: HashSet<i32> = HashSet::from_iter(
                c
                    .name("received")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|number| number.parse::<i32>().unwrap())
            );

            winning_numbers.intersection(&received_numbers).count()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day4/input_example.txt"));

        assert_eq!(puzzle_1(input), 13);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day4/input.txt"));

        assert_eq!(puzzle_1(input), 21158);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day4/input_example.txt"));

        assert_eq!(puzzle_2(input), 30);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day4/input.txt"));

        assert_eq!(puzzle_2(input), 6050769);
    }
}