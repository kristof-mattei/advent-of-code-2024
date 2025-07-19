use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!(686, 717);

fn calculate_distances(input: &str) -> PartSolution {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut safe = 0;

    for line in lines {
        let v = line
            .split(' ')
            .map(|v| v.parse::<i32>().expect("Bad input"))
            .collect::<Vec<_>>();

        assert!(v.len() >= 2, "Bad input");

        if is_safe(&v) {
            safe += 1;
        }
    }

    PartSolution::U32(safe)
}

fn is_safe(v: &[i32]) -> bool {
    let first = v[0];
    let last = v[v.len() - 1];

    match first.cmp(&last) {
        cmp @ (std::cmp::Ordering::Less | std::cmp::Ordering::Greater) => {
            for window in v.windows(2) {
                let [l, r] = window.try_into().unwrap();

                if !(l.cmp(&r) == cmp && l.abs_diff(r) <= 3) {
                    return false;
                }
            }
        },
        std::cmp::Ordering::Equal => {
            // they're the same, not safe
            return false;
        },
    }

    true
}

fn calculate_distances_2(input: &str) -> PartSolution {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut safe = 0;

    for line in lines {
        let v = line
            .split(' ')
            .map(|v| v.parse::<i32>().expect("Bad input"))
            .collect::<Vec<_>>();

        assert!(v.len() >= 2, "Bad input");

        if is_safe(&v) || is_safe_with_one_less(&v) {
            safe += 1;
        }
    }

    PartSolution::U32(safe)
}

fn is_safe_with_one_less(v: &[i32]) -> bool {
    for i in 0..v.len() {
        let mut v = v.to_vec();

        v.remove(i);

        if is_safe(&v) {
            return true;
        }
    }

    false
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_distances(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        calculate_distances_2(input)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(686, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(2, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(717, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(4, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
