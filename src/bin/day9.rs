use itertools::Itertools;

fn main() {
    let input = parse(include_str!("../../data/day9/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

fn parse(raw_input: &str) -> Vec<Vec<i32>>
{
    raw_input
        .lines()
        .map(|l| l
            .split(' ')
            .map(|d| d.parse::<i32>().unwrap())
            .collect()
        )
        .collect()
}

fn puzzle_1(data: &[Vec<i32>]) -> i32 {
    data
        .iter()
        .map(|start_sequence| {
            let mut sequence = start_sequence.clone();
            let mut next_value = *start_sequence.last().unwrap();

            while !sequence.iter().all_equal() {
                sequence = elementwise_diff(&sequence);
                next_value += sequence.last().unwrap();
            }

            next_value
        })
        .sum()
}

fn puzzle_2(data: &[Vec<i32>]) -> i32 {
    data
        .iter()
        .map(|start_sequence| {
            let mut sequences = Vec::from([start_sequence.clone()]);

            while !sequences.last().unwrap().iter().all_equal() {
                sequences.push(elementwise_diff(sequences.last().unwrap()));
            }

            let initial_diff = *sequences.last().unwrap().first().unwrap();
            sequences
                .iter()
                .rev()
                .skip(1)
                .fold(initial_diff, |prev_diff, sequence| sequence.first().unwrap() - prev_diff)
        })
        .sum()
}

fn elementwise_diff(vec: &[i32]) -> Vec<i32> {
    vec
        .iter()
        .tuple_windows()
        .map(|(&d1, &d2)| d2 - d1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day9/input_example.txt"));

        assert_eq!(puzzle_1(&input), 114);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day9/input.txt"));

        assert_eq!(puzzle_1(&input), 1974232246);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day9/input_example.txt"));

        assert_eq!(puzzle_2(&input), 2);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day9/input.txt"));

        assert_eq!(puzzle_2(&input), 928);
    }
}