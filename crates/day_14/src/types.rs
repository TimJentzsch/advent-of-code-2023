use std::fmt::{Debug, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Platform<const SIZE: usize> {
    entries: [[Option<Rock>; SIZE]; SIZE],
}

impl<const SIZE: usize> Debug for Platform<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..SIZE {
            for col in 0..SIZE {
                let char = if let Some(rock) = self.entries[row][col] {
                    match rock {
                        Rock::Cube => '#',
                        Rock::Round => 'O',
                    }
                } else {
                    '.'
                };

                f.write_char(char)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl<const SIZE: usize> Platform<SIZE> {
    pub fn parse(input: &str) -> Self {
        let mut entries = [[None; SIZE]; SIZE];

        let line_count = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                assert_eq!(line.len(), SIZE);

                line.chars().enumerate().for_each(|(col, char)| {
                    match char {
                        'O' => entries[row][col] = Some(Rock::Round),
                        '#' => entries[row][col] = Some(Rock::Cube),
                        // Default is already [`None`]
                        '.' => (),
                        ch => panic!("Invalid character {ch}"),
                    }
                });
            })
            .count();

        assert_eq!(line_count, SIZE);

        Self { entries }
    }

    pub fn total_load(&self) -> usize {
        (1..=SIZE)
            .map(|rev_row| {
                self.entries[SIZE - rev_row]
                    .iter()
                    .filter(|rock| rock.is_some_and(|rock| rock == Rock::Round))
                    .count()
                    * rev_row
            })
            .sum()
    }

    pub fn spin_cycle(self) -> Self {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }

    pub fn tilt_north(mut self) -> Self {
        for col in 0..SIZE {
            let mut next_free_row = 0;

            for row in 0..SIZE {
                if let Some(rock) = self.entries[row][col] {
                    match rock {
                        Rock::Cube => next_free_row = row + 1,
                        Rock::Round => {
                            self.entries[row][col] = None;

                            debug_assert!(self.entries[next_free_row][col].is_none());
                            self.entries[next_free_row][col] = Some(rock);
                            next_free_row += 1;
                        }
                    }
                }
            }
        }

        self
    }

    pub fn tilt_south(mut self) -> Self {
        for col in 0..SIZE {
            let mut next_free_row = SIZE - 1;

            for row in (0..SIZE).rev() {
                if let Some(rock) = self.entries[row][col] {
                    match rock {
                        Rock::Cube => next_free_row = row.saturating_sub(1),
                        Rock::Round => {
                            self.entries[row][col] = None;

                            debug_assert!(self.entries[next_free_row][col].is_none());
                            self.entries[next_free_row][col] = Some(rock);

                            next_free_row = next_free_row.saturating_sub(1);
                        }
                    }
                }
            }
        }

        self
    }

    pub fn tilt_west(mut self) -> Self {
        for row in 0..SIZE {
            let mut next_free_col = 0;

            for col in 0..SIZE {
                if let Some(rock) = self.entries[row][col] {
                    match rock {
                        Rock::Cube => next_free_col = col + 1,
                        Rock::Round => {
                            self.entries[row][col] = None;

                            debug_assert!(self.entries[row][next_free_col].is_none());
                            self.entries[row][next_free_col] = Some(rock);
                            next_free_col += 1;
                        }
                    }
                }
            }
        }

        self
    }

    pub fn tilt_east(mut self) -> Self {
        for row in 0..SIZE {
            let mut next_free_col = SIZE - 1;

            for col in (0..SIZE).rev() {
                if let Some(rock) = self.entries[row][col] {
                    match rock {
                        Rock::Cube => next_free_col = col.saturating_sub(1),
                        Rock::Round => {
                            self.entries[row][col] = None;

                            debug_assert!(self.entries[row][next_free_col].is_none());
                            self.entries[row][next_free_col] = Some(rock);
                            next_free_col = next_free_col.saturating_sub(1);
                        }
                    }
                }
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_tilt() {
        let platform = Platform::<10>::parse(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );

        let tilted_platform = Platform::<10>::parse(
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....",
        );

        assert_eq!(platform.tilt_north(), tilted_platform);
    }

    #[test]
    fn test_platform_spin_cycle() {
        let mut platform = Platform::<10>::parse(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );

        // Cycle 1
        platform = platform.spin_cycle();
        assert_eq!(
            platform,
            Platform::<10>::parse(
                ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
            )
        );

        // Cycle 2
        platform = platform.spin_cycle();
        assert_eq!(
            platform,
            Platform::<10>::parse(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
            )
        );

        // Cycle 3
        platform = platform.spin_cycle();
        assert_eq!(
            platform,
            Platform::<10>::parse(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
            )
        );
    }
}
