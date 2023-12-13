use std::slice::Iter;
use crate::Direction::{East, North, South, West};

pub fn moore_neighborhood((y, x): (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    [
        (y.wrapping_sub(1), x.wrapping_sub(1)),
        (y.wrapping_sub(1), x),
        (y.wrapping_sub(1), x + 1),
        (y, x.wrapping_sub(1)),
        (y, x + 1),
        (y + 1, x.wrapping_sub(1)),
        (y + 1, x),
        (y + 1, x + 1),
    ]
        .into_iter()
        .filter(|&(ny, nx)| ny < height && nx < width)
        .collect()
}

pub fn transpose<T: Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut row_iters: Vec<Iter<T>> = matrix
        .iter()
        .map(|r| r.iter())
        .collect();

    (0..matrix.first().unwrap().len())
        .map(|_| row_iters
            .iter_mut()
            .map(|it| *it.next().unwrap())
            .collect::<Vec<T>>()
        )
        .collect()
}

pub fn count_different_elements<T: Eq>(a: &[T], b: &[T]) -> usize {
    a
        .iter()
        .zip(b)
        .filter(|&(element_a, element_b) | element_a != element_b)
        .count()
}

pub fn get_boundary_coordinates(height: usize, width: usize) -> Vec<(usize, usize)> {
    let top: Vec<(usize, usize)> = vec![0; width]
        .into_iter()
        .zip(0..width)
        .collect();

    let bottom: Vec<(usize, usize)> = vec![height - 1; width]
        .into_iter()
        .zip(0..width)
        .collect();

    let left: Vec<(usize, usize)> = (0..height)
        .zip(vec![0; height])
        .collect();

    let right: Vec<(usize, usize)> = (0..height)
        .zip(vec![width - 1; height])
        .collect();

    Vec::from_iter([top, right, bottom, left].into_iter().flatten())
}

pub type Coordinate = (usize, usize);
pub type Compass = Vec<(Direction, Coordinate)>;

#[derive(Clone, Copy)]
pub enum Direction { North, East, South, West }
impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn rotate_clockwise(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

pub fn von_neumann_compass((y, x): (usize, usize), height: usize, width: usize) -> Compass {
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

pub fn coordinate_in_direction(coordinate: &Coordinate, direction: &Direction, height: usize, width: usize) -> Option<Coordinate> {
    let next_coordinate = match direction {
        North => (coordinate.0.wrapping_sub(1), coordinate.1),
        East => (coordinate.0, coordinate.1 + 1),
        South => (coordinate.0 + 1, coordinate.1),
        West => (coordinate.0, coordinate.1.wrapping_sub(1)),
    };

    if next_coordinate.0 < height && next_coordinate.1 < width { Some(next_coordinate) } else { None }
}