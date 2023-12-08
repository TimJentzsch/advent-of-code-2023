use core::panic;
use std::collections::HashMap;

use aoc_utils::AocDay;

fn main() {
    Day08::run();
}

struct Day08;

impl AocDay<usize, ()> for Day08 {
    const DAY: u8 = 8;

    fn part_1(input: &str) -> usize {
        let (instructions, rest) = input.split_once("\n\n").unwrap();

        let map: HashMap<_, _> = rest
            .lines()
            .map(|line| {
                let (node, rest) = line.split_once(" = ").unwrap();
                let (left, right) = rest
                    .split_once(", ")
                    .map(|(left, right)| {
                        (left.trim_start_matches('('), right.trim_end_matches(')'))
                    })
                    .unwrap();

                (node, (left, right))
            })
            .collect();

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

    fn part_2(_input: &str) {
        todo!()
    }
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
}
