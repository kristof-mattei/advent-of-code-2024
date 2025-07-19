use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2024::solution!(17_262_627_539_u64, 1986);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect()
}

fn mix(secret: u64, number: u64) -> u64 {
    let new_secret = number ^ secret;

    new_secret
}

fn prune(secret: u64) -> u64 {
    #[expect(
        clippy::decimal_literal_representation,
        reason = "Doesn't make sense here"
    )]
    const REPEAT: u64 = 16_777_216_u64;

    secret % REPEAT
}

fn calculate_new_secret(secret: u64) -> u64 {
    // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    let times_64 = secret * 64;
    let new_secret = mix(secret, times_64);
    let new_secret = prune(new_secret);

    // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    let divided_32 = new_secret / 32;
    let new_secret = mix(new_secret, divided_32);
    let new_secret = prune(new_secret);

    // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    let times_2048 = new_secret * 2048;
    let new_secret = mix(new_secret, times_2048);
    let new_secret = prune(new_secret);

    new_secret
}

fn calculate_secret_sum(input: &str) -> PartSolution {
    let parsed = parse_input(input);

    let mut sum = 0;
    for mut secret in parsed {
        for _i in 0..2000 {
            secret = calculate_new_secret(secret);
        }

        sum += secret;
    }

    sum.into()
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::as_conversions,
    reason = "We select the last digit, so our range is 0 -> 9"
)]
fn banana_price(secret: u64) -> u8 {
    (secret - ((secret / 10) * 10)) as u8
}

fn calculate_banana_price_combos(mut secret: u64) -> HashMap<[i8; 4], u8> {
    let mut rolling_4 = [0; 4];

    let mut last_banana_price = banana_price(secret);

    let mut combinations = HashMap::<[i8; 4], u8>::with_capacity(2000 - 4);

    for i in 0..2000 {
        secret = calculate_new_secret(secret);

        let banana_price = banana_price(secret);

        #[expect(
            clippy::cast_possible_wrap,
            clippy::as_conversions,
            reason = "Banana price is 0 -> 9"
        )]
        let diff = banana_price as i8 - last_banana_price as i8;

        last_banana_price = banana_price;

        if i < 4 {
            rolling_4[i] = diff;
        } else {
            rolling_4.swap(0, 1);
            rolling_4.swap(1, 2);
            rolling_4.swap(2, 3);
            rolling_4[3] = diff;

            if !combinations.contains_key(&rolling_4) {
                combinations.insert(rolling_4, banana_price);
            }
        }
    }

    combinations
}

fn calculate_max_bananas(input: &str) -> PartSolution {
    let parsed = parse_input(input);

    let mut all_combos = HashMap::<[i8; 4], u64>::with_capacity(50_000);

    let mut max_total_banana_price = 0;

    for p in parsed {
        let banana_price_combos = calculate_banana_price_combos(p);

        for (combination, price) in banana_price_combos {
            let price = all_combos
                .entry(combination)
                .and_modify(|p| *p += u64::from(price))
                .or_insert(u64::from(price));

            max_total_banana_price = max_total_banana_price.max(*price);
        }
    }

    max_total_banana_price.into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_secret_sum(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        calculate_max_bananas(input)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution, calculate_new_secret, mix, prune};

        #[test]
        fn outcome() {
            assert_eq!(
                17_262_627_539_u64,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example_1() {
            assert_eq!(
                37_327_623,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn manual() {
            let mut secret = 123;

            let expected = [
                15_887_950, 16_495_136, 527_345, 704_524, 1_553_684, 12_683_156, 11_100_544,
                12_249_484, 7_753_432, 5_908_254,
            ];

            for &e in &expected {
                secret = calculate_new_secret(secret);

                assert_eq!(e, secret);
            }
        }

        #[test]
        fn mix_test() {
            assert_eq!(mix(42, 15), 37);
        }

        #[test]
        fn prune_test() {
            assert_eq!(prune(100_000_000), 16_113_920);
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution, calculate_banana_price_combos};

        #[test]
        fn outcome() {
            assert_eq!(1986, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn banana() {
            calculate_banana_price_combos(123);
        }

        #[test]
        fn example_1() {
            assert_eq!(
                23,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 2))
            );
        }
    }
}
