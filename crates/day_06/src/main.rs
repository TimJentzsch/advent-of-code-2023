use std::iter::zip;

use aoc_utils::AocDay;

fn main() {
    Day06::run();
}

struct Day06;

impl AocDay<usize, ()> for Day06 {
    const DAY: u8 = 6;

    fn part_1(input: &str) -> usize {
        parse_input(input)
            .iter()
            .map(Race::count_winning_presses)
            .product()
    }

    fn part_2(_input: &str) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u32,
    record: u32,
}

impl Race {
    pub fn new(time: u32, record: u32) -> Self {
        Self { time, record }
    }

    pub fn distance_with_press(&self, duration: u32) -> u32 {
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

fn parse_input(input: &str) -> Vec<Race> {
    let (time_line, distance_line) = input.trim().split_once('\n').unwrap();

    zip(parse_line(time_line), parse_line(distance_line))
        .map(|(time, distance)| Race::new(time, distance))
        .collect()
}

fn parse_line(line: &str) -> Vec<u32> {
    let (_, raw_nums) = line.split_once(':').unwrap();
    raw_nums
        .trim()
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(INPUT),
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(Day06::part_1(INPUT), 288);
    }
}
