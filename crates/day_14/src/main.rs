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

    fn part_2(input: &str) -> usize {
        const ITERATIONS: usize = 1_000_000_000;

        let mut platform = Platform::<SIZE>::parse(input);
        let mut cache = Vec::with_capacity(100);

        for iter in 1..=ITERATIONS {
            cache.push(platform.clone());
            platform = platform.spin_cycle();

            if let Some(cache_idx) = cache.iter().position(|a| a == &platform) {
                let cycle_size = iter - cache_idx;
                let remaining = ITERATIONS - iter;
                let remaining_short = remaining % cycle_size;

                for _ in 0..remaining_short {
                    platform = platform.spin_cycle();
                }

                break;
            }
        }

        platform.total_load()
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

    #[test]
    fn test_part_2() {
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

        assert_eq!(Day14::<10>::part_2(input), 64);
    }
}
