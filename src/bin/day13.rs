use std::cmp::min;
use advent_of_code_2023::{count_different_elements, transpose};

type Pattern = Vec<Vec<char>>;

fn main() {
    let input = parse(include_str!("../../data/day13/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

fn parse(raw_input: &str) -> Vec<Pattern>
{
    raw_input
        .split("\n\n")
        .map(|block| block.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn puzzle_1(data: &[Pattern]) -> usize {
    data
        .iter()
        .map(|p| {
            match find_mirror_position(p) {
                Some(position) => position * 100,
                None => find_mirror_position(&transpose(p)).unwrap_or(0),
            }
        })
        .sum()
}

fn puzzle_2(data: &[Pattern]) -> usize {
    data
        .iter()
        .map(|p| {
            match find_mirror_position_with_smudge(p) {
                Some(position) => position * 100,
                None => find_mirror_position_with_smudge(&transpose(p)).unwrap_or(0),
            }
        })
        .sum()
}

fn find_mirror_position(pattern: &Pattern) -> Option<usize> {
    (1..pattern.len())
        .find(|&position| (1..=min(position, pattern.len() - position))
            .all(|offset| pattern[position - offset] == pattern[position + offset - 1])
        )
}

fn find_mirror_position_with_smudge(pattern: &Pattern) -> Option<usize> {
    (1..pattern.len())
        .find(|&position| (1..=min(position, pattern.len() - position))
            .try_fold(false, | found_smudge, offset | {
                let diff = count_different_elements(&pattern[position - offset], &pattern[position + offset - 1]);
                match (diff, found_smudge) {
                    (0, _) => Some(found_smudge),
                    (1, false) => Some(true),
                    _ => None,
                }
            })
            .unwrap_or(false)
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day13/input_example.txt"));

        assert_eq!(puzzle_1(&input), 405);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day13/input.txt"));

        assert_eq!(puzzle_1(&input), 37381);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day13/input_example.txt"));

        assert_eq!(puzzle_2(&input), 400);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day13/input.txt"));

        assert_eq!(puzzle_2(&input), 28210);
    }
}