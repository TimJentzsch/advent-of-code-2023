use std::fmt::Debug;
use std::fs;
use std::panic;

pub trait AocDay<P1: Debug, P2: Debug> {
    /// The number of the day that the solution is for.
    const DAY: u8;

    /// The implementation of the first part of the puzzle.
    fn part_1(input: &str) -> P1;

    /// The implementation of the second part of the puzzle.
    fn part_2(input: &str) -> P2;

    /// Try to get the input file for this day.
    fn get_input() -> String {
        let file_name = format!("day_{:0>2}.txt", Self::DAY);

        fs::read_to_string(format!("inputs/{file_name}")).unwrap_or_else(|_| {
            panic!("Failed to read input file, make sure to add it at /inputs/{file_name}",)
        })
    }

    fn run() {
        let input = Self::get_input();

        eprint!("PART 1: ");
        eprintln!("{:?}", Self::part_1(&input));

        eprint!("PART 2: ");
        eprintln!("{:?}", Self::part_2(&input));
    }
}
