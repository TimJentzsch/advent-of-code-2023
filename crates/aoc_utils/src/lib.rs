use std::fmt::Debug;
use std::fmt::Display;
use std::fs;
use std::hint::black_box;
use std::panic;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

use clap::Parser;
use inquire::Confirm;
use inquire::Text;

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

    /// Try to read the input from the file system.
    fn get_input_from_file() -> Option<String> {
        let file_path = Self::get_file_path();

        if Path::new(&file_path).exists() {
            Some(
                fs::read_to_string(file_path.clone())
                    .unwrap()
                    .trim_end()
                    .to_string(),
            )
        } else {
            None
        }
    }

    /// Try to read the input from the file system.
    fn download_and_save_input(session_cookie: &str) -> String {
        let text = reqwest::blocking::Client::new()
            .get(format!(
                "https://adventofcode.com/2023/day/{}/input",
                Self::DAY
            ))
            .header("Cookie", format!("session={session_cookie}"))
            .send()
            .unwrap()
            .text()
            .unwrap();

        let file_path = Self::get_file_path();
        fs::write(file_path, text.clone()).unwrap();

        text
    }

    fn get_input() -> String {
        if let Some(input) = Self::get_input_from_file() {
            return input;
        }

        let file_path = Self::get_file_path();

        eprintln!();

        if !Confirm::new(&format!(
            "The input file {file_path} does not exist. Do you want to download it?"
        ))
        .prompt()
        .unwrap()
        {
            panic!("No input provided! You can add it at {file_path}.")
        }

        let session_cookie = get_session_cookie();
        Self::download_and_save_input(&session_cookie)
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

fn get_session_cookie() -> String {
    let saved_cookie_path = Path::new("inputs/.session_cookie");

    if saved_cookie_path.exists() {
        fs::read_to_string("inputs/.session_cookie")
            .unwrap()
            .trim()
            .to_string()
    } else {
        let session_cookie = Text::new("What is your Advent of Code session cookie?")
        .with_help_message("You can get your session cookie on https://adventofcode.com using your browsers dev tools.").prompt().unwrap().trim().to_string();

        fs::write(saved_cookie_path, session_cookie.clone()).unwrap();

        session_cookie
    }
}
