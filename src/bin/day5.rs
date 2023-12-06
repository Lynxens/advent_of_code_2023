use itertools::{Itertools};

fn main() {
    let input = parse(include_str!("../../data/day5/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

fn parse(raw_input: &str) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>)
{
    let (first, rest) = raw_input.split_once("\n\n").unwrap();

    let seeds = first
        .split(' ')
        .into_iter()
        .skip(1)
        .map(|d| d.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let maps = rest
        .split("\n\n")
        .into_iter()
        .map(|section| {
            section
                .split('\n')
                .into_iter()
                .skip(1)
                .map(|line| line
                    .splitn(3,' ')
                    .map(|d| d.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
                )
                .collect::<Vec<(i64, i64, i64)>>()
        })
        .collect::<Vec<Vec<(i64, i64, i64)>>>();

    (seeds, maps)
}

fn puzzle_1((seeds, maps): &(Vec<i64>, Vec<Vec<(i64, i64, i64)>>)) -> i64 {
    seeds
        .iter()
        .map(|seed| {
            maps
                .iter()
                .fold(seed.clone(), | value, ranges | {
                    for &(destination_range_start, source_range_start, range_length) in ranges {
                        let offset = value - source_range_start;

                        if offset >= 0 && offset <= range_length {
                            return destination_range_start + offset
                        }
                    }

                    value
                })
        })
        .min()
        .unwrap()
}

fn puzzle_2((seeds, maps): &(Vec<i64>, Vec<Vec<(i64, i64, i64)>>)) -> i64 {
    let value_ranges = seeds
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut c| {
            let &start = c.next().unwrap();
            let &length = c.next().unwrap();
            (start, start + length - 1)
        })
        .collect::<Vec<(i64, i64)>>();

    maps
        .iter()
        .fold(value_ranges, | value_ranges, ranges| {
            let source_range_starts = ranges
                .iter()
                .map(|&(_, source_range_start, _)| source_range_start)
                .collect::<Vec<i64>>();

            let mut new_value_ranges: Vec<(i64, i64)> = Vec::new();

            value_ranges
                .iter()
                .for_each(|&(start, end)| {
                    let mut value = start;

                    while value <= end {
                        let in_any_range = ranges
                            .iter()
                            .any(|&(destination_range_start, source_range_start, range_length)| {
                                let offset = value - source_range_start;

                                if offset >= 0 && offset <= range_length {
                                    let new_range_start = destination_range_start + offset;
                                    let new_range_length = (range_length - offset).min(end - value + 1);
                                    new_value_ranges.push((
                                        new_range_start,
                                        new_range_start + 0.max(new_range_length - 1)
                                    ));
                                    value += 1.max(new_range_length);

                                    true
                                } else {
                                    false
                                }
                            });

                        if !in_any_range {
                            let next_source_range_start = source_range_starts.clone()
                                .into_iter()
                                .filter(|&s| s > value)
                                .min()
                                .unwrap_or(end + 1);

                            let new_range_end = end.min(next_source_range_start - 1);

                            new_value_ranges.push((value, new_range_end));
                            value = new_range_end + 1;
                        }
                    }
                });

            new_value_ranges
        })
        .into_iter()
        .map(|(start, _)| start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day5/input_example.txt"));

        assert_eq!(puzzle_1(&input), 35);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day5/input.txt"));

        assert_eq!(puzzle_1(&input), 178159714);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day5/input_example.txt"));

        assert_eq!(puzzle_2(&input), 46);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day5/input.txt"));

        assert_eq!(puzzle_2(&input), 100165128);
    }
}