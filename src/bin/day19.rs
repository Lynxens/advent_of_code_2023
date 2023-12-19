use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;
use nom::Slice;
use regex::Regex;

fn main() {
    let input = parse(include_str!("../../data/day19/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

type Categories = [usize; 4];
type CategoryRanges = [(usize, usize); 4];

struct Rule {
    category: usize,
    equation: Ordering,
    right_value: usize,
    next_workflow: String,
}

impl Rule {
    fn eval(&self, categories: &Categories) -> Option<&str> {
        let value = categories[self.category];

        if match self.equation {
            Ordering::Less => value < self.right_value,
            Ordering::Equal => value == self.right_value,
            Ordering::Greater => value > self.right_value,
        } {
            return Some(&self.next_workflow);
        }

        None
    }

    fn eval_range(&self, category_ranges: &CategoryRanges) -> (&str, Option<CategoryRanges>, Option<CategoryRanges>) {
        let mut true_category_ranges = category_ranges.clone();
        let mut false_category_ranges = category_ranges.clone();
        let (range_min, range_max) = category_ranges[self.category];

        return if self.equation == Ordering::Less {
            if range_min < self.right_value {
                true_category_ranges[self.category] = (range_min, self.right_value - 1);

                if range_max >= self.right_value {
                    false_category_ranges[self.category] = (self.right_value, range_max);

                    (&self.next_workflow, Some(true_category_ranges), Some(false_category_ranges))
                } else {
                    (&self.next_workflow, Some(true_category_ranges), None)
                }
            } else {
                (&self.next_workflow, None, Some(false_category_ranges))
            }
        } else {
            if range_max > self.right_value {
                true_category_ranges[self.category] = (self.right_value + 1, range_max);

                if range_min <= self.right_value {
                    false_category_ranges[self.category] = (range_min, self.right_value);
                    (&self.next_workflow, Some(true_category_ranges), Some(false_category_ranges))
                } else {
                    (&self.next_workflow, Some(true_category_ranges), None)
                }
            } else {
                (&self.next_workflow, None, Some(false_category_ranges))
            }
        }
    }
}

struct Workflows {
    workflow_map: HashMap<String, (Vec<Rule>, String)>
}

impl Workflows {
    fn part_is_accepted(&self, categories: &Categories) -> bool {
        let mut workflow_name = "in";

        'workflow_loop: loop {
            let (rules, otherwise) = self.workflow_map.get(workflow_name).unwrap();

            for rule in rules {
                if let Some(new_workflow_name) = rule.eval(categories) {
                    match new_workflow_name {
                        "A" => return true,
                        "R" => return false,
                        _ => {
                            workflow_name = new_workflow_name;
                            continue 'workflow_loop;
                        },
                    }
                }
            }

            match otherwise.as_str() {
                "A" => return true,
                "R" => return false,
                _ => workflow_name = otherwise,
            }
        }
    }

    fn count_accepted_combinations(&self, workflow_name: &str, start_category_ranges: &CategoryRanges) -> usize {
        match workflow_name {
            "A" => start_category_ranges.map(|(range_min, range_max)| range_max - range_min + 1).iter().product(),
            "R" => 0,
            _ => {
                let mut category_ranges = start_category_ranges.clone();
                let mut total_count = 0;

                let (rules, otherwise) = self.workflow_map.get(workflow_name).unwrap();

                for rule in rules {
                    let (new_workflow_name, true_category_ranges, false_category_ranges) = rule.eval_range(&category_ranges);

                    if true_category_ranges.is_some() {
                        total_count += self.count_accepted_combinations(new_workflow_name, &true_category_ranges.unwrap());
                    }

                    if false_category_ranges.is_none() {
                        return total_count;
                    }

                    category_ranges = false_category_ranges.unwrap();
                }

                match otherwise.as_str() {
                    "A" => total_count + category_ranges.map(|(range_min, range_max)| range_max - range_min + 1).iter().product::<usize>(),
                    "R" => total_count,
                    _ => total_count + self.count_accepted_combinations(otherwise, &category_ranges),
                }
            }
        }
    }
}

fn parse(raw_input: &str) -> (Workflows, Vec<Categories>)
{
    let (workflows, parts) = raw_input.split_once("\n\n").unwrap();

    let workflow_re = Regex::new(r"(?P<name>\w+)\{(?P<rules>.*),(?P<otherwise>\w+)}").unwrap();
    let rule_re = Regex::new(r"(?P<category>\w)(?P<equation>[<>])(?P<right>\d+):(?P<next>\w+)").unwrap();
    let workflow_map = HashMap::from_iter(
        workflows
            .lines()
            .map(|l| {
                let captures = workflow_re.captures(l).unwrap();
                (
                    String::from(&captures["name"]),
                    (
                        captures.name("rules").unwrap().as_str()
                            .split(",")
                            .map(|r| {
                                let rule_captures = rule_re.captures(r).unwrap();

                                Rule {
                                    category: match &rule_captures["category"] {
                                        "x" => Ok(0),
                                        "m" => Ok(1),
                                        "a" => Ok(2),
                                        "s" => Ok(3),
                                        _ => Err("Unexpected category")
                                    }.expect(""),
                                    equation: match &rule_captures["equation"] {
                                        ">" => Ok(Ordering::Greater),
                                        "<" => Ok(Ordering::Less),
                                        _ => Err("Unexpected equation"),
                                    }.expect(""),
                                    right_value: rule_captures["right"].parse::<usize>().unwrap(),
                                    next_workflow: String::from(&rule_captures["next"]),
                                }
                            })
                            .collect::<Vec<Rule>>(),
                        String::from(&captures["otherwise"]),
                    ),
                )
            })
    );

    (
        Workflows { workflow_map },
        parts
            .lines()
            .map(|l| {
                let (x, m, a, s) = l.slice(1..l.len() - 1).splitn(4, ',').collect_tuple().unwrap();

                [
                    x.split_once('=').unwrap().1.parse::<usize>().unwrap(),
                    m.split_once('=').unwrap().1.parse::<usize>().unwrap(),
                    a.split_once('=').unwrap().1.parse::<usize>().unwrap(),
                    s.split_once('=').unwrap().1.parse::<usize>().unwrap(),
                ]
            })
            .collect()
    )
}

fn puzzle_1((workflows, categories): &(Workflows, Vec<Categories>)) -> usize {
    categories
        .iter()
        .filter(|&c| workflows.part_is_accepted(c))
        .map(|c| c.iter().sum::<usize>())
        .sum()
}

fn puzzle_2((workflows, _): &(Workflows, Vec<Categories>)) -> usize {
    workflows.count_accepted_combinations("in", &[(1, 4000); 4])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day19/input_example.txt"));

        assert_eq!(puzzle_1(&input), 19114);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day19/input.txt"));

        assert_eq!(puzzle_1(&input), 330820);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day19/input_example.txt"));

        assert_eq!(puzzle_2(&input), 167409079868000);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day19/input.txt"));

        assert_eq!(puzzle_2(&input), 123972546935551);
    }
}