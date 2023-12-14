use aoc_utils::AocDay;
use types::Platform;

mod types;

fn main() {
    Day14::<100>::run();
}

struct Day14<const SIZE: usize>;

impl<const SIZE: usize> AocDay<usize, usize> for Day14<SIZE> {
    const DAY: u8 = 14;

    fn part_1(input: &str) -> usize {
        Platform::<SIZE>::parse(input).tilt_north().total_load()
    }

    fn part_2(_input: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(Day14::<10>::part_1(input), 136);
    }
}
