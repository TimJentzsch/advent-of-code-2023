use aoc_utils::AocDay;

fn main() {
    Day09::run();
}

type Num = i32;

struct Day09;

impl AocDay<Num, Num> for Day09 {
    const DAY: u8 = 9;

    fn part_1(input: &str) -> Num {
        input
            .lines()
            .map(|line| {
                let sequence: Vec<_> = line
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<Num>().expect("Invalid number"))
                    .collect();

                extrapolate_value_forwards(sequence)
            })
            .sum()
    }

    fn part_2(input: &str) -> Num {
        input
            .lines()
            .map(|line| {
                let sequence: Vec<_> = line
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<Num>().expect("Invalid number"))
                    .collect();

                extrapolate_value_backwards(sequence)
            })
            .sum()
    }
}

fn extrapolate_value_forwards(sequence: Vec<Num>) -> Num {
    if sequence.iter().all(|num| *num == 0) {
        0
    } else {
        let next_sequence: Vec<_> = sequence
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        let next_diff = extrapolate_value_forwards(next_sequence);
        sequence.last().unwrap() + next_diff
    }
}

fn extrapolate_value_backwards(sequence: Vec<Num>) -> Num {
    if sequence.iter().all(|num| *num == 0) {
        0
    } else {
        let next_sequence: Vec<_> = sequence
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        let next_diff = extrapolate_value_backwards(next_sequence);
        sequence.first().unwrap() - next_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        assert_eq!(Day09::part_1(INPUT), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day09::part_2(INPUT), 2);
    }
}
