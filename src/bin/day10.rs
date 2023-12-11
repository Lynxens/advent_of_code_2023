use std::cmp::Ordering;
use itertools::Itertools;
use nom::InputIter;
use advent_of_code_2023::*;
use crate::Direction::{East, North, South, West};

fn main() {
    let input = parse(include_str!("../../data/day10/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

type Coordinate = (usize, usize);

struct LoopMap {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
    start: Coordinate,
}

struct Branch {
    coord: Coordinate,
    path_length: usize,
    intersection_count: usize,
}

enum Direction { North, East, South, West }
type Compass = Vec<(Direction, Coordinate)>;

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
    find_longest_loop(loop_map).len() / 2
}

const OUTSIDE: char = '0';
const INSIDE: char = '1';
const V_PATH: char = 'V';
const H_PATH: char = 'H';
const VISITED_V_PATH: char = 'Y';
const VISITED_H_PATH: char = 'X';

fn puzzle_2(loop_map: &LoopMap) -> usize {
    let longest_loop = find_longest_loop(loop_map);

    let mut map = vec![vec![INSIDE; loop_map.width]; loop_map.height];

    // Add chars that are part of loop
    longest_loop
        .iter()
        .for_each(|&c| map[c.0][c.1] = loop_map.map[c.0][c.1]);

    // Convert start of loop to the correct pipe
    map[loop_map.start.0][loop_map.start.1] = match (
        compare_coordinates(&loop_map.start, &longest_loop[2]),
        compare_coordinates(&loop_map.start, &longest_loop[longest_loop.len() - 1])
    ) {
        ((_, Ordering::Equal), (_, Ordering::Equal)) => Ok('|'),
        ((Ordering::Equal, _), (Ordering::Equal, _)) => Ok('-'),
        ((Ordering::Less, _), (_, Ordering::Greater)) => Ok('L'),
        ((_, Ordering::Greater), (Ordering::Less, _)) => Ok('L'),
        ((Ordering::Less, _), (_, Ordering::Less)) => Ok('J'),
        ((_, Ordering::Less), (Ordering::Less, _)) => Ok('J'),
        ((_, Ordering::Less), (Ordering::Greater, _)) => Ok('7'),
        ((Ordering::Greater, _), (_, Ordering::Less)) => Ok('7'),
        ((Ordering::Greater, _), (_, Ordering::Greater)) => Ok('F'),
        ((_, Ordering::Greater), (Ordering::Greater, _)) => Ok('F'),
        _ => Err("Could not connect pipes")
    }.expect("Could not connect pipes");

    // Add vertical and horizontal pathways
    // longest_loop
    //     .iter()
    //     .for_each(|&c| {
    //         let c_sign = map[c.0][c.1];
    //
    //         for &nc in von_neumann_neighborhood(c, loop_map.height, loop_map.width).iter() {
    //             let nc_sign = map[nc.0][nc.1];
    //
    //             let sign = match (c_sign, nc_sign, compare_coordinates(&c, &nc)) {
    //                 ('|' | 'L' | 'F' | V_PATH, '|' | '7' | 'J' | V_PATH, (_, Ordering::Less)) => V_PATH,
    //                 ('|' | '7' | 'J' | V_PATH, '|' | 'L' | 'F' | V_PATH, (_, Ordering::Greater)) => V_PATH,
    //                 ('-' | '7' | 'F' | H_PATH, '-' | 'L' | 'J' | H_PATH,  (Ordering::Less, _)) => H_PATH,
    //                 ('-' | 'L' | 'J' | H_PATH, '-' | '7' | 'F' | H_PATH,  (Ordering::Greater, _)) => H_PATH,
    //                 _ => continue,
    //             };
    //
    //             map[c.0][c.1] = sign;
    //             break;
    //         }
    //     });

    let mut queue: Vec<Coordinate> = get_boundary_coordinates(loop_map.height, loop_map.width)
        .into_iter()
        .filter(|&c| {
            let sign = map[c.0][c.1];

            map[c.0][c.1] = match sign {
                INSIDE => OUTSIDE,
                // V_PATH => VISITED_V_PATH,
                // H_PATH => VISITED_H_PATH,
                _ => return false,
            };

            return true;
        })
        .collect();

    let mut has_followed_loop = false;
    while let Some(c) = queue.pop() {
        let c_sign = map[c.0][c.1];

        for (direction, nc) in von_neumann_compass(c, loop_map.height, loop_map.width).into_iter() {
            let nc_sign = map[nc.0][nc.1];

            match nc_sign {
                INSIDE => {
                    map[nc.0][nc.1] = OUTSIDE;
                    queue.push(nc);
                },
                OUTSIDE => continue,
                _ => {
                    if has_followed_loop {
                       continue
                    }

                    let loop_index = longest_loop.iter().position(|&lc| lc == nc).unwrap();
                    let mut rotated_loop = longest_loop.clone();
                    rotated_loop.rotate_left(loop_index);


                    for (&current_lc, &next_lc) in rotated_loop.iter().tuple_windows() {
                        match compare_coordinates(&current_lc, &next_lc) {
                            (O)
                        }
                    }


                    has_followed_loop = false;
                }
            }
        }
    }

    map
        .into_iter()
        .map(|row| row
            .into_iter()
            .filter(|&c| c == INSIDE)
            .count()
        )
        .sum()
}

fn find_longest_loop(loop_map: &LoopMap) -> Vec<Coordinate> {
    let mut longest_path: Vec<Coordinate> = Vec::new();
    let mut path: Vec<Coordinate> = Vec::from([loop_map.start]);
    let mut intersections: Vec<Coordinate> = Vec::new();
    let mut branches: Vec<Branch> = von_neumann_neighborhood(loop_map.start, loop_map.height, loop_map.width)
        .into_iter()
        .filter(|&(y, x)| loop_map.map[y][x] != '.')
        .map(|coord| Branch { coord, path_length: 1, intersection_count: 0 })
        .collect();

    'branch_loop: while let Some(branch) = branches.pop() {
        let mut current_coord = branch.coord;
        path.truncate(branch.path_length);
        intersections.truncate(branch.intersection_count);

        while current_coord != loop_map.start {
            let current_sign = loop_map.map[current_coord.0][current_coord.1];

            let mut connecting_pipes: Vec<Coordinate> = von_neumann_neighborhood(current_coord, loop_map.height, loop_map.width)
                .into_iter()
                .filter(|&next_coord| {
                    let next_sign = loop_map.map[next_coord.0][next_coord.1];

                    if next_sign == '.' || next_coord == path[path.len() -1] || intersections.contains(&next_coord) {
                        return false;
                    }

                    match (current_sign, next_sign, compare_coordinates(&current_coord, &next_coord)) {
                        (_, 'S', _) => true,
                        ('|' | 'L' | 'J', '|' | '7' | 'F', (Ordering::Less, _)) => true,
                        ('|' | '7' | 'F', '|' | 'L' | 'J',  (Ordering::Greater, _)) => true,
                        ('-' | 'J' | '7', '-' | 'L' | 'F', (_, Ordering::Less)) => true,
                        ('-' | 'L' | 'F', '-' | '7' | 'J', (_, Ordering::Greater)) => true,
                        _ => false,
                    }
                }).collect();

            path.push(current_coord);

            if connecting_pipes.len() > 1 {
                intersections.push(current_coord);
                connecting_pipes
                    .drain(1..)
                    .for_each(|c| branches.push(Branch {
                        coord: c,
                        path_length: path.len(),
                        intersection_count: intersections.len(),
                    }));
            }

            current_coord = match connecting_pipes.pop() {
                Some(c) => c,
                None => continue 'branch_loop,
            };
        }

        if path.len() > longest_path.len() {
            longest_path = path.clone();
        }
    }

    longest_path
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


fn von_neumann_compass((y, x): (usize, usize), height: usize, width: usize) -> Compass {
    [
        (North, (y.wrapping_sub(1), x)),
        (East, (y, x + 1)),
        (South, (y + 1, x)),
        (West, (y, x.wrapping_sub(1))),
    ]
        .into_iter()
        .filter(|&(_, (ny, nx))| ny < height && nx < width)
        .collect()
}

fn compare_coordinates(from: &Coordinate, to: &Coordinate) -> (Ordering, Ordering) {
    (
        to.0.cmp(&from.0),
        to.1.cmp(&from.1),
    )
}

fn get_boundary_coordinates(height: usize, width: usize) -> Vec<Coordinate> {
    let top: Vec<Coordinate> = vec![0; width]
        .into_iter()
        .zip(0..width)
        .collect();

    let bottom: Vec<Coordinate> = vec![height - 1; width]
        .into_iter()
        .zip(0..width)
        .collect();

    let left: Vec<Coordinate> = (0..height)
        .into_iter()
        .zip(vec![0; height])
        .collect();

    let right: Vec<Coordinate> = (0..height)
        .into_iter()
        .zip(vec![width - 1; height])
        .collect();

    Vec::from_iter([top, right, bottom, left].into_iter().flatten())
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
        let input1 = parse(include_str!("../../data/day10/input_example_2.txt"));
        let input2 = parse(include_str!("../../data/day10/input_example_3.txt"));
        let input3 = parse(include_str!("../../data/day10/input_example_4.txt"));

        assert_eq!(puzzle_2(&input1), 4);
        assert_eq!(puzzle_2(&input2), 8);
        assert_eq!(puzzle_2(&input3), 10);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day10/input.txt"));

        assert_eq!(puzzle_2(&input), 0);
    }
}