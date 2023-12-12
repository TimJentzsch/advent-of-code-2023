use std::fmt::Debug;
use std::fmt::Display;
use std::fs;
use std::hint::black_box;
use std::panic;
use std::time::Duration;
use std::time::Instant;

use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct Cli {
    /// Run the first part of the puzzle
    #[arg(long)]
    part_1: bool,

    /// Run the second part of the puzzle
    #[arg(long)]
    part_2: bool,

    /// Benchmark the implementation
    #[arg(long)]
    bench: bool,
}

pub trait AocDay<P1: Eq + Debug, P2: Eq + Debug> {
    /// The number of the day that the solution is for.
    const DAY: u8;

    /// The implementation of the first part of the puzzle.
    fn part_1(input: &str) -> P1;

    /// The implementation of the second part of the puzzle.
    fn part_2(input: &str) -> P2;

    /// The name of the file for this day.
    fn get_file_name() -> String {
        format!("day_{:0>2}.txt", Self::DAY)
    }

    /// The path to the input file for this day.
    fn get_file_path() -> String {
        format!("inputs/{}", Self::get_file_name())
    }

    /// Try to get the input file for this day.
    fn get_input() -> String {
        let file_path = Self::get_file_path();

        fs::read_to_string(file_path.clone())
            .unwrap_or_else(|_| {
                panic!("Failed to read input file, make sure to add it at {file_path}",)
            })
            .trim_end()
            .to_string()
    }

    fn run() {
        let start = Instant::now();
        let cli = Cli::parse();

        let (part_1, part_2) = if !cli.part_1 && !cli.part_2 {
            (true, true)
        } else {
            (cli.part_1, cli.part_2)
        };

        eprintln!("DAY {:0>2}\n----", Self::DAY);
        eprint!("INPUT: ");
        let start_input = Instant::now();
        let input = Self::get_input();
        let time_input = start_input.elapsed();
        eprintln!(
            "{} ({} lines) [{:?}]",
            Self::get_file_path(),
            input.lines().count(),
            time_input
        );

        if part_1 {
            eprint!("PART 1: ");
            eprintln!("{}", run_part(Self::part_1, &input, cli.bench));
        }

        if part_2 {
            eprint!("PART 2: ");
            eprintln!("{}", run_part(Self::part_2, &input, cli.bench));
        }

        let time = start.elapsed();
        eprintln!("----\nFinished in {:?}", time);
    }
}

struct RunResult<T: Debug> {
    res: T,
    times: Vec<Duration>,
}

impl<T: Debug> Display for RunResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.times.len() == 1 {
            write!(f, "{:?} [{:?}]", self.res, self.times[0])
        } else {
            let avg = self.times.iter().sum::<Duration>() / self.times.len() as u32;
            let min = self.times.iter().min().unwrap();
            let max = self.times.iter().max().unwrap();
            let deviation = (avg - *min).max(*max - avg);

            write!(
                f,
                "{:?} [{:?} ± {:?}, {} samples]",
                self.res,
                avg,
                deviation,
                self.times.len()
            )
        }
    }
}

fn run_part<T: Eq + Debug>(part: impl Fn(&str) -> T, input: &str, bench: bool) -> RunResult<T> {
    let mut times = Vec::new();

    let start = Instant::now();
    let res = part(black_box(input));
    times.push(start.elapsed());

    if bench {
        while start.elapsed() < Duration::from_secs(5) {
            let iter_start = Instant::now();
            let iter_res = part(black_box(input));
            times.push(iter_start.elapsed());
            assert_eq!(res, iter_res);
        }
    }

    RunResult { res, times }
}
