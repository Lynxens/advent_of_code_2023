use itertools::Itertools;
use nom::InputIter;

fn main() {
    let input = parse(include_str!("../../data/day10/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

struct LoopMap {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
    start: (usize, usize),
}

fn parse(raw_input: &str) -> LoopMap {
    let start_i = raw_input.position(|c| c == 'S').unwrap();

    let map: Vec<Vec<char>> = raw_input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect();

    let width = map[0].len();
    let height = map.len();

    LoopMap {
        map,
        height,
        width,
        start: (start_i / (width + 1), start_i % (width + 1)),
    }
}

fn puzzle_1(loop_map: &LoopMap) -> usize {
    let mut max_loop_length = 0;
    let mut sidepaths: Vec<((usize, usize), Vec<(usize, usize)>)> = von_neumann_neighborhood(loop_map.start, loop_map.height, loop_map.width)
        .into_iter()
        .filter(|&(y, x)| loop_map.map[y][x] != '.')
        .map(|coord| (coord, Vec::from([loop_map.start])))
        .collect();

    while let Some((mut coord, mut path)) = sidepaths.pop() {
        while coord != loop_map.start {
            let mut options: Vec<(usize, usize)> = von_neumann_neighborhood(coord, loop_map.height, loop_map.width)
                .into_iter()
                .filter(|&(y, x)| {
                    let next_sign = loop_map.map[y][x];

                    if next_sign == 'S' {
                        return true;
                    }

                    if path.contains(&(y, x)) {
                        return false;
                    }

                    match (loop_map.map[coord.0][coord.1], y as i32 - coord.0 as i32, x as i32 - coord.1 as i32) {
                        ('|' | 'L' | 'J', -1, _) => ['|', '7', 'F'].contains(&next_sign),
                        ('|' | '7' | 'F', 1, _) => ['|', 'L', 'J'].contains(&next_sign),
                        ('-' | 'J' | '7', _, -1) => ['-', 'L', 'F'].contains(&next_sign),
                        ('-' | 'L' | 'F', _, 1) => ['-', '7', 'J'].contains(&next_sign),
                        _ => false,
                    }
                }).collect();

            if options.is_empty() {
                break;
            }

            path.push(coord);
            coord = options.pop().unwrap();

            sidepaths.extend(
                options
                    .iter()
                    .map(|&option| (option, path.clone()))
            );
        }

        if coord == loop_map.start {
            max_loop_length = max_loop_length.max(path.len())
        }
    }

    max_loop_length / 2
}

fn puzzle_2(_data: &LoopMap) -> i32 {
    0
}

fn von_neumann_neighborhood((y, x): (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    [
        (y.wrapping_sub(1), x),
        (y, x.wrapping_sub(1)),
        (y, x + 1),
        (y + 1, x),
    ]
        .into_iter()
        .filter(|&(ny, nx)| ny < height && nx < width)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day10/input_example.txt"));

        assert_eq!(puzzle_1(&input), 8);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day10/input.txt"));

        assert_eq!(puzzle_1(&input), 6717);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day10/input_example.txt"));

        assert_eq!(puzzle_2(&input), 0);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day10/input.txt"));

        assert_eq!(puzzle_2(&input), 0);
    }
}