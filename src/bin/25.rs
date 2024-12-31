use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!(3133usize);

fn parse_lock(lock: &[&str]) -> Vec<usize> {
    lock.iter().skip(1).fold(vec![0usize; 5], |mut acc, curr| {
        for (i, c) in curr.chars().enumerate() {
            if c == '#' {
                acc[i] += 1;
            }
        }

        acc
    })
}

fn parse_key(key: &[&str]) -> Vec<usize> {
    key.iter()
        .rev()
        .skip(1)
        .fold(vec![0usize; 5], |mut acc, curr| {
            for (i, c) in curr.chars().enumerate() {
                if c == '#' {
                    acc[i] += 1;
                }
            }

            acc
        })
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for lock_or_key in input.split("\n\n") {
        let lock_or_key = lock_or_key.lines().map(str::trim).collect::<Vec<_>>();

        if lock_or_key
            .first()
            .map(|line| line.chars().all(|c| c == '#'))
            .unwrap()
        {
            // it's a lock
            locks.push(parse_lock(&lock_or_key));
        } else {
            // it's a key
            keys.push(parse_key(&lock_or_key));
        }
    }

    (locks, keys)
}

fn key_fits_lock(lock: &[usize], key: &[usize]) -> bool {
    for (l, k) in lock.iter().zip(key.iter()) {
        if l + k > 5 {
            return false;
        }
    }

    true
}

fn get_unique_lock_key_pairs(input: &str) -> PartSolution {
    let (locks, keys) = parse_input(input);

    let mut unique_combinations = 0;

    for lock in &locks {
        for key in &keys {
            if key_fits_lock(lock, key) {
                unique_combinations += 1;
            }
        }
    }

    PartSolution::USize(unique_combinations)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        get_unique_lock_key_pairs(input)
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        None.into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::{PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(3133),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(3),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::{PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
