use aoc_utils::AocDay;
use types::Player;

mod types;

fn main() {
    Day07::run();
}

struct Day07;

impl AocDay<u32, u32> for Day07 {
    const DAY: u8 = 7;

    fn part_1(input: &str) -> u32 {
        total_winnings(input, false)
    }

    fn part_2(input: &str) -> u32 {
        total_winnings(input, true)
    }
}

fn total_winnings(input: &str, part_2: bool) -> u32 {
    let mut list: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand_str, bid_str) = line.split_once(' ').unwrap();
            Player::new(hand_str, bid_str.parse().unwrap(), part_2)
        })
        .collect();

    list.sort_unstable();

    list.iter()
        .enumerate()
        .map(|(idx, player)| (idx as u32 + 1) * player.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_1() {
        assert_eq!(Day07::part_1(INPUT), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day07::part_2(INPUT), 5905);
    }
}
