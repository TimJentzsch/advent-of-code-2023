use std::fmt::Debug;
use std::fs;
use std::panic;
use std::time::Instant;

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

        fs::read_to_string(format!("inputs/{file_name}"))
            .unwrap_or_else(|_| {
                panic!("Failed to read input file, make sure to add it at /inputs/{file_name}",)
            })
            .trim_end()
            .to_string()
    }

    fn run() {
        let start = Instant::now();

        eprintln!("DAY {:0>2}\n----", Self::DAY);
        eprint!("INPUT: ");
        let start_input = Instant::now();
        let input = Self::get_input();
        let time_input = start_input.elapsed();
        eprintln!(
            "inputs/day_{:0>2}.txt ({} lines) [{:?}]",
            Self::DAY,
            input.lines().count(),
            time_input
        );

        eprint!("PART 1: ");
        let start_part_1 = Instant::now();
        let res_part_1 = Self::part_1(&input);
        let time_part_1 = start_part_1.elapsed();
        eprintln!("{:?} [{:?}]", res_part_1, time_part_1);

        eprint!("PART 2: ");
        let start_part_2 = Instant::now();
        let res_part_2 = Self::part_2(&input);
        let time_part_2 = start_part_2.elapsed();
        eprintln!("{:?} [{:?}]", res_part_2, time_part_2);

        let time = start.elapsed();
        eprintln!("----\nFinished in {:?}", time);
    }
}
