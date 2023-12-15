#[derive(Clone, Default)]
struct LensBox {
    lenses: Vec<(String, usize)>,
}

impl LensBox {
    fn find_lens_position(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|(l, _)| l == label)
    }

    fn add_lens(&mut self, label: &str, focal_length: usize) {
        if let Some(index) = self.find_lens_position(label) {
            self.lenses[index] = (label.to_string(), focal_length);
        } else {
            self.lenses.push((label.to_string(), focal_length));
        }
    }

    fn remove_lens(&mut self, label: &str) {
        if let Some(index) = self.find_lens_position(label) {
            self.lenses.remove(index);
        }
    }
}

fn main() {
    let input = parse(include_str!("../../data/day15/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn parse(raw_input: &str) -> Vec<&str>
{
    raw_input
        .split(',')
        .collect()
}

fn puzzle_1(data: &[&str]) -> usize {
    data
        .iter()
        .map(|&s| hash(s))
        .sum()
}

fn puzzle_2(data: &[&str]) -> usize {
    data
        .iter()
        .fold(vec![Default::default(); 256], |mut boxes: Vec<LensBox>, &step| {
            if let Some((label, _)) = step.split_once('-') {
                boxes[hash(label)].remove_lens(label);
            } else {
                let (label, focal_length) = step.split_once('=').unwrap();

                boxes[hash(label)].add_lens(label, focal_length.parse::<usize>().unwrap());
            }

            boxes
        })
        .into_iter()
        .zip(1..)
        .map(|(lens_box, box_id)| lens_box
            .lenses
            .into_iter()
            .zip(1..)
            .map(|((_, focal_length), slot)| box_id * slot * focal_length)
            .sum::<usize>()
        )
        .sum()
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |value, c| ((value + (c as usize)) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day15/input_example.txt"));

        assert_eq!(puzzle_1(&input), 1320);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day15/input.txt"));

        assert_eq!(puzzle_1(&input), 513172);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day15/input_example.txt"));

        assert_eq!(puzzle_2(&input), 145);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day15/input.txt"));

        assert_eq!(puzzle_2(&input), 237806);
    }
}