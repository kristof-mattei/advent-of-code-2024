use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2024::solution!(1_579_939, 20_351_745);

fn calculate_distances(input: &str) -> PartSolution {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut left = Vec::with_capacity(lines.len());
    let mut right = Vec::with_capacity(lines.len());

    for line in lines {
        let (l, r) = line
            .split_once(' ')
            .map(|(l, r)| (l.trim().parse::<u32>(), r.trim().parse::<u32>()))
            .expect("Bad input");

        left.push(l.expect("Bad input"));
        right.push(r.expect("Bad input"));
    }

    // we don't need to sort smallest to largest for now
    left.sort_unstable();
    right.sort_unstable();

    let mut total_diff = 0;

    for (l, r) in left.into_iter().zip(right) {
        total_diff += l.abs_diff(r);
    }

    PartSolution::U32(total_diff)
}

fn calculate_similarity(input: &str) -> PartSolution {
    let mut map_l: HashMap<u32, u32> = HashMap::new();
    let mut map_r: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let (l, r) = line
            .split_once(' ')
            .map(|(l, r)| (l.trim().parse::<u32>(), r.trim().parse::<u32>()))
            .expect("Bad input");

        let (l, r) = (l.expect("Bad input"), r.expect("Bad input"));

        map_l.entry(l).and_modify(|v| *v += 1).or_insert(1);
        map_r.entry(r).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut similiarity = 0;

    for (number, count) in map_l {
        similiarity += number * count * map_r.get(&number).copied().unwrap_or(0);
    }

    PartSolution::U32(similiarity)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_distances(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        calculate_similarity(input)
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
            assert_eq!(1_579_939, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(11, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn example() {
            assert_eq!(31, Solution {}.part_2(&read_file("examples", &DAY)));
        }

        #[test]
        fn outcome() {
            assert_eq!(20_351_745, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }
    }
}
