use std::cmp::{max, min, Ordering};
use itertools::Itertools;

fn main() {
    let input = parse(include_str!("../../data/day22/input.txt"));

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input));
}

fn parse(raw_input: &str) -> Vec<Block>
{
    raw_input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('~').unwrap();

            let (ax, ay, az) = a
                .splitn(3, ',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            let (bx, by, bz) = b
                .splitn(3, ',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            Block {
                x0: min(ax, bx),
                x1: max(ax, bx),
                y0: min(ay, by),
                y1: max(ay, by),
                z0: min(az, bz),
                z1: max(az, bz),
            }
        })
        .sorted()
        .collect()
}

fn puzzle_1(data: &[Block]) -> usize {
    let mut tree: BlockTree = Default::default();

    data.iter().for_each(|b| tree.add_block(b));

    tree.nodes
        .iter()
        .filter(|&n| n.children.iter().all(|&c| tree.nodes[c].parents.len() > 1))
        .count()
}

fn puzzle_2(data: &[Block]) -> usize {
    let mut tree: BlockTree = Default::default();

    data.iter().for_each(|b| tree.add_block(b));

    let skips: Vec<usize> = tree.nodes
        .iter()
        .filter(|&n| n.children.iter().all(|&c| tree.nodes[c].parents.len() > 1))
        .map(|n| n.block_index)
        .collect();

    tree
        .nodes
        .iter()
        .filter(|&n| !skips.contains(&n.block_index))
        .map(|start_node| {
            let mut removed_blocks = Vec::from([start_node.block_index]);

            loop {
                let falling_blocks: Vec<usize> = tree.nodes
                    .iter()
                    .filter(|&n| n.parents.len() > 0)
                    .filter(|&n| !removed_blocks.contains(&n.block_index))
                    .filter(|&n| n.parents.iter().all(|p| removed_blocks.contains(p)))
                    .map(|n| n.block_index)
                    .collect();

                if falling_blocks.len() == 0 {
                    break;
                }

                removed_blocks.extend(falling_blocks);
            }

            removed_blocks.len() - 1
        })
        .sum()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Block { x0: usize, x1: usize, y0: usize, y1: usize, z0: usize, z1: usize }
impl Block {
    fn has_intersection(&self, other: &Self) -> bool {
        max(self.x0, other.x0) <= min(self.x1, other.x1) && max(self.y0, other.y0) <= min(self.y1, other.y1)
    }

    fn fall_to(&self, z: usize) -> Self {
        Self {
            x0: self.x0,
            x1: self.x1,
            y0: self.y0,
            y1: self.y1,
            z0: z + 1,
            z1: z + 1 + (self.z1 - self.z0),
        }
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z0.cmp(&other.z0)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

struct Node { block_index: usize, parents: Vec<usize>, children: Vec<usize> }
struct BlockTree { blocks: Vec<Block>, nodes: Vec<Node>, ground: Vec<usize> }
impl Default for BlockTree {
    fn default() -> Self {
        Self { blocks: Default::default(), nodes: Vec::new(), ground: Vec::new() }
    }
}

impl BlockTree {
    fn add_block(&mut self, block: &Block) {
        let own_index = self.blocks.len();
        let sorted_blocks: Vec<&Block> = self.blocks.iter().sorted_by(|&a, &b| b.z1.cmp(&a.z1)).collect();
        if let Some(b) = sorted_blocks.into_iter().find(|&b| b.has_intersection(block)) {
            let mut node = Node {
                block_index: own_index,
                parents: vec![],
                children: vec![],
            };

            self.blocks
                .iter()
                .enumerate()
                .filter(|&(_, ob)| ob.z1 == b.z1 && ob.has_intersection(block))
                .for_each(|(j, _)| {
                    node.parents.push(j);
                    self.nodes[j].children.push(own_index);
                });
            self.blocks.push(block.fall_to(b.z1));
            self.nodes.push(node);
        } else {
            self.nodes.push(Node {
                block_index: own_index,
                parents: vec![],
                children: vec![],
            });
            self.ground.push(own_index);
            self.blocks.push(block.fall_to(0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_example() {
        let input = parse(include_str!("../../data/day22/input_example.txt"));

        assert_eq!(puzzle_1(&input), 5);
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse(include_str!("../../data/day22/input.txt"));

        assert_eq!(puzzle_1(&input), 519);
    }

    #[test]
    fn test_puzzle_2_example() {
        let input = parse(include_str!("../../data/day22/input_example.txt"));

        assert_eq!(puzzle_2(&input), 7);
    }

    #[test]
    fn test_puzzle_2() {
        let input = parse(include_str!("../../data/day22/input.txt"));

        assert_eq!(puzzle_2(&input), 109531);
    }
}