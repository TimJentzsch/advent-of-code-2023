use aoc_utils::AocDay;

fn main() {
    Day04::run();
}

struct Day04;

impl AocDay<u32, ()> for Day04 {
    const DAY: u8 = 4;

    fn part_1(input: &str) -> u32 {
        input
            .trim()
            .lines()
            .map(|line| {
                let (_, second) = line.split_once(':').expect("No : contained in line");
                let (winning, draw) = second.split_once('|').expect("No | contained in line");
                let winning_numbers = parse_number_list(winning);
                let drawn_numbers = parse_number_list(draw);
                let drawn_winning = drawn_numbers
                    .iter()
                    .filter(|num| winning_numbers.contains(num))
                    .collect::<Vec<_>>();

                let mut score = 0;
                for _ in drawn_winning {
                    score = match score {
                        0 => 1,
                        _ => score * 2,
                    };
                }
                score
            })
            .sum()
    }

    fn part_2(_input: &str) {
        todo!()
    }
}

fn parse_number_list(input: &str) -> Vec<u32> {
    input
        .split(' ')
        .filter_map(|segment| segment.parse::<u32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(Day04::part_1(input), 13);
    }
}
