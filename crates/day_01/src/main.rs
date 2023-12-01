use aoc_utils::AocDay;

fn main() {
    Day01::run();
}

struct Day01;

impl AocDay<u32, u32> for Day01 {
    const DAY: u8 = 1;

    fn part_1(input: &str) -> u32 {
        input
            .trim()
            .lines()
            .map(|line| {
                let mut digits = line
                    .chars()
                    .filter_map(|char| char.to_string().parse::<u32>().ok());
                digits.clone().nth(0).unwrap() * 10 + digits.nth_back(0).unwrap()
            })
            .sum::<u32>()
    }

    fn part_2(input: &str) -> u32 {
        let digit_map = [
            // Numerical
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            // Words
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        input
            .trim()
            .lines()
            .map(|line| {
                // First occurring digit in the line
                let (first, _) = digit_map
                    .iter()
                    .filter_map(|(token, num)| line.find(token).map(|pos| (num, pos)))
                    .min_by_key(|(_, pos)| *pos)
                    .unwrap();

                // Last occurring digit in the line
                let (second, _) = digit_map
                    .iter()
                    .filter_map(|(token, num)| {
                        // Result is the index of the first character, so we should add the token length to be sure
                        line.rfind(token).map(|pos| (num, pos + token.len()))
                    })
                    .max_by_key(|(_, pos)| *pos)
                    .unwrap();

                first * 10 + second
            })
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use aoc_utils::AocDay;

    use crate::Day01;

    #[test]
    fn part_1() {
        assert_eq!(
            Day01::part_1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(
            Day01::part_2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        )
    }
}
