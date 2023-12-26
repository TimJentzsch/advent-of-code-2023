use aoc_utils::AocDay;
use parser::parse_full_schematic;

mod parser;
mod types;

fn main() {
    Day03::run();
}

struct Day03;

impl AocDay<u32, u32> for Day03 {
    const DAY: u8 = 3;

    fn part_1(input: &str) -> u32 {
        parse_full_schematic(input.trim())
            .part_numbers()
            .iter()
            .map(|num| num.value)
            .sum()
    }

    fn part_2(input: &str) -> u32 {
        parse_full_schematic(input.trim()).gear_ratios()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(Day03::part_1(input), 4361);
    }

    #[test]
    fn test_part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(Day03::part_2(input), 467835);
    }
}
