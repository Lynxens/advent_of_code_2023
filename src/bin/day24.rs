use std::ops::{Add, Mul};
use itertools::Itertools;
use z3::{Config, Context, Solver};
use z3::ast::{Ast, Real, Int};

fn main() {
    let input = parse(include_str!("../../data/day24/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input, (200000000000000.0, 400000000000000.0)));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

struct Trajectory {
    x0: i64, y0: i64, z0: i64,
    dx: i64, dy: i64, dz: i64,
}

fn parse(raw_input: &str) -> Vec<Trajectory>
{
    raw_input
        .lines()
        .map(|l| {
            let (position, velocity) = l.split_once(" @ ").unwrap();
            let (x0, y0, z0) = position.splitn(3, ", ").map(|v| v.parse::<i64>().unwrap()).collect_tuple().unwrap();
            let (dx, dy, dz) = velocity.splitn(3, ", ").map(|v| v.parse::<i64>().unwrap()).collect_tuple().unwrap();

            Trajectory {
                x0, y0, z0,
                dx, dy, dz,
            }
        })
        .collect()
}

fn puzzle_1(data: &[Trajectory], boundary: (f64, f64)) -> usize {
    data
        .iter()
        .tuple_combinations()
        .filter(|&(t1, t2) | {
            // Solve ax + c = bx + d
            let a = t1.dy as f64 / t1.dx as f64;
            let c = t1.y0 as f64 + (a * -t1.x0  as f64);
            let b = t2.dy as f64 / t2.dx as f64;
            let d = t2.y0 as f64 + (b * -t2.x0 as f64);

            // Handle parallel lines
            if a == b {
                return c == d;
            }

            let intersection_x = (d - c) / (a - b);
            let intersection_y = a * intersection_x + c;

            let intersection_t1 = (intersection_x - t1.x0 as f64) / t1.dx as f64;
            let intersection_t2 = (intersection_x - t2.x0 as f64) / t2.dx as f64;

            intersection_t1 >= 0.0 && intersection_t2 >= 0.0 && intersection_x >= boundary.0 && intersection_x <= boundary.1 && intersection_y >= boundary.0 && intersection_y <= boundary.1
        })
        .count()
}

fn puzzle_2(data: &[Trajectory]) -> usize {
    let config = Config::new();
    let ctx = Context::new(&config);
    let solver = Solver::new(&ctx);

    let rock_x0 = Real::new_const(&ctx, "rock_x0");
    let rock_y0 = Real::new_const(&ctx, "rock_y0");
    let rock_z0 = Real::new_const(&ctx, "rock_z0");
    let rock_dx = Real::new_const(&ctx, "rock_dx");
    let rock_dy = Real::new_const(&ctx, "rock_dy");
    let rock_dz = Real::new_const(&ctx, "rock_dz");
    let zero = Real::from_int(&Int::from_i64(&ctx, 0));

    for (i, t) in data[..3].iter().enumerate() {
        let collision_time = Real::new_const(&ctx, format!("ct{i}"));
        let x0 = Real::from_int(&Int::from_i64(&ctx, t.x0));
        let y0 = Real::from_int(&Int::from_i64(&ctx, t.y0));
        let z0 = Real::from_int(&Int::from_i64(&ctx, t.z0));
        let dx = Real::from_int(&Int::from_i64(&ctx, t.dx));
        let dy = Real::from_int(&Int::from_i64(&ctx, t.dy));
        let dz = Real::from_int(&Int::from_i64(&ctx, t.dz));

        solver.assert(&collision_time.gt(&zero));
        solver.assert(&rock_x0.clone().add(&collision_time.clone().mul(&rock_dx))._eq(&x0.add(&collision_time.clone().mul(&dx))));
        solver.assert(&rock_y0.clone().add(&collision_time.clone().mul(&rock_dy))._eq(&y0.add(&collision_time.clone().mul(&dy))));
        solver.assert(&rock_z0.clone().add(&collision_time.clone().mul(&rock_dz))._eq(&z0.add(&collision_time.clone().mul(&dz))));
    }

    solver.check();

    let model = solver.get_model().unwrap();

    let x0_f = model.get_const_interp(&rock_x0).unwrap().to_string().parse::<f64>().unwrap();
    let y0_f = model.get_const_interp(&rock_y0).unwrap().to_string().parse::<f64>().unwrap();
    let z0_f = model.get_const_interp(&rock_z0).unwrap().to_string().parse::<f64>().unwrap();

    (x0_f + y0_f + z0_f) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day24/input_example.txt"));

        assert_eq!(puzzle_1(&input, (7.0, 27.0)), 2);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day24/input.txt"));

        assert_eq!(puzzle_1(&input, (200000000000000.0, 400000000000000.0)), 11246);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day24/input_example.txt"));

        assert_eq!(puzzle_2(&input), 47);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day24/input.txt"));

        assert_eq!(puzzle_2(&input), 716599937560103);
    }
}