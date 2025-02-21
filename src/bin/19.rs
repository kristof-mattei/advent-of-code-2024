use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2024::solution!(272, 1_041_529_704_688_380_u64);

fn count_possible_towels(input: &str) -> PartSolution {
    let (designs, wanted_towels) = parse_input(input);

    let mut cache = HashMap::new();
    let mut can_build: u32 = 0;

    for towel in wanted_towels {
        if count_towel_permutations(&designs, &towel, &mut cache) >= 1 {
            can_build += 1;
        }
    }

    can_build.into()
}

fn how_many_towel_combinations(input: &str) -> PartSolution {
    let (designs, wanted_towels) = parse_input(input);

    let mut cache = HashMap::new();
    let mut can_build = 0;

    for towel in wanted_towels {
        can_build += count_towel_permutations(&designs, &towel, &mut cache);
    }

    can_build.into()
}

fn count_towel_permutations(
    designs: &Vec<Vec<char>>,
    towel: &[char],
    cache: &mut HashMap<Vec<char>, u64>,
) -> u64 {
    if towel.is_empty() {
        return 1;
    }

    if let Some(&c) = cache.get(towel) {
        return c;
    }

    let mut sub_towels = 0;
    for p in designs.iter().filter(|d| towel.starts_with(d)) {
        sub_towels += count_towel_permutations(designs, &towel[p.len()..], cache);
    }

    cache.insert(towel.to_vec(), sub_towels);

    sub_towels
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut lines = input.trim().lines();

    let designs = lines
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let _skip = lines.next().unwrap();

    let mut towels = vec![];

    for towel in lines {
        towels.push(towel.chars().collect::<Vec<_>>());
    }

    (designs, towels)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        count_possible_towels(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        how_many_towel_combinations(input)
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
            assert_eq!(272, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(6, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                1_041_529_704_688_380_u64,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(16, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
