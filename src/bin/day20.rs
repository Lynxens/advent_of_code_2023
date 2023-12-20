use std::collections::{HashMap, VecDeque};
use nom::Slice;
use advent_of_code_2023::lcm;

fn main() {
    let input = parse(include_str!("../../data/day20/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

enum ModuleType { Broadcast, FlipFlop, Conjunction }

fn parse(raw_input: &str) -> HashMap<&str, (ModuleType, Vec<&str>)> {
    raw_input
        .lines()
        .map(|l| {
            let (component_name, output_list) = l.split_once(" -> ").unwrap();
            let outputs: Vec<&str> = output_list.split(", ").collect();

            match component_name.chars().nth(0).unwrap() {
                '%' => (component_name.slice(1..), (ModuleType::FlipFlop, outputs)),
                '&' => (component_name.slice(1..), (ModuleType::Conjunction, outputs)),
                _ => (component_name, (ModuleType::Broadcast, outputs)),
            }
        })
        .collect()
}

struct Configuration { components: HashMap<String, Box<dyn Module>> }
impl Configuration {
    fn from_map(map: &HashMap<&str, (ModuleType, Vec<&str>)>) -> Self {
        let mut configuration = Self { components: Default::default() };

        configuration.components.insert(
            "button".to_string(),
            Box::new(Button { outputs: Vec::from(["broadcaster".to_string()]) })
        );

        for (&name, (component_type, outputs_slice)) in map {
            let outputs = Vec::from_iter(outputs_slice.iter().map(|&o| o.to_string()));

            let component: Box<dyn Module> = match component_type {
                ModuleType::Broadcast => Box::new(Broadcaster { outputs }),
                ModuleType::FlipFlop => Box::new(FlipFlop { outputs, state: false }),
                ModuleType::Conjunction => {
                    let state = HashMap::from_iter(
                        map
                            .iter()
                            .filter_map(|(&c_name, (_, c_outputs))| {
                                if c_outputs.contains(&name) {
                                    Some((c_name.to_string(), Pulse::Low))
                                } else {
                                    None
                                }
                            })
                    );

                    Box::new(Conjunction { outputs, state })
                },
            };

            configuration.components.insert(name.to_string(), component);
        }

        configuration
    }

    fn send_pulse(&mut self) -> (i32, i32, bool) {
        let mut low_pulse_count = 0;
        let mut high_pulse_count = 0;
        let mut found_rx = false;
        let mut queue = VecDeque::from([(String::from("broadcaster"), Pulse::Low, String::from("button"))]);

        while let Some((component_name, pulse, from)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_pulse_count += 1,
                Pulse::High => high_pulse_count += 1,
            }

            if component_name == "rx" && pulse == Pulse::Low {
                found_rx = true;
                continue;
            }

            if let Some(component) = self.components.get_mut(&component_name) {
                if let Some(new_pulse) = component.handle_pulse(&pulse, &from) {
                    for output in component.get_outputs() {
                        queue.push_back((output.clone(), new_pulse, component_name.clone()));
                    }
                }
            }
        }

        (low_pulse_count, high_pulse_count, found_rx)
    }

    fn send_pulse_until(&mut self, until_component_name: &str, until_pulse: Pulse) -> i64 {
        let mut count = 0;
        let mut found = false;

        while !found {
            let mut queue = VecDeque::from([(String::from("broadcaster"), Pulse::Low, String::from("button"))]);

            while let Some((component_name, pulse, from)) = queue.pop_front() {
                if from == until_component_name && pulse == until_pulse {
                    found = true;
                }

                if let Some(component) = self.components.get_mut(&component_name) {
                    if let Some(new_pulse) = component.handle_pulse(&pulse, &from) {
                        for output in component.get_outputs() {
                            queue.push_back((output.clone(), new_pulse, component_name.clone()));
                        }
                    }
                }
            }

            count += 1;
        }

        count
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pulse { Low, High }

trait Module {
    fn get_outputs(&self) -> &[String];
    fn handle_pulse(&mut self, pulse: &Pulse, from: &str) -> Option<Pulse>;
}

struct Button { outputs: Vec<String> }
impl Module for Button {
    fn get_outputs(&self) -> &[String] {
        &self.outputs
    }

    fn handle_pulse(&mut self, pulse: &Pulse, _: &str) -> Option<Pulse> {
        Some(pulse.clone())
    }
}

struct Broadcaster { outputs: Vec<String> }
impl Module for Broadcaster {
    fn get_outputs(&self) -> &[String] {
        &self.outputs
    }

    fn handle_pulse(&mut self, pulse: &Pulse, _: &str) -> Option<Pulse> {
        Some(pulse.clone())
    }
}

struct FlipFlop { outputs: Vec<String>, state: bool }
impl Module for FlipFlop {
    fn get_outputs(&self) -> &[String] {
        &self.outputs
    }

    fn handle_pulse(&mut self, pulse: &Pulse, _: &str) -> Option<Pulse> {
        if pulse == &Pulse::High {
            return None;
        }

        // Flip state
        self.state = !self.state;

        Some(if self.state { Pulse::High } else { Pulse::Low })
    }
}

struct Conjunction { outputs: Vec<String>, state: HashMap<String, Pulse> }
impl Module for Conjunction {
    fn get_outputs(&self) -> &[String] {
        &self.outputs
    }

    fn handle_pulse(&mut self, pulse: &Pulse, from: &str) -> Option<Pulse> {
        self.state.insert(from.to_string(), pulse.clone());

        Some(if self.state.values().all(|&p| p == Pulse::High) { Pulse::Low } else { Pulse::High })
    }
}

fn puzzle_1(data: &HashMap<&str, (ModuleType, Vec<&str>)>) -> i32 {
    let mut configuration = Configuration::from_map(data);

    let mut total_low_pulse_count = 0;
    let mut total_high_pulse_count = 0;
    for _ in 0..1000 {
        let (low_pulse_count, high_pulse_count, _) = configuration.send_pulse();
        total_low_pulse_count += low_pulse_count;
        total_high_pulse_count += high_pulse_count;
    }

    total_low_pulse_count * total_high_pulse_count
}

fn puzzle_2(data: &HashMap<&str, (ModuleType, Vec<&str>)>) -> i128 {
    let mut configuration = Configuration::from_map(data);

    let component_name_before_rx = data
        .iter()
        .find(|&(_, (_, outputs))| outputs.contains(&"rx"))
        .unwrap()
        .0;

    // Find the iteration where all components that output to the conjunction before rx send a high pulse
    data
        .iter()
        .filter_map(|(&component_name, (_, outputs))| {
            if !outputs.contains(&component_name_before_rx) {
                return None;
            }

            configuration.send_pulse_until(component_name, Pulse::High); // Skip first time it find it to exclude the offset
            Some(configuration.send_pulse_until(component_name, Pulse::High) as i128) // Cycle length
        })
        .reduce(|combined_cycle_length, next_cycle_length| lcm(combined_cycle_length, next_cycle_length))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day20/input_example.txt"));
        let input2 = parse(include_str!("../../data/day20/input_example2.txt"));

        assert_eq!(puzzle_1(&input), 32000000);
        assert_eq!(puzzle_1(&input2), 11687500);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day20/input.txt"));

        assert_eq!(puzzle_1(&input), 712543680);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day20/input.txt"));

        assert_eq!(puzzle_2(&input), 238920142622879);
    }
}