use std::collections::BinaryHeap;
use std::sync::LazyLock;

use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2024::solution!(134_120, 167_389_793_580_400_usize);

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
static KEYPAD_COORDINATES: LazyLock<HashMap<(usize, usize), char>> = LazyLock::new(|| {
    let map = [
        ((0, 0), '7'),
        ((0, 1), '8'),
        ((0, 2), '9'),
        ((1, 0), '4'),
        ((1, 1), '5'),
        ((1, 2), '6'),
        ((2, 0), '1'),
        ((2, 1), '2'),
        ((2, 2), '3'),
        // void is purposfully omitted
        ((3, 1), '0'),
        ((3, 2), 'A'),
    ];

    map.into_iter().collect()
});

#[expect(clippy::type_complexity)]
static KEYPAD_ALL_PATHS: LazyLock<HashMap<(char, char), Vec<Vec<char>>>> =
    LazyLock::new(|| calculate_all_paths(&KEYPAD_COORDINATES));

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
static ARROW_COORDINATES: LazyLock<HashMap<(usize, usize), char>> = LazyLock::new(|| {
    let map = [
        // void is purposfully omitted
        ((0, 1), '^'),
        ((0, 2), 'A'),
        ((1, 0), '<'),
        ((1, 1), 'v'),
        ((1, 2), '>'),
    ];

    map.into_iter().collect()
});

#[expect(clippy::type_complexity)]
static ARROW_ALL_PATHS: LazyLock<HashMap<(char, char), Vec<Vec<char>>>> =
    LazyLock::new(|| calculate_all_paths(&ARROW_COORDINATES));

fn parse_input(input: &str) -> Vec<(usize, Vec<char>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            (
                {
                    let mut chars = line.chars();
                    // yeet last character
                    chars.next_back();

                    chars.collect::<String>().parse::<usize>().unwrap()
                },
                line.chars().collect(),
            )
        })
        .collect::<Vec<_>>()
}

#[derive(Clone)]
struct Journey {
    coordinates: (usize, usize),
    history: Vec<(usize, usize)>,
    cost: u64,
}

impl std::cmp::Eq for Journey {}

impl std::cmp::PartialEq for Journey {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl std::fmt::Debug for Journey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JourneyWithCost")
            .field("coordinates", &self.coordinates)
            .field("history", &self.history)
            .field("cost", &self.cost)
            .finish()
    }
}

impl std::cmp::Ord for Journey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl std::cmp::PartialOrd for Journey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for Journey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coordinates.hash(state);
    }
}

fn get_neighbor_directions((row, column): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    // up
    if let Some(up) = row.checked_sub(1).map(|new_row| (new_row, column)) {
        neighbors.push(up);
    }

    // down
    neighbors.push((row + 1, column));

    // left
    if let Some(left) = column.checked_sub(1).map(|new_column| (row, new_column)) {
        neighbors.push(left);
    }

    // down
    neighbors.push((row, column + 1));

    neighbors
}

fn calculate_all_paths(
    keyboard: &HashMap<(usize, usize), char>,
) -> HashMap<(char, char), Vec<Vec<char>>> {
    let permutations: Vec<_> = keyboard
        .iter()
        .flat_map(|(&from, _)| {
            keyboard
                .iter()
                .map(|(&to, _)| (from, to))
                .collect::<Vec<_>>()
        })
        .collect();

    permutations
        .iter()
        .map(|&(from, to)| {
            (
                (keyboard[&from], keyboard[&to]),
                calculate_cost(keyboard, from, to),
            )
        })
        .collect::<HashMap<(char, char), Vec<Vec<char>>>>()
}

fn calculate_cost<T>(
    keyboard: &HashMap<(usize, usize), T>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<Vec<char>> {
    let mut cheapest_path = None;

    let mut all_paths = vec![];

    let start = Journey {
        coordinates: start,
        history: vec![],
        cost: 0,
    };

    let mut g_score: HashMap<(usize, usize), u64> = HashMap::from([(start.coordinates, 0)]);

    let mut heap = BinaryHeap::from([start]);

    while let Some(mut journey_with_cost) = heap.pop() {
        if cheapest_path.is_some_and(|cheapest| journey_with_cost.cost > cheapest) {
            return all_paths;
        } else if journey_with_cost.coordinates == goal {
            cheapest_path = Some(journey_with_cost.cost);
            journey_with_cost
                .history
                .push(journey_with_cost.coordinates);

            let mut arrows: Vec<_> = journey_with_cost
                .history
                .windows(2)
                .map(|w| find_arrow(w[0], w[1]))
                .collect();

            arrows.push('A');

            all_paths.push(arrows);
        } else if journey_with_cost.cost
            <= g_score
                .get(&journey_with_cost.coordinates)
                .copied()
                .unwrap_or(u64::MAX)
        {
            for neighbor_coordinates in get_neighbor_directions(journey_with_cost.coordinates) {
                if keyboard.contains_key(&neighbor_coordinates) {
                    let tentative_g_score =
                        *g_score.get(&journey_with_cost.coordinates).unwrap() + 1;

                    let f_score = tentative_g_score;

                    let mut neighbor = journey_with_cost.clone();
                    neighbor.history.push(journey_with_cost.coordinates);
                    neighbor.coordinates = neighbor_coordinates;
                    neighbor.cost = f_score;

                    if tentative_g_score
                        < g_score
                            .get(&neighbor.coordinates)
                            .copied()
                            .unwrap_or(u64::MAX)
                    {
                        g_score.insert(neighbor.coordinates, tentative_g_score);
                    }

                    // always push, we want to consider all paths with the same cost
                    heap.push(neighbor);
                }
            }
        }
    }

    all_paths
}

fn find_arrow(from: (usize, usize), to: (usize, usize)) -> char {
    match (from.0.cmp(&to.0), from.1.cmp(&to.1)) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => 'v',
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => '^',
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => '>',
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => '<',
        _ => panic!("Bad diff"),
    }
}

fn find_cost(
    code: Vec<char>,
    depth: usize,
    arrows: &HashMap<(char, char), Vec<Vec<char>>>,
    cache: &mut HashMap<(Vec<char>, usize), usize>,
) -> usize {
    let key = (code, depth);

    if let Some(&cached) = cache.get(&key) {
        cached
    } else {
        // take it apart again
        let (code, depth) = key;

        // always start from A
        let mut code_from_a = vec!['A'];

        for c in &code {
            code_from_a.push(*c);
        }

        let value = code_from_a
            .windows(2)
            .map(|w| {
                let paths = arrows.get(&(w[0], w[1])).unwrap();

                if depth == 0 {
                    paths.iter().map(Vec::len).min().unwrap()
                } else {
                    paths
                        .iter()
                        .map(|path| find_cost(path.clone(), depth - 1, &ARROW_ALL_PATHS, cache))
                        .min()
                        .unwrap()
                }
            })
            .sum();

        cache.insert((code, depth), value);

        value
    }
}

fn press_keys(input: &str, depth: usize) -> PartSolution {
    let codes = parse_input(input);

    let mut total = 0usize;

    let mut cache = HashMap::new();

    for (code, presses) in codes {
        total += find_cost(presses, depth, &KEYPAD_ALL_PATHS, &mut cache) * code;
    }

    total.into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        press_keys(input, 2)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        press_keys(input, 25)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(134_120, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(126_384, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                167_389_793_580_400_usize,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                154_115_708_116_294_usize,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
