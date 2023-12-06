use std::iter::zip;

use aoc_utils::AocDay;

fn main() {
    Day06::run();
}

struct Day06;

impl AocDay<usize, usize> for Day06 {
    const DAY: u8 = 6;

    fn part_1(input: &str) -> usize {
        parse_input_part_1(input)
            .iter()
            .map(Race::count_winning_presses)
            .product()
    }

    fn part_2(input: &str) -> usize {
        parse_input_part_2(input).count_winning_presses()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn new(time: u64, record: u64) -> Self {
        Self { time, record }
    }

    pub fn distance_with_press(&self, duration: u64) -> u64 {
        let speed = duration;
        let time_to_move = self.time.saturating_sub(duration);
        time_to_move * speed
    }

    pub fn count_winning_presses(&self) -> usize {
        (0..self.time)
            .filter(|duration| self.distance_with_press(*duration) > self.record)
            .count()
    }
}

fn parse_input_part_1(input: &str) -> Vec<Race> {
    let (time_line, distance_line) = input.trim().split_once('\n').unwrap();

    zip(
        parse_line_part_1(time_line),
        parse_line_part_1(distance_line),
    )
    .map(|(time, distance)| Race::new(time, distance))
    .collect()
}

fn parse_line_part_1(line: &str) -> Vec<u64> {
    let (_, raw_nums) = line.split_once(':').unwrap();
    raw_nums
        .trim()
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse::<u64>().unwrap())
        .collect()
}

fn parse_input_part_2(input: &str) -> Race {
    let (time_line, distance_line) = input.trim().split_once('\n').unwrap();

    Race {
        time: parse_line_part_2(time_line),
        record: parse_line_part_2(distance_line),
    }
}

fn parse_line_part_2(line: &str) -> u64 {
    let (_, raw_nums) = line.split_once(':').unwrap();
    let parts: Vec<_> = raw_nums.trim().split_ascii_whitespace().collect();
    parts.join("").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse_input_part_1() {
        assert_eq!(
            parse_input_part_1(INPUT),
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        )
    }

    #[test]
    fn test_parse_input_part_2() {
        assert_eq!(parse_input_part_2(INPUT), Race::new(71530, 940200))
    }

    #[test]
    fn test_part_1() {
        assert_eq!(Day06::part_1(INPUT), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day06::part_2(INPUT), 71503);
    }
}
