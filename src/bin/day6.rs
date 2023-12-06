use float_next_after::NextAfter;

fn main() {
    puzzle_1(&Vec::from([
        (49, 298),
        (78, 1185),
        (79, 1066),
        (80, 1181),
    ]));
    puzzle_2(49787980, 298118510661181);
}

fn puzzle_1(data: &[(i64, i64)]) -> i64 {
    data
        .iter()
        .map(|&(time_limit, target_distance)| count_winning_options(time_limit, target_distance))
        .product()
}

fn puzzle_2(time_limit: i64, target_distance: i64) -> i64 {
    count_winning_options(time_limit, target_distance)
}

fn count_winning_options(time_limit: i64, target_distance: i64) -> i64 {
    // Limits where the resulting distance will be larger than the target distance
    let min_hold_time_limit: f64 = 0.5 * ((time_limit as f64) - ((time_limit.pow(2) - 4 * target_distance) as f64).sqrt());
    let max_hold_time_limit: f64 = 0.5 * ((time_limit as f64) + ((time_limit.pow(2) - 4 * target_distance) as f64).sqrt());

    // Find the outer integers that are within these float limits
    let min_hold_time = min_hold_time_limit.next_after(f64::INFINITY).ceil() as i64;
    let max_hold_time = max_hold_time_limit.next_after(f64::NEG_INFINITY).floor() as i64;

    max_hold_time - min_hold_time + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = Vec::from([
            (7, 9),
            (15, 40),
            (30, 200),
        ]);

        assert_eq!(puzzle_1(&input), 288);
    }

    #[test]
    fn test_puzzle_1() {
        let input = Vec::from([
            (49, 298),
            (78, 1185),
            (79, 1066),
            (80, 1181),
        ]);

        assert_eq!(puzzle_1(&input), 2269432);
    }

    #[test]
    fn test_puzzle_2_example() {

        assert_eq!(puzzle_2(71530, 940200), 71503);
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(49787980, 298118510661181), 35865985);
    }
}