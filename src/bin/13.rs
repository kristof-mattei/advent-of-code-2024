use std::sync::LazyLock;

use advent_of_code_2024::shared::{PartSolution, Parts};
use regex::Regex;

advent_of_code_2024::solution!(29522, 101_214_869_433_312_u64);

static PRIZE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap());
static BUTTON_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Button (?:A|B): X\+(\d+), Y\+(\d+)").unwrap());

struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Button A: X+{}, Y+{}", self.button_a.0, self.button_a.1)?;
        writeln!(f, "Button B: X+{}, Y+{}", self.button_b.0, self.button_b.1)?;
        writeln!(f, "Prize: X={}, Y={}", self.prize.0, self.prize.1)?;

        Ok(())
    }
}

fn parse_prize(prize: &str) -> (i64, i64) {
    let prize_captures = PRIZE_REGEX.captures(prize).unwrap();

    let prize_x = prize_captures
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap();

    let prize_y = prize_captures
        .get(2)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap();

    (prize_x, prize_y)
}

fn parse_button(button: &str) -> (i64, i64) {
    let button_captures = BUTTON_REGEX.captures(button).unwrap();

    let button_x = button_captures
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap();
    let button_y = button_captures
        .get(2)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap();

    (button_x, button_y)
}

fn parse_game(input: &[&str]) -> Game {
    let [button_a, button_b, prize] = input.try_into().unwrap();

    Game {
        button_a: parse_button(button_a),
        button_b: parse_button(button_b),
        prize: parse_prize(prize),
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut buffer = Vec::with_capacity(3);
    let mut games = Vec::new();

    for line in input.trim().lines() {
        if line.is_empty() {
            continue;
        }

        buffer.push(line);

        if buffer.len() == 3 {
            games.push(parse_game(&buffer));
            buffer.clear();
        }
    }

    games
}

fn find_solution(game: &Game) -> Option<i64> {
    let a_x = game.button_a.0;
    let a_y = game.button_a.1;

    let b_x = game.button_b.0;
    let b_y = game.button_b.1;

    let prize_x = game.prize.0;
    let prize_y = game.prize.1;

    // cramer
    let a_times = (prize_x * b_y - prize_y * b_x) / (a_x * b_y - a_y * b_x);
    let b_times = (prize_y * a_x - prize_x * a_y) / (a_x * b_y - a_y * b_x);

    if a_times * a_x + b_times * b_x == prize_x && a_times * a_y + b_times * b_y == prize_y {
        Some(3 * a_times + b_times)
    } else {
        None
    }
}

fn calculate_button_presses(input: &str, prize_mod: i64) -> PartSolution {
    let games = parse_input(input);

    let mut total = 0;

    for mut game in games {
        game.prize.0 += prize_mod;
        game.prize.1 += prize_mod;

        if let Some(solution) = find_solution(&game) {
            total += solution;
        }
    }

    total.into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_button_presses(input, 0)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        calculate_button_presses(input, 10_000_000_000_000)
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
            assert_eq!(29522, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(480, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                101_214_869_433_312_u64,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                875_318_608_908_u64,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
