use std::fmt::Display;

use crate::shared::{Day, PartSolution};

fn count_valid_games(lines: &str) -> u32 {
    let mut total = 0;

    for line in lines.lines() {
        let game_number = is_valid_game_optimized(line);

        if let Some(g_n) = game_number {
            total += g_n;
        }
    }

    total
}

fn is_valid_game_optimized(line: &str) -> Option<u32> {
    let (left, right) = line.split_once(|d| d == ':')?;

    let game_number = left[5..].parse::<u32>().ok()?;

    for subset in right.split(';') {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        for c in subset.trim().split(',') {
            let (left, right) = c.trim().split_once(' ')?;

            let amount = left.trim().parse::<u32>().ok()?;

            match right {
                "blue" => blue = amount,
                "green" => green = amount,
                "red" => red = amount,
                _ => return None,
            }
        }

        if blue > 14 || green > 13 || red > 12 {
            return None;
        }
    }

    Some(game_number)
}

fn sum_of_powers(lines: &str) -> u32 {
    lines.lines().fold(0, |total, line| {
        let game = naive_parse_line(line);
        total + game.get_lowest_amount_of_cubes()
    })
}

struct Game {
    game_number: u32,
    runs: Vec<Cubes>,
}

struct Cubes {
    blue: u32,
    green: u32,
    red: u32,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Game {}: valid?: {}",
            self.game_number,
            self.is_valid()
        ))
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        for cubes in &self.runs {
            if cubes.blue > 14 || cubes.green > 13 || cubes.red > 12 {
                return false;
            }
        }

        true
    }

    fn get_lowest_amount_of_cubes(&self) -> u32 {
        let mut lowest = Cubes {
            blue: u32::MIN,
            green: u32::MIN,
            red: u32::MIN,
        };

        for run in &self.runs {
            lowest.blue = lowest.blue.max(run.blue);
            lowest.green = lowest.green.max(run.green);
            lowest.red = lowest.red.max(run.red);
        }

        lowest.blue * lowest.green * lowest.red
    }
}

enum Cube {
    Blue(u32),
    Green(u32),
    Red(u32),
}

impl TryFrom<(u32, &str)> for Cube {
    type Error = &'static str;

    fn try_from((amount, color): (u32, &str)) -> Result<Self, Self::Error> {
        match color {
            "blue" => Ok(Cube::Blue(amount)),
            "green" => Ok(Cube::Green(amount)),
            "red" => Ok(Cube::Red(amount)),
            _ => Err("Invalid color"),
        }
    }
}

fn naive_parse_line(line: &str) -> Game {
    let (left, right) = line.split_once(|d| d == ':').expect("Invalid game");

    let game_number = left[5..].parse::<u32>().unwrap();

    let mut parsed_runs = vec![];

    for subset in right.split(';') {
        let mut parsed_cubes = Cubes {
            blue: 0,
            green: 0,
            red: 0,
        };

        for c in subset.trim().split(',') {
            let (left, right) = c.trim().split_once(' ').expect("Invalid combo");

            let amount = left.trim().parse::<u32>().unwrap();

            match right {
                "blue" => parsed_cubes.blue = amount,
                "green" => parsed_cubes.green = amount,
                "red" => parsed_cubes.red = amount,
                _ => panic!("Invalid color"),
            }
        }

        parsed_runs.push(parsed_cubes);
    }

    Game {
        game_number,
        runs: parsed_runs,
    }
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let input = include_str!("input.txt");

        let games = count_valid_games(input);

        games.into()
    }

    fn part_2(&self) -> PartSolution {
        let input = include_str!("input.txt");

        sum_of_powers(input).into()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> &'static str {
        include_str!("example.txt")
    }

    mod part_1 {
        use super::super::Solution;
        use super::get_example;
        use crate::day_02::count_valid_games;
        use crate::shared::Day;

        #[test]
        fn outcome() {
            assert_eq!(2449, (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let valid_games_sum = count_valid_games(lines);

            assert_eq!(8, valid_games_sum);
        }
    }

    mod part_2 {
        use super::super::Solution;
        use super::get_example;
        use crate::day_02::sum_of_powers;
        use crate::shared::Day;

        #[test]
        fn outcome() {
            assert_eq!(63_981, (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let sum_of_powers = sum_of_powers(lines);

            assert_eq!(2286, sum_of_powers);
        }
    }
}
