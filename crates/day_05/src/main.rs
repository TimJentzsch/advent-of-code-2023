use aoc_utils::AocDay;
use parser::parse_full_almanac;

mod parser;
mod types;

fn main() {
    Day05::run();
}

struct Day05;

impl AocDay<u32, ()> for Day05 {
    const DAY: u8 = 5;

    fn part_1(input: &str) -> u32 {
        let almanac = parse_full_almanac(input);
        almanac
            .seeds
            .iter()
            .map(|seed| almanac.seed_location(*seed))
            .min()
            .unwrap()
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
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(Day05::part_1(input), 35);
    }
}
