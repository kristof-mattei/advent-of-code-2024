use crate::shared::{Day, PartSolution};

fn first_number_with_index(line: &str) -> Option<(usize, u32)> {
    line.chars()
        .enumerate()
        .find_map(|(index, c)| c.to_digit(10).map(|d| (index, d)))
}

fn last_number_with_index(line: &str) -> Option<(usize, u32)> {
    line.chars()
        .rev()
        .enumerate()
        .find_map(|(index, c)| c.to_digit(10).map(|d| (line.len() - index - 1, d)))
}

fn calculate_total_calibration_value_part_1(lines: &[&str]) -> u32 {
    let mut total = 0;
    for line in lines {
        let (_, first_number) = first_number_with_index(line).expect("No number found");
        let (_, last_number) = last_number_with_index(line).expect("No number found");

        let total_this_line = first_number * 10 + last_number;

        total += total_this_line;
    }

    total
}

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn calculate_total_calibration_value_part_2(lines: &[&str]) -> u32 {
    let mut total = 0;
    for line in lines {
        let first_number = first_number_with_index(line);
        let last_number = last_number_with_index(line);

        let first_word = first_word_with_index(line);
        let last_word = last_word_with_index(line);

        let first = match (first_number, first_word) {
            (None, None) => panic!("No number found"),
            (None, Some((_, first))) | (Some((_, first)), None) => first,
            (Some((first_number_index, first_number)), Some((first_word_index, first_word))) => {
                if first_number_index < first_word_index {
                    first_number
                } else {
                    first_word
                }
            },
        };

        total += first * 10;

        let last = match (last_number, last_word) {
            (None, None) => panic!("No number found"),
            (None, Some((_, last))) | (Some((_, last)), None) => last,
            (Some((last_number_index, last_number)), Some((last_word_index, last_word))) => {
                if last_number_index > last_word_index {
                    last_number
                } else {
                    last_word
                }
            },
        };

        total += last;
    }
    total
}

fn first_word_with_index(line: &str) -> Option<(usize, u32)> {
    for i in 0..line.len() {
        for (word_index, word) in NUMBER_WORDS.iter().enumerate() {
            if line[i..].starts_with(word) {
                return Some((i, word_index as u32 + 1));
            }
        }
    }

    None
}

fn last_word_with_index(line: &str) -> Option<(usize, u32)> {
    for i in (0..line.len()).rev() {
        for (word_index, word) in NUMBER_WORDS.iter().enumerate() {
            if line[i..].starts_with(word) {
                return Some((i, word_index as u32 + 1));
            }
        }
    }

    None
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        calculate_total_calibration_value_part_1(&lines).into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        calculate_total_calibration_value_part_2(&lines).into()
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use crate::day_01::{calculate_total_calibration_value_part_1, Solution};
        use crate::shared::{Day, PartSolution};

        fn get_example() -> Vec<&'static str> {
            include_str!("example_part_1.txt")
                .lines()
                .map(Into::into)
                .collect()
        }

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(54159), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let max = calculate_total_calibration_value_part_1(&lines);

            assert_eq!(max, 142);
        }
    }

    mod part_2 {
        use crate::day_01::{calculate_total_calibration_value_part_2, Solution};
        use crate::shared::{Day, PartSolution};

        fn get_example() -> Vec<&'static str> {
            include_str!("example_part_2.txt")
                .lines()
                .map(Into::into)
                .collect()
        }

        #[test]
        fn example() {
            let lines = get_example();

            let max = calculate_total_calibration_value_part_2(&lines);

            assert_eq!(max, 281);
        }

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(53866), (Solution {}).part_2());
        }

        #[test]
        fn first_word_with_index() {
            let line = "one two three four five six seven eight nine";
            let (index, number) = super::super::first_word_with_index(line).unwrap();

            assert_eq!(index, 0);
            assert_eq!(number, 1);
        }

        #[test]
        fn last_word_with_index() {
            let line = "one two three four five six seven eight nine";
            let (index, number) = super::super::last_word_with_index(line).unwrap();

            assert_eq!(index, 40);
            assert_eq!(number, 9);
        }
    }
}
