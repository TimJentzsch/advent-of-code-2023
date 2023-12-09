use core::panic;
use std::collections::HashMap;

use aoc_utils::AocDay;

fn main() {
    Day08::run();
}

struct Day08;

impl AocDay<usize, usize> for Day08 {
    const DAY: u8 = 8;

    fn part_1(input: &str) -> usize {
        let (instructions, map) = parse_map(input);

        let mut node = "AAA";
        let mut steps = 0;
        let mut instructions = instructions.chars().cycle();

        while node != "ZZZ" {
            let (left, right) = map
                .get(node)
                .unwrap_or_else(|| panic!("Did not find node {}", node));

            node = match instructions.next().unwrap() {
                'L' => left,
                'R' => right,
                instruction => panic!("Invalid instruction {instruction}"),
            };
            steps += 1;
        }

        steps
    }

    fn part_2(input: &str) -> usize {
        let (instructions, map) = parse_map(input);

        const CACHED_REPETITIONS: usize = 100;

        let full_instructions: HashMap<_, _> = map
            .keys()
            .map(|node| {
                let mut end_node_steps = Vec::new();
                let mut cur_node = node;
                let mut steps = 0;

                if is_end_node(&cur_node) {
                    end_node_steps.push(steps);
                }

                for instruction in instructions
                    .chars()
                    .cycle()
                    .take(instructions.len() * CACHED_REPETITIONS)
                {
                    steps += 1;

                    let (left, right) = map
                        .get(cur_node)
                        .unwrap_or_else(|| panic!("Did not find node {}", cur_node));

                    cur_node = match instruction {
                        'L' => left,
                        'R' => right,
                        instruction => panic!("Invalid instruction {instruction}"),
                    };

                    if is_end_node(&cur_node) {
                        end_node_steps.push(steps);
                    }
                }

                (node, (cur_node, end_node_steps))
            })
            .collect();

        println!("Cache created");

        let mut nodes: Vec<_> = map.keys().filter(is_start_node).collect();
        let mut total_steps = 0;

        'search: loop {
            let node_instructions: Vec<_> = nodes
                .iter()
                .map(|node| full_instructions.get(node).unwrap())
                .collect();

            let (_, end_node_steps) = node_instructions[0];

            for steps in end_node_steps.iter() {
                if node_instructions
                    .iter()
                    .all(|(_, end_steps_2)| end_steps_2.contains(steps))
                {
                    break 'search total_steps + steps;
                }
            }

            nodes = node_instructions
                .into_iter()
                .map(|(next_node, _)| *next_node)
                .collect();

            total_steps += instructions.len() * CACHED_REPETITIONS;
        }
    }
}

fn parse_map(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (instructions, rest) = input.split_once("\n\n").unwrap();

    let map: HashMap<_, _> = rest
        .lines()
        .map(|line| {
            let (node, rest) = line.split_once(" = ").unwrap();
            let (left, right) = rest
                .split_once(", ")
                .map(|(left, right)| (left.trim_start_matches('('), right.trim_end_matches(')')))
                .unwrap();

            (node, (left, right))
        })
        .collect();

    (instructions, map)
}

fn is_start_node(node: &&&str) -> bool {
    node.ends_with('A')
}

fn is_end_node(node: &&&str) -> bool {
    node.ends_with('Z')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(Day08::part_1(input), 2);

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(Day08::part_1(input), 6);
    }

    #[test]
    fn test_part_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(Day08::part_2(input), 6);
    }
}
