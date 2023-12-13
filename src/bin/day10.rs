use std::cmp::Ordering;
use itertools::Itertools;
use nom::InputIter;
use advent_of_code_2023::*;
use advent_of_code_2023::Direction::{East, North, South, West};

fn main() {
    let input = parse(include_str!("../../data/day10/input.txt"));

    puzzle_1(&input);
    puzzle_2(&input);
}

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

const OUTSIDE: char = '0';
const INSIDE: char = '1';

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

fn puzzle_2(loop_map: &LoopMap) -> usize {
    let longest_loop = find_longest_loop(loop_map);

    let mut map = vec![vec![INSIDE; loop_map.width]; loop_map.height];

    // Add chars that are part of loop
    longest_loop
        .iter()
        .for_each(|&c| map[c.0][c.1] = loop_map.map[c.0][c.1]);

    let mut queue: Vec<Coordinate> = get_boundary_coordinates(loop_map.height, loop_map.width)
        .into_iter()
        .filter(|&c| {
            if map[c.0][c.1] == INSIDE {
                map[c.0][c.1] = OUTSIDE;
                return true;
            }

            return false;
        })
        .collect();

    let mut has_followed_loop = false;
    while let Some(c) = queue.pop() {
        for (direction, nc) in von_neumann_compass(c, loop_map.height, loop_map.width).into_iter() {
            let nc_sign = map[nc.0][nc.1];

            match nc_sign {
                INSIDE => {
                    map[nc.0][nc.1] = OUTSIDE;
                    queue.push(nc);
                },
                '-' | '|' => {
                    if has_followed_loop {
                       continue
                    }

                    let loop_index = longest_loop.iter().position(|&lc| lc == nc).unwrap();
                    let mut rotated_loop = longest_loop.clone();
                    rotated_loop.rotate_left(loop_index);

                    let mut outside_direction = direction.opposite();
                    let mut last_direction = movement_direction(&rotated_loop[0], &rotated_loop[1]);
                    for (current_lc, next_lc) in rotated_loop.iter().skip(1).tuple_windows() {
                        match coordinate_in_direction(current_lc, &outside_direction, loop_map.height, loop_map.width) {
                            Some(outside_coordinate) => {
                                if map[outside_coordinate.0][outside_coordinate.1] == INSIDE {
                                    map[outside_coordinate.0][outside_coordinate.1] = OUTSIDE;
                                    queue.push(outside_coordinate);
                                }
                            },
                            None => {},
                        }

                        let current_direction = movement_direction(current_lc, next_lc);
                        outside_direction = match (last_direction, current_direction) {
                            (North, East) | (East, South) | (South, West) | (West, North) => outside_direction.rotate_clockwise(),
                            (North, West) | (West, South) | (South, East) | (East, North) => outside_direction.rotate_counterclockwise(),
                            _ => outside_direction,
                        };

                        match coordinate_in_direction(current_lc, &outside_direction, loop_map.height, loop_map.width) {
                            Some(outside_coordinate) => {
                                if map[outside_coordinate.0][outside_coordinate.1] == INSIDE {
                                    map[outside_coordinate.0][outside_coordinate.1] = OUTSIDE;
                                    queue.push(outside_coordinate);
                                }
                            },
                            None => {},
                        }

                        last_direction = current_direction;
                    }

                    has_followed_loop = true;
                },
                _ => continue,
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
    let mut branches: Vec<Branch> = von_neumann_compass(loop_map.start, loop_map.height, loop_map.width)
        .into_iter()
        .filter(|&(_, c)| loop_map.map[c.0][c.1] != '.')
        .map(|(_, c)| Branch { coord: c, path_length: 1, intersection_count: 0 })
        .collect();

    'branch_loop: while let Some(branch) = branches.pop() {
        let mut current_coord = branch.coord;
        path.truncate(branch.path_length);
        intersections.truncate(branch.intersection_count);

        while current_coord != loop_map.start {
            let current_sign = loop_map.map[current_coord.0][current_coord.1];

            let mut connecting_pipes: Vec<Coordinate> = von_neumann_compass(current_coord, loop_map.height, loop_map.width)
                .into_iter()
                .filter_map(|(direction, next_coord)| {
                    let next_sign = loop_map.map[next_coord.0][next_coord.1];

                    if next_sign == '.' || next_coord == path[path.len() -1] || intersections.contains(&next_coord) {
                        return None;
                    }

                    match (current_sign, next_sign, direction) {
                        (_, 'S', _) => Some(next_coord),
                        ('|' | 'L' | 'J', '|' | '7' | 'F', North) => Some(next_coord),
                        ('|' | '7' | 'F', '|' | 'L' | 'J',  South) => Some(next_coord),
                        ('-' | 'J' | '7', '-' | 'L' | 'F', West) => Some(next_coord),
                        ('-' | 'L' | 'F', '-' | '7' | 'J', East) => Some(next_coord),
                        _ => None,
                    }
                })
                .collect();

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

fn movement_direction(from: &Coordinate, to: &Coordinate) -> Direction {
    match (to.0.cmp(&from.0), to.1.cmp(&from.1)) {
        (Ordering::Less, Ordering::Equal) => Ok(North),
        (Ordering::Equal, Ordering::Greater) => Ok(East),
        (Ordering::Greater, Ordering::Equal) => Ok(South),
        (Ordering::Equal, Ordering::Less) => Ok(West),
        _ => Err("Unknown direction"),
    }.expect("Unknown direction")
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

        assert_eq!(puzzle_2(&input), 381);
    }
}