use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let (grid, grid_width) = parse(include_str!("../../data/day3/input.txt"));

    puzzle_1(&grid, grid_width);
    puzzle_2(&grid, grid_width);
}

fn parse(raw_input: &str) -> (String, usize)
{
    // Create a border of dots around the grid
    let lines = raw_input
        .split('\n')
        .map(|line| format!(".{line}."))
        .collect::<Vec<String>>();

    let grid_width = lines.first().unwrap().len();
    let hor_border = str::repeat(".", grid_width);

    (format!("{hor_border}{}{hor_border}", lines.join("")), grid_width)
}

fn puzzle_1(grid: &str, grid_width: usize) -> i32 {
    let symbol_indices: HashSet<usize> = HashSet::from_iter(
        Regex::new(r"[^\d.]")
            .unwrap()
            .find_iter(grid)
            .map(|m| m.start())
    );

    Regex::new(r"\d+")
        .unwrap()
        .find_iter(grid)
        .filter(|m| {
            let start = m.start();
            let end = m.end();

            for i in (start - grid_width - 1) .. (end - grid_width + 1) {
                if symbol_indices.contains(&i) {
                    return true;
                }
            }

            for i in (start + grid_width - 1) .. (end + grid_width + 1) {
                if symbol_indices.contains(&i) {
                    return true;
                }
            }

            symbol_indices.contains(&(start - 1)) || symbol_indices.contains(&end)
        })
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .sum()
}

fn puzzle_2(grid: &str, grid_width: usize) -> i32 {
    let number_indices: HashMap<usize, i32> = HashMap::from_iter(
        Regex::new(r"\d+")
            .unwrap()
            .find_iter(grid)
            .flat_map(|m| {
                m
                    .range()
                    .map(|i| (i, m.as_str().parse::<i32>().unwrap()))
                    .collect::<Vec<(usize, i32)>>()
            })
    );

    grid
        .chars()
        .enumerate()
        .filter_map(|(i, char)| {
            if char != '*' {
                return None;
            }

            let part_numbers: HashSet<&i32> = HashSet::from_iter(
                [
                    i - grid_width - 1,
                    i - grid_width,
                    i - grid_width + 1,
                    i - 1,
                    i + 1,
                    i + grid_width - 1,
                    i + grid_width,
                    i + grid_width + 1,
                ]
                    .into_iter()
                    .filter_map(|j| number_indices.get(&j))
            );

            if part_numbers.len() != 2 {
                return None;
            }

            Some(
                part_numbers
                    .into_iter()
                    .product::<i32>()
            )
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let (grid, grid_width) = parse(include_str!("../../data/day3/input_example.txt"));

        assert_eq!(puzzle_1(&grid, grid_width), 4361);
    }

    #[test]
    fn test_puzzle_1() {
        let (grid, grid_width) = parse(include_str!("../../data/day3/input.txt"));

        assert_eq!(puzzle_1(&grid, grid_width), 528819);
    }

    #[test]
    fn test_puzzle_2_example() {
        let (grid, grid_width) = parse(include_str!("../../data/day3/input_example.txt"));

        assert_eq!(puzzle_2(&grid, grid_width), 467835);
    }

    #[test]
    fn test_puzzle_2() {
        let (grid, grid_width) = parse(include_str!("../../data/day3/input.txt"));

        assert_eq!(puzzle_2(&grid, grid_width), 80403602);
    }
}