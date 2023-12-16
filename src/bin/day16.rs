use std::collections::HashMap;
use std::cmp::{min, max};
use advent_of_code_2023::{Coordinate, Direction, get_boundary_coordinates};
use advent_of_code_2023::Direction::{North, East, South, West};

fn main() {
    let input = parse(include_str!("../../data/day16/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

// fn parse(raw_input: &str) -> Vec<Vec<char>> {
//     raw_input
//         .lines()
//         .map(|l| l.chars().collect())
//         .collect()
// }

fn parse(raw_input: &str) -> (Vec<Vec<char>>, HashMap<Coordinate, Reflector>) {
    let map: Vec<Vec<char>> = raw_input
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let height = map.len();
    let width = map[0].len();

    let mut reflectors: HashMap<Coordinate, Reflector> = HashMap::new();

    for row in 0..height {
        for col in 0..width {
            if map[row][col] != '.' {
                reflectors.insert((row, col), Reflector::from_map(&(row, col), &map));
            }
        }
    }

    (map, reflectors)
}

enum ReflectorType {
    Horizontal,
    Vertical,
    Angle45,
    Angle315,
}

impl ReflectorType {
    fn from_char(char: char) -> Option<Self> {
        match char {
            '-' => Some(ReflectorType::Horizontal),
            '|' => Some(ReflectorType::Vertical),
            '/' => Some(ReflectorType::Angle45),
            '\\' => Some(ReflectorType::Angle315),
            _ => None,
        }
    }

    fn bounce_light(&self, light_movement_direction: &Direction) -> Result<Direction, &'static str> {
        match self {
            ReflectorType::Angle45 => match light_movement_direction {
                North => Ok(East),
                East => Ok(North),
                South => Ok(West),
                West => Ok(South),
            }
            ReflectorType::Angle315 => match light_movement_direction {
                North => Ok(West),
                East => Ok(South),
                South => Ok(East),
                West => Ok(North),
            }
            _ => Err("Unexpected reflector type for bouncing"),
        }
    }

    fn split_light(&self, light_movement_direction: &Direction) -> Result<Option<(Direction, Direction)>, &'static str> {
        match self {
            ReflectorType::Horizontal => Ok(match light_movement_direction {
                North | South => Some((East, West)),
                East | West => None,
            }),
            ReflectorType::Vertical => Ok(match light_movement_direction {
                East | West => Some((North, South)),
                North | South => None,
            }),
            _ => Err("Unexpected reflector type for splitting"),
        }
    }
}

struct Reflector {
    reflector_type: ReflectorType,
    connections: HashMap<Direction, Coordinate>,
}

impl Reflector {
    fn from_map(coord: &Coordinate, map: &[Vec<char>]) -> Self {
        let height = map.len();
        let width = map[0].len();

        Self {
            reflector_type: ReflectorType::from_char(map[coord.0][coord.1]).unwrap(),
            connections: HashMap::from([
                (North, find_next_reflector(map, &coord, &North).unwrap_or((0, coord.1))),
                (East, find_next_reflector(map, &coord, &East).unwrap_or((coord.0, width - 1))),
                (South, find_next_reflector(map, &coord, &South).unwrap_or((height - 1, coord.1))),
                (West, find_next_reflector(map, &coord, &West).unwrap_or((coord.0, 0))),
            ])
        }
    }

    fn next(&self, light_movement_direction: &Direction) -> Vec<(Direction, Coordinate)> {
        match self.reflector_type {
            ReflectorType::Horizontal | ReflectorType::Vertical => match self.reflector_type.split_light(light_movement_direction).expect("Could not split light") {
                Some((d1, d2)) => vec![(d1, self.connections[&d1]), (d2, self.connections[&d2])],

                // Continues in same direction
                None => vec![(light_movement_direction.clone(), self.connections[light_movement_direction])],
            },
            ReflectorType::Angle45 | ReflectorType::Angle315 => {
                let next_direction = self.reflector_type.bounce_light(light_movement_direction).expect("Could not bounce light");
                vec![(next_direction, self.connections[&next_direction])]
            },
        }
    }
}

fn find_next_reflector(map: &[Vec<char>], coord: &Coordinate, direction: &Direction) -> Option<Coordinate> {
    let height = map.len();
    let width = map[0].len();

    match direction {
        North => if let Some(row) = (0..coord.0).rposition(|row| map[row][coord.1] != '.') { Some((row, coord.1)) } else { None },
        East => if let Some(col) = ((coord.1 + 1)..width).find(|&col| map[coord.0][col] != '.') { Some((coord.0, col)) } else { None }
        South => if let Some(row) = ((coord.0 + 1)..height).find(|&row| map[row][coord.1] != '.') { Some((row, coord.1)) } else { None }
        West => if let Some(col) = map[coord.0][..coord.1].iter().rposition(|&c| c != '.') { Some((coord.0, col)) } else { None }
    }
}

struct BeamGrid {
    grid: Vec<Vec<HashMap<Direction, bool>>>,
}

impl BeamGrid {
    fn new(height: usize, width: usize) -> Self {
        Self {
            grid: vec![
                vec![HashMap::from([(North, false), (East, false), (South, false), (West, false)]); width];
                height
            ],
        }
    }

    // Adds beam to grid and returns the number of new beam coordinates that have been added in the given direction
    fn add_beam(&mut self, direction: &Direction, from: &Coordinate, to: &Coordinate, goes_outside: bool) -> usize {
        match direction {
            North|South => (min(from.0, to.0)..=max(from.0, to.0))
                .filter(|&row| {
                    if !goes_outside && row == to.0 {
                        return false;
                    }

                    self.grid[row][from.1].insert(*direction, true).unwrap() == false
                })
                .count(),
            East|West => (min(from.1, to.1)..=max(from.1, to.1))
                .filter(|&col| {
                    if !goes_outside && col == to.1 {
                        return false;
                    }

                    self.grid[from.0][col].insert(*direction, true).unwrap() == false
                })
                .count(),
        }
    }
}

fn puzzle_1((map, reflectors): &(Vec<Vec<char>>, HashMap<Coordinate, Reflector>)) -> usize {
    count_energized_tiles(map, reflectors, &(0, 0), &East)
}

fn puzzle_2((map, reflectors): &(Vec<Vec<char>>, HashMap<Coordinate, Reflector>)) -> usize {
    let height = map.len();
    let width = map[0].len();

    let max_top = (0..width)
        .map(|col| count_energized_tiles(map, reflectors, &(0, col), &South))
        .max()
        .unwrap();

    let max_left = (0..height)
        .map(|row| count_energized_tiles(map, reflectors, &(row, 0), &East))
        .max()
        .unwrap();

    let max_right = (0..height)
        .map(|row| count_energized_tiles(map, reflectors, &(row, width - 1), &West))
        .max()
        .unwrap();

    let max_bottom = (0..width)
        .map(|col| count_energized_tiles(map, reflectors, &(height - 1, col), &North))
        .max()
        .unwrap();

    [max_top, max_left, max_right, max_bottom]
        .into_iter()
        .max()
        .unwrap()
}

fn count_energized_tiles(map: &[Vec<char>], reflectors: &HashMap<Coordinate, Reflector>, start_coord: &Coordinate, start_direction: &Direction) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut beam_grid: BeamGrid = BeamGrid::new(height, width);
    let mut beams: Vec<(Direction, Coordinate)> = Vec::new();

    if let Some(first_reflector_coord) = find_first_reflector(map, &start_coord, &start_direction) {
        beam_grid.add_beam(&start_direction, &start_coord, &first_reflector_coord, false);
        beams.push((start_direction.clone(), first_reflector_coord));
    } else {
        return match start_direction {
            North => beam_grid.add_beam(start_direction, start_coord, &(0, start_coord.1), true),
            East => beam_grid.add_beam(start_direction, start_coord, &(start_coord.0, width - 1), true),
            South => beam_grid.add_beam(start_direction, start_coord, &(height - 1, start_coord.1), true),
            West => beam_grid.add_beam(start_direction, start_coord, &(start_coord.0, 0), true),
        };
    }

    while let Some((direction, coord)) = beams.pop() {
        if let Some(reflector) = reflectors.get(&coord) {
            for (next_direction, next_coord) in reflector.next(&direction) {
                let goes_outside = coord == next_coord || !reflectors.contains_key(&next_coord);
                let new_beam_size = beam_grid.add_beam(&next_direction, &coord, &next_coord, goes_outside);
                if !goes_outside && new_beam_size > 0 {
                    beams.push((next_direction, next_coord))
                }
            }
        }
    }

    beam_grid.grid
        .into_iter()
        .flatten()
        .filter(|beams| beams.values().any(|&b| b == true))
        .count()
}

fn find_first_reflector(map: &[Vec<char>], coord: &Coordinate, direction: &Direction) -> Option<Coordinate> {
    let height = map.len();
    let width = map[0].len();

    match direction {
        North => if let Some(row) = (0..width).rposition(|row| map[row][coord.1] != '.') { Some((row, coord.1)) } else { None },
        East => if let Some(col) = (coord.1..width).find(|&col| map[coord.0][col] != '.') { Some((coord.0, col)) } else { None }
        South => if let Some(row) = (coord.0..height).find(|&row| map[row][coord.1] != '.') { Some((row, coord.1)) } else { None }
        West => if let Some(col) = map[coord.0].iter().rposition(|&c| c != '.') { Some((coord.0, col)) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day16/input_example.txt"));

        assert_eq!(puzzle_1(&input), 46);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day16/input.txt"));

        assert_eq!(puzzle_1(&input), 6816);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day16/input_example.txt"));

        assert_eq!(puzzle_2(&input), 51);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day16/input.txt"));

        assert_eq!(puzzle_2(&input), 8163);
    }
}