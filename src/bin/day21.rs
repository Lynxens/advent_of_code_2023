use std::collections::{HashMap, HashSet, VecDeque};
use advent_of_code_2023::{Coordinate, von_neumann_compass};

fn main() {
    let input = parse(include_str!("../../data/day21/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input, 64));
    println!("Puzzle 2: {}", puzzle_2(&input, 26501365));
}

fn parse(raw_input: &str) -> Vec<Vec<char>>
{
    raw_input
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn puzzle_1(data: &[Vec<char>], steps: usize) -> usize {
    let mut garden = Garden::from_data(data, steps);

    for _ in 0..steps / 2 {
        garden.set_two_steps();
    }

    garden.garden_plot_count()
}

fn puzzle_2(data: &[Vec<char>], total_steps: usize) -> usize {
    let height = data.len();
    let width = data[0].len();

    let start = data
        .iter()
        .flatten()
        .position(|&c| c == 'S')
        .unwrap();

    let start_coord = (start / width, start % width);

    let mut steps_to_plot: HashMap<Coordinate, usize> = HashMap::new();
    let mut queue: VecDeque<(Coordinate, usize)> = VecDeque::from([(start_coord, 0)]);

    while let Some((c, steps)) = queue.pop_front() {
        if steps_to_plot.contains_key(&c) {
            continue;
        }

        steps_to_plot.insert(c, steps);

        von_neumann_compass(c, height, width)
            .into_iter()
            .filter(|&(_, (ny, nx))| data[ny][nx] != '#')
            .for_each(|(_, nc)| {
                if !steps_to_plot.contains_key(&nc) {
                    queue.push_back((nc, steps + 1));
                }
            });
    }

    let full_gardens_in_each_direction = total_steps / width;
    let offset_in_corners = total_steps % width;

    let plot_count_even_full = steps_to_plot
        .values()
        .filter(|&&steps| steps % 2 == 0)
        .count();

    let plot_count_odd_full = steps_to_plot
        .values()
        .filter(|&&steps| steps % 2 == 1)
        .count();

    let plot_count_even_corner = steps_to_plot
        .iter()
        .filter(|&(c, &steps)| steps % 2 == 0 && steps > offset_in_corners)
        .count();

    let plot_count_odd_corner = steps_to_plot
        .iter()
        .filter(|&(c, &steps)| steps % 2 == 1 && steps > offset_in_corners)
        .count();

    full_gardens_in_each_direction.pow(2) * plot_count_even_full
        + (full_gardens_in_each_direction + 1).pow(2) * plot_count_odd_full
        + full_gardens_in_each_direction * plot_count_even_corner
        - (full_gardens_in_each_direction + 1) * plot_count_odd_corner
        - full_gardens_in_each_direction // ???
}

type CoordinateI64 = (i64, i64);

struct Garden {
    rock_map: Vec<Vec<char>>,
    height: usize,
    width: usize,
    visited: HashSet<CoordinateI64>,
    queue: VecDeque<CoordinateI64>,
}

impl Garden {
    fn from_data(data: &[Vec<char>], total_steps: usize) -> Self {
        let mut garden = Garden {
            rock_map: Vec::from(data),
            height: data.len(),
            width: data[0].len(),
            visited: HashSet::new(),
            queue: VecDeque::new(),
        };

        let start = data
            .iter()
            .flatten()
            .position(|&c| c == 'S')
            .unwrap();

        let start_y = (start / garden.width) as i64;
        let start_x = (start % garden.width) as i64;

        if total_steps % 2 == 0 {
            garden.visited.insert((start_y, start_x));
            garden.queue.push_back((start_y, start_x));
        } else {
            garden.von_neumann((start_y, start_x))
                .into_iter()
                .for_each(|((ny, nx), (my, mx))| {
                    if data[my][mx] != '#' {
                        garden.visited.insert((ny, nx));
                        garden.queue.push_back((ny, nx));
                    }
                });
        }

        garden
    }

    fn set_two_steps(&mut self) {
        let mut first_step_queue: VecDeque<CoordinateI64> = VecDeque::new();
        while let Some(c) = self.queue.pop_front() {
            first_step_queue.extend(
                self.von_neumann(c)
                    .into_iter()
                    .filter_map(|((ny, nx), (my, mx))| {
                        if self.rock_map[my][mx] != '#' {
                            Some((ny, nx))
                        } else {
                            None
                        }
                    })
            );
        }

        while let Some(c) = first_step_queue.pop_front() {
            self.von_neumann(c)
                .into_iter()
                .for_each(|((ny, nx), (my, mx))| {
                    if self.rock_map[my][mx] == '#' {
                        return
                    }

                    if self.visited.insert((ny, nx)) {
                        self.queue.push_back((ny, nx));
                    }
                })
        }
    }

    fn von_neumann(&self, (y, x): CoordinateI64) -> Vec<(CoordinateI64, Coordinate)> {
        [
            (y - 1, x),
            (y, x + 1),
            (y + 1, x),
            (y, x - 1),
        ]
            .into_iter()
            .map(|(ny, nx)| (
                (ny, nx),
                (ny.rem_euclid(self.height as i64) as usize, nx.rem_euclid(self.width as i64) as usize)
            ))
            .collect()
    }

    fn garden_plot_count(&self) -> usize {
        self.visited.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day21/input_example.txt"));

        assert_eq!(puzzle_1(&input, 6), 16);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day21/input.txt"));

        assert_eq!(puzzle_1(&input, 64), 3689);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day21/input.txt"));

        assert_eq!(puzzle_2(&input, 26501365), 610158187362102);
    }
}