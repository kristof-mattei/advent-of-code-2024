use advent_of_code_2024::shared::{PartSolution, Parts};
use regex::Regex;

advent_of_code_2024::solution!(183_380_722, 82_733_683);

fn find_muls(input: &str) -> PartSolution {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total = 0;

    for line in lines {
        const MUL: &str = "mul(";
        const COMMA: char = ',';
        const CLOSING_PARENTESES: char = ')';

        for (mul_offset, _) in line.match_indices(MUL) {
            let after_mul = mul_offset + MUL.len();

            // we can split the string at index and be guaranteed it's valid, otherwise we couldn't get anything
            let Some(comma_offset @ 1..=3) = line[after_mul..].find(COMMA) else {
                continue;
            };

            // validate all are numbers
            let Ok(left) = line[after_mul..after_mul + comma_offset].parse::<u32>() else {
                continue;
            };

            let after_comma = after_mul + comma_offset + 1;

            let Some(closing_parenteses_offset @ 1..=3) =
                line[after_comma..].find(CLOSING_PARENTESES)
            else {
                continue;
            };

            // validate all are numbers
            let Ok(right) =
                line[after_comma..after_comma + closing_parenteses_offset].parse::<u32>()
            else {
                continue;
            };

            total += left * right;
        }
    }

    PartSolution::U32(total)
}

fn find_muls_do_dont(input: &str) -> PartSolution {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total = 0;

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for line in lines {
        let mut enabled = true;

        for capture in re.captures_iter(line) {
            match (
                capture.get(0).expect("0 is always non-None").as_str(),
                capture.get(1).map(|v| v.as_str().parse::<u32>().unwrap()),
                capture.get(2).map(|v| v.as_str().parse::<u32>().unwrap()),
            ) {
                (v, Some(l), Some(r)) => {
                    assert!(v.starts_with("mul"));

                    if enabled {
                        total += l * r;
                    }
                },
                ("do()", None, None) => {
                    enabled = true;
                },
                ("don't()", None, None) => {
                    enabled = false;
                },
                _ => {
                    panic!()
                },
            }
        }
    }

    PartSolution::U32(total)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        find_muls(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        find_muls_do_dont(input)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use advent_of_code_2024::shared::Parts;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                183_380_722,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(161, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(82_733_683, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(161, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
