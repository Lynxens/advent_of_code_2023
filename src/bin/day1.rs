use regex::Regex;

fn main() {
    let input = parse(include_str!("../../data/day1/input.txt"));

    puzzle_1(input);
    puzzle_2(input);
}

fn parse(raw_input: &str) -> &str
{
    raw_input
}

fn puzzle_1(data: &str) -> i32 {
    calculate_calibration_value_sum(data)
}

fn puzzle_2(data: &str) -> i32 {
    let numbers = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let data_with_replacements = numbers
        .into_iter()
        .enumerate()
        .fold(String::from(data), | new_data, (i, number) | {
            new_data.replace(number, format!("{number}{}{number}", i+1).as_str())
        });

    calculate_calibration_value_sum(data_with_replacements.as_str())
}

fn calculate_calibration_value_sum(data: &str) -> i32 {
    let re = Regex::new(r"\D*(?:(?P<first>\d).*(?P<last>\d)\D*|(?P<single>\d)\D*)").unwrap();

    re
        .captures_iter(data)
        .map(|c| {
            let single = c.name("single");
            let first = single.unwrap_or_else(|| c.name("first").unwrap()).as_str();
            let last = single.unwrap_or_else(|| c.name("last").unwrap()).as_str();

            format!("{first}{last}").parse::<i32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day1/input_example.txt"));

        assert_eq!(puzzle_1(&input), 142i32);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day1/input.txt"));

        assert_eq!(puzzle_1(&input), 54953i32);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day1/input_example_2.txt"));

        assert_eq!(puzzle_2(&input), 281i32);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day1/input.txt"));

        assert_eq!(puzzle_2(&input), 53868i32);
    }
}