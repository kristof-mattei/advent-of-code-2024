use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::shared::{Day, PartSolution};

struct Game {
    my_numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
    game_number: usize,
}

impl Hash for Game {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.game_number.hash(state);
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.game_number == other.game_number
    }
}

impl Eq for Game {}

fn parse_lines(lines: &[&str]) -> Vec<Game> {
    let mut games = Vec::new();

    for line in lines {
        let (game, all_numbers) = line.split_once(':').expect("Invalid game");

        let (winning_numbers, my_numbers) = all_numbers.split_once('|').expect("Invalid numbers");

        games.push(Game {
            game_number: parse_game_number(game),
            my_numbers: split_string_into_numbers(my_numbers),
            winning_numbers: split_string_into_numbers(winning_numbers),
        });
    }

    games
}

fn get_scores(games: &[Game]) -> usize {
    let mut scores = 0;

    for game in games {
        let score = game.get_score();

        scores += score;
    }

    scores
}

impl Game {
    fn get_score(&self) -> usize {
        let intersection: u32 = self
            .get_intersection_count()
            .try_into()
            .expect("Intersection too large");

        intersection.checked_sub(1).map_or(0, |i| 2usize.pow(i))
    }

    fn get_intersection_count(&self) -> usize {
        self.winning_numbers.intersection(&self.my_numbers).count()
    }
}

fn parse_game_number(game: &str) -> usize {
    let (_, game_number) = game.split_once(' ').expect("Invalid game");

    game_number
        .trim()
        .parse::<usize>()
        .expect("Invalid game number")
}

fn count_played_cards(games: &mut [Game]) -> u32 {
    let mut cache: HashMap<&Game, u32> = HashMap::new();

    count_played_cards_r(Some(games), true, &mut cache)
}

fn count_played_cards_r<'games>(
    games: Option<&'games [Game]>,
    base_run: bool,
    cache: &mut HashMap<&'games Game, u32>,
) -> u32 {
    let Some(games) = games else {
        return 0;
    };

    let Some(game) = games.get(0) else {
        return 0;
    };

    if let Some(c) = cache.get(&game) {
        return *c;
    }

    let mut total_cards = 1;

    for offset in 0..game.get_intersection_count() {
        total_cards += count_played_cards_r(games.get(1 + offset..), false, cache);
    }

    cache.insert(game, total_cards);

    // there is one base run where we always advance
    if base_run {
        for i in 1..games.len() {
            total_cards += count_played_cards_r(games.get(i..), false, cache);
        }
    }

    total_cards
}

fn split_string_into_numbers(numbers: &str) -> HashSet<u32> {
    numbers
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<u32>().expect("Invalid number"))
        .collect()
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let games = parse_lines(&lines);

        get_scores(&games).into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let mut parsed = parse_lines(&lines);

        let total_cards = count_played_cards(&mut parsed);

        total_cards.into()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use super::super::{parse_lines, Solution};
        use super::get_example;
        use crate::{day_04::get_scores, shared::Day};

        #[test]
        fn outcome() {
            assert_eq!(18_619, (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            assert_eq!(13, get_scores(&parsed));
        }
    }

    mod part_2 {
        use super::super::{parse_lines, Solution};
        use super::get_example;
        use crate::{day_04::count_played_cards, shared::Day};

        #[test]
        fn outcome() {
            assert_eq!(8_063_216, (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let mut parsed = parse_lines(&lines);

            let total_cards = count_played_cards(&mut parsed);

            assert_eq!(30, total_cards);
        }
    }
}
