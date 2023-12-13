use itertools::Itertools;

fn main() {
    let input = parse(include_str!("../../data/day11/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input, 1000000));
}

fn parse(raw_input: &str) -> Vec<&str>
{
    raw_input
        .lines()
        .collect()
}

fn puzzle_1(data: &[&str]) -> usize {
    calculate_summed_galaxy_distances(data, 2)
}

fn puzzle_2(data: &[&str], expansion_factor: usize) -> usize {
    calculate_summed_galaxy_distances(data, expansion_factor)
}


fn calculate_summed_galaxy_distances(space: &[&str], expansion_factor: usize) -> usize {
    let galaxies: Vec<(usize, usize)> = space
        .iter()
        .enumerate()
        .flat_map(|(i, &row)| {
            row
                .chars()
                .positions(|c| c == '#')
                .map(|j| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let row_expansion_factor: Vec<usize> = space
        .iter()
        .map(|&row| if row.chars().all(|c| c == '.') { expansion_factor } else { 1 })
        .collect();

    let col_expansion_factor: Vec<usize> = (0..space[0].len())
        .map(|column| if space.iter().all(|&row| row.chars().nth(column).unwrap() == '.') { expansion_factor } else { 1 })
        .collect();

    let mut row_freq_table = vec![0; space.len()];
    let mut col_freq_table = vec![0; space[0].len()];

    for (&(ya, xa), &(yb, xb)) in galaxies.iter().tuple_combinations() {
        row_freq_table[ya.min(yb)..ya.max(yb)].iter_mut().for_each(|y| *y += 1);
        col_freq_table[xa.min(xb)..xa.max(xb)].iter_mut().for_each(|x| *x += 1);
    }

    let vertical_steps: usize = row_freq_table
        .iter()
        .zip(row_expansion_factor)
        .map(|(&steps, factor)| steps * factor)
        .sum();

    let horizontal_steps: usize = col_freq_table
        .iter()
        .zip(col_expansion_factor)
        .map(|(&steps, factor)| steps * factor)
        .sum();

    vertical_steps + horizontal_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day11/input_example.txt"));

        assert_eq!(puzzle_1(&input), 374);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day11/input.txt"));

        assert_eq!(puzzle_1(&input), 9556896);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day11/input_example.txt"));

        assert_eq!(puzzle_2(&input, 10), 1030);
        assert_eq!(puzzle_2(&input, 100), 8410);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day11/input.txt"));

        assert_eq!(puzzle_2(&input, 1000000), 685038186836);
    }
}