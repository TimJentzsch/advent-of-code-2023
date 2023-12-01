use std::fs;
use std::panic;

pub trait AocDay {
    /// The number of the day that the solution is for.
    const DAY: u8;

    /// The implementation of the first part of the puzzle.
    fn part_1(input: &str);

    /// The implementation of the second part of the puzzle.
    fn part_2(input: &str);

    /// Try to get the input file for this day.
    fn get_input() -> String {
        let file_name = format!("day_{:0>2}.txt", Self::DAY);

        fs::read_to_string(format!("../inputs/{file_name}")).unwrap_or_else(|_| {
            panic!("Failed to read input file, make sure to add it at /inputs/{file_name}",)
        })
    }

    fn run() {
        let input = Self::get_input();

        println!("PART 1:");
        Self::part_1(&input);

        println!("\n\nPART 2:");
        Self::part_2(&input);
    }
}
