use itertools::{Itertools};

fn main() {
    let input = parse(include_str!("../../data/day7/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

fn parse(raw_input: &str) -> Vec<(&str, i64)>
{
    raw_input
        .lines()
        .map(|l| {
            let (cards, bet) = l.split_once(' ').unwrap();

            (cards, bet.parse::<i64>().unwrap())
        })
        .collect()
}

fn puzzle_1(data: &[(&str, i64)]) -> i64 {
    let hands: Vec<(i64, Vec<usize>, i64)> = data
        .iter()
        .map(|&(cards, bet)| (
            get_hand_type(cards),
            get_card_strengths(cards),
            bet,
        ))
        .collect();

    calculate_winnings(&hands)
}

fn puzzle_2(data: &[(&str, i64)]) -> i64 {
    let hands: Vec<(i64, Vec<usize>, i64)> = data
        .iter()
        .map(|&(cards, bet)| {
            (
                cards
                    .chars()
                    .unique()
                    .map(|replacement| get_hand_type(&cards.replace('J', &replacement.to_string())))
                    .max()
                    .unwrap(),
                get_card_strengths(&cards.replace('J', "*")),
                bet,
            )
        })
        .collect();

    calculate_winnings(&hands)
}

fn get_hand_type(cards: &str) -> i64 {
    let card_counts = cards
        .chars()
        .counts()
        .into_values()
        .sorted()
        .rev()
        .collect::<Vec<usize>>();
    
    match (card_counts[0], card_counts.get(1).unwrap_or(&0)) {
        (5, _) => 7,
        (4, _) => 6,
        (3, 2) => 5,
        (3, _) => 4,
        (2, 2) => 3,
        (2, _) => 2,
        _ => 1,
    }
}

fn get_card_strengths(cards: &str) -> Vec<usize> {
    cards
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => 1,
        })
        .collect()
}

fn calculate_winnings(hands: &[(i64, Vec<usize>, i64)]) -> i64 {
    hands
        .iter()
        .sorted_by(| (hand_type_a, cards_a, _), (hand_type_b, cards_b, _) | {
            hand_type_a.cmp(hand_type_b).then(cards_a.cmp(cards_b))
        })
        .enumerate()
        .map(|(rank, &(_, _, bet))| bet * ((rank as i64) + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day7/input_example.txt"));

        assert_eq!(puzzle_1(&input), 6440);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day7/input.txt"));

        assert_eq!(puzzle_1(&input), 248836197);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day7/input_example.txt"));

        assert_eq!(puzzle_2(&input), 5905);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day7/input.txt"));

        assert_eq!(puzzle_2(&input), 251195607);
    }
}