use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2024::solution!(218_079, 259_755_538_429_618_u64);

fn count_digits(mut number: u64) -> u32 {
    if number > 0 {
        let mut digits = 0;
        while number != 0 {
            number /= 10;

            digits += 1;
        }

        digits
    } else {
        1
    }
}

fn process_stones(input: &str, times: u32) -> PartSolution {
    let parsed = input
        .trim()
        .split(' ')
        .map(|stone| stone.parse::<u64>().expect("Bad input"))
        .collect::<Vec<u64>>();

    let mut memory1 = HashMap::<u64, u64>::new();
    let mut memory2 = HashMap::<u64, u64>::new();

    for p in parsed {
        memory1.entry(p).and_modify(|c| *c += 1).or_insert(1);
    }

    for _blink in 0..times {
        memory2.clear();

        // if digit_memory.len() == 75 {
        //     println!("{:?}", digit_memory);
        //     break;
        // }
        // println!("{:?}", parsed);

        for (value, count) in &memory1 {
            if *value == 0 {
                memory2
                    .entry(1)
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
            } else {
                let digits = count_digits(*value);

                if digits % 2 == 0 {
                    let left = *value / (10u64.pow(digits / 2));
                    let right = *value - (left * (10u64.pow(digits / 2)));

                    memory2
                        .entry(left)
                        .and_modify(|c| *c += *count)
                        .or_insert(*count);
                    memory2
                        .entry(right)
                        .and_modify(|c| *c += *count)
                        .or_insert(*count);
                } else {
                    memory2
                        .entry(*value * 2024)
                        .and_modify(|c| *c += *count)
                        .or_insert(*count);
                }
            }
        }

        std::mem::swap(&mut memory1, &mut memory2);
    }

    memory1.values().copied().sum::<u64>().into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        process_stones(input, 25)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        process_stones(input, 75)
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
            assert_eq!(218_079, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(55312, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                259_755_538_429_618_u64,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                65_601_038_650_482_u64,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
