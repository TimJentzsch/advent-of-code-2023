use aoc_utils::AocDay;
use parser::parse_full_game;

mod parser;
mod types;

fn main() {
    Day02::run();
}

struct Day02;

impl AocDay<u32, u32> for Day02 {
    const DAY: u8 = 2;

    fn part_1(input: &str) -> u32 {
        input
            .trim()
            .lines()
            .map(parse_full_game)
            .filter(|game| {
                game.reveals
                    .iter()
                    .all(|reveal| reveal.red() <= 12 && reveal.green() <= 13 && reveal.blue() <= 14)
            })
            .map(|game| game.id)
            .sum()
    }

    fn part_2(input: &str) -> u32 {
        input
            .trim()
            .lines()
            .map(parse_full_game)
            .map(|game| game.min_set_power())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_utils::AocDay;

    use crate::Day02;

    #[test]
    fn part_1() {
        assert_eq!(
            Day02::part_1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(
            Day02::part_2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        )
    }
}
