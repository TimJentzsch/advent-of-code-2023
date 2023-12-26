use std::fmt::Debug;

use aoc_utils::AocDay;

fn main() {
    Day10::run();
}

struct Day10;

impl AocDay<usize, usize> for Day10 {
    const DAY: u8 = 10;

    fn part_1(_input: &str) -> usize {
        todo!()
    }

    fn part_2(_input: &str) -> usize {
        todo!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NorthSouth => write!(f, "|"),
            Self::EastWest => write!(f, "-"),
            Self::NorthEast => write!(f, "L"),
            Self::NorthWest => write!(f, "J"),
            Self::SouthWest => write!(f, "7"),
            Self::SouthEast => write!(f, "F"),
            Self::Start => write!(f, "S"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    East,
    South,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Index {
    row: usize,
    col: usize,
}

impl Index {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn add_dir(&self, dir: Direction) -> Self {
        match dir {
            // Using wrapping_sub to ensure that the index is out of bounds if not in the grid
            Direction::North => Self::new(self.row, self.col.wrapping_sub(1)),
            Direction::West => Self::new(self.row.wrapping_sub(1), self.col),
            Direction::East => Self::new(self.row + 1, self.col),
            Direction::South => Self::new(self.row, self.col + 1),
        }
    }
}

struct Map<const SIZE: usize> {
    grid: [[Option<Pipe>; SIZE]; SIZE],
}

impl<const SIZE: usize> Map<SIZE> {
    pub fn parse(input: &str) -> Self {
        let mut grid = [[None; SIZE]; SIZE];

        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                let pipe = match ch {
                    '.' => None,
                    '|' => Some(Pipe::NorthSouth),
                    '-' => Some(Pipe::EastWest),
                    'L' => Some(Pipe::NorthEast),
                    'J' => Some(Pipe::NorthWest),
                    '7' => Some(Pipe::SouthWest),
                    'F' => Some(Pipe::SouthEast),
                    'S' => Some(Pipe::Start),
                    _ => panic!("Unexpected character {ch}"),
                };

                grid[row][col] = pipe;
            });
        });

        Self { grid }
    }

    fn get(&self, index: Index) -> Option<Pipe> {
        if let Some(row) = self.grid.get(index.row) {
            if let Some(pipe) = row.get(index.col) {
                *pipe
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<const SIZE: usize> Debug for Map<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .flat_map(|row| {
                    row.iter().map(|pipe| {
                        if let Some(pipe) = pipe {
                            format!("{pipe:?}")
                        } else {
                            ".".to_string()
                        }
                    })
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
