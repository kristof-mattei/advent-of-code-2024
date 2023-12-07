use std::convert::Into;

use advent_of_code_2023::shared::{Day, PartSolution};

advent_of_code_2023::solution!(6, 345_015, 42_588_603);

fn parse_lines(input: &str) -> Vec<(usize, usize)> {
    let mut time_and_distances = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();

    let (_, times) = lines[0].split_once(':').expect("Bad input");
    let (_, distances) = lines[1].split_once(':').expect("Bad input");

    for (time, distance) in times
        .split_ascii_whitespace()
        .zip(distances.split_ascii_whitespace())
    {
        time_and_distances.push((
            time.parse().expect("Bad time"),
            distance.parse().expect("Bad distance"),
        ));
    }

    time_and_distances
}

fn parse_lines_with_bad_kerning(input: &str) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<&str>>();

    let (_, times) = lines[0].split_once(':').expect("Bad input");
    let (_, distances) = lines[1].split_once(':').expect("Bad input");

    (
        times.replace(' ', "").parse().expect("Bad time"),
        distances.replace(' ', "").parse().expect("Bad distance"),
    )
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let time_and_distances = parse_lines(input);

        let mut possibilities = 1;

        for (time, distance) in time_and_distances {
            possibilities *= calculate_possibilities(time, distance);
        }

        possibilities.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (time, distance) = parse_lines_with_bad_kerning(input);

        calculate_possibilities(time, distance).into()
    }
}

fn calculate_possibilities(time: usize, distance: usize) -> usize {
    let mut possibilities = 0;

    for i in 0..time {
        let speed = i;
        let time_remaining = time - i;

        if speed * time_remaining > distance {
            possibilities += 1;
        }
    }

    possibilities
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::{solution::read_file, Day};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(345_015, (Solution {}).part_1(&read_file("inputs", DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(288, (Solution {}).part_1(&read_file("examples", DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::{solution::read_file, Day};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(42_588_603, (Solution {}).part_2(&read_file("inputs", DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(71503, (Solution {}).part_2(&read_file("examples", DAY)));
        }
    }
}
