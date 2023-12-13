

fn main() {
    let input = parse(include_str!("../../data/day0/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

fn parse(raw_input: &str) -> Vec<&str>
{
    raw_input
        .lines()
        .collect()
}

fn puzzle_1(_data: &[&str]) -> i32 {
    0
}

fn puzzle_2(_data: &[&str]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day0/input_example.txt"));

        assert_eq!(puzzle_1(&input), 0);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day0/input.txt"));

        assert_eq!(puzzle_1(&input), 0);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day0/input_example.txt"));

        assert_eq!(puzzle_2(&input), 0);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day0/input.txt"));

        assert_eq!(puzzle_2(&input), 0);
    }
}