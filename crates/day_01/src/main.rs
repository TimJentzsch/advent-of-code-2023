use aoc_utils::AocDay;

fn main() {
    Day01::run();
}

struct Day01;

impl AocDay<u32, ()> for Day01 {
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

    fn part_2(_input: &str) {
        todo!()
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
}
