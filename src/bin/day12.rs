use std::collections::{HashMap};

fn main() {
    let input1 = parse(include_str!("../../data/day12/input.txt"), 1);
    let input2 = parse(include_str!("../../data/day12/input.txt"), 5);

    puzzle_1(&input1);
    puzzle_2(&input2);
}

struct ConditionRecord {
    springs: Vec<char>,
    groups: Vec<usize>,
}

type Memo = HashMap<(usize, usize), u64>;

fn parse(raw_input: &str, repeat: usize) -> Vec<ConditionRecord>
{
    raw_input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();

            ConditionRecord {
                springs: vec![left; repeat].join(&"?").chars().collect(),
                groups: vec![right; repeat].join(&",").split(',').map(|count| count.parse::<usize>().unwrap()).collect(),
            }
        })
        .collect()
}

fn puzzle_1(data: &[ConditionRecord]) -> u64 {
    data
        .iter()
        .map(|record| count_possible_arrangements(record))
        .sum()
}

fn puzzle_2(data: &[ConditionRecord]) -> u64 {
    data
        .iter()
        .map(|record| count_possible_arrangements(record))
        .sum()
}


fn count_possible_arrangements(record: &ConditionRecord) -> u64 {
    let mut memo: Memo = HashMap::new();
    traverse_arrangement_tree(record, 0, 0, &mut memo)
}

fn traverse_arrangement_tree(record: &ConditionRecord, spring_index: usize, group_index: usize, memo: &mut Memo) -> u64 {
    if group_index == record.groups.len() && spring_index <= record.springs.len() {
        return 1;
    }

    if spring_index >= record.springs.len() {
        return 0;
    }

    match memo.get(&(spring_index, group_index)) {
        Some(&total) => return total,
        None => {}
    }

    let count_with_operational_next = match record.springs[spring_index] {
        '#' => 0,
        _ => traverse_arrangement_tree(record, spring_index + 1, group_index, memo),
    };

    let count_with_damaged_next = match record.springs[spring_index] {
        '.' => 0,
        _ => {
            match next_spring_index_with_group(record, spring_index, group_index) {
                Some(next_spring_index) => traverse_arrangement_tree(record, next_spring_index, group_index + 1, memo),
                None => 0,
            }
        },
    };

    let sum = count_with_operational_next + count_with_damaged_next;

    memo.insert((spring_index, group_index), sum);

    return sum;
}

fn next_spring_index_with_group(record: &ConditionRecord, spring_index: usize, group_index: usize) -> Option<usize> {
    let next_group_size = record.groups[group_index];

    if spring_index + next_group_size > record.springs.len() {
        return None;
    }

    if !&record.springs[spring_index..spring_index + next_group_size].iter().all(|&c| c != '.') {
        return None;
    }

    if group_index == (record.groups.len() - 1) {
        return if record.springs[spring_index + next_group_size..].iter().all(|&c| c != '#') {
            Some(spring_index + next_group_size)
        } else {
            None
        }
    } else {
        match record.springs.get(spring_index + (next_group_size - 1) + 1) {
            Some('#') => None,
            Some(_) => Some(spring_index + next_group_size + 1),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day12/input_example.txt"), 1);

        assert_eq!(puzzle_1(&input), 21);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day12/input.txt"), 1);

        assert_eq!(puzzle_1(&input), 6981);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day12/input_example.txt"), 5);

        assert_eq!(puzzle_2(&input), 525152);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day12/input.txt"), 5);

        assert_eq!(puzzle_2(&input), 4546215031609);
    }
}