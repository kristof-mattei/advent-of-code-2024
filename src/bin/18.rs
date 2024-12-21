use std::collections::BinaryHeap;

use advent_of_code_2024::shared::grids::grid::Grid;
use advent_of_code_2024::shared::grids::GridIter;
use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2024::solution!();

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Corrupted,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Corrupted => '#',
            Cell::Empty => '.',
        };

        write!(f, "{}", c)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coordinates {
    row_index: usize,
    column_index: usize,
}

impl From<(usize, usize)> for Coordinates {
    fn from((row_index, column_index): (usize, usize)) -> Self {
        Coordinates {
            row_index,
            column_index,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply<T>(self, grid: &Grid<T>, row_column_index: Coordinates) -> Option<Coordinates> {
        let row_limit = grid.get_row_length();
        let column_limit = grid.get_column_length();

        let Coordinates {
            row_index,
            column_index,
        } = row_column_index;

        match self {
            Direction::North => (row_index.checked_sub(1)).map(|up| (up, column_index).into()),
            Direction::East => {
                let right = column_index + 1;
                if right < column_limit {
                    Some((row_index, right).into())
                } else {
                    None
                }
            },
            Direction::South => {
                let down = row_index + 1;

                if down < row_limit {
                    Some((down, column_index).into())
                } else {
                    None
                }
            },
            Direction::West => (column_index.checked_sub(1)).map(|left| (row_index, left).into()),
        }
    }
}

fn get_neighbor_directions(
    grid: &Grid<Cell>,
    coordinates: Coordinates,
) -> [Option<Coordinates>; 4] {
    const DIRECTIONS: [Direction; 4] = [
        Direction::South,
        Direction::North,
        Direction::West,
        Direction::East,
    ];

    let mut neighbors: [Option<Coordinates>; 4] = [None; 4];

    for i in 0..DIRECTIONS.len() {
        let direction = DIRECTIONS[i];
        if let Some(new_row_column_index) = direction.apply(grid, coordinates) {
            if grid[new_row_column_index.row_index][new_row_column_index.column_index]
                == Cell::Empty
            {
                neighbors[i] = Some(new_row_column_index);
            }
        }
    }

    neighbors
}

#[derive(Eq, Copy, Clone)]
struct Node {
    coordinates: Coordinates,
    cost: u32,
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coordinates.hash(state);
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(_map: &Grid<Cell>, _c: Coordinates, _n: Coordinates) -> u32 {
    1
}

#[expect(clippy::cast_possible_wrap)]
#[expect(clippy::cast_possible_truncation)]
#[expect(clippy::cast_sign_loss)]
fn heuristic(_map: &Grid<Cell>, current: Coordinates, goal: Coordinates) -> u32 {
    ((current.row_index as isize - goal.row_index as isize).abs()
        + (current.column_index as isize - goal.column_index as isize).abs()) as u32
}

fn reconstruct_path(mut came_from: HashMap<Node, Node>, mut current: Node) -> Vec<Coordinates> {
    let mut total_path = vec![current.coordinates];

    while let Some(next) = came_from.remove(&current) {
        total_path.push(next.coordinates);

        current = next;
    }

    total_path.reverse();
    total_path
}

fn a_star(
    map: &Grid<Cell>,
    start: Coordinates,
    goal: Coordinates,
) -> Option<(HashMap<Node, Node>, Node)> {
    let start = Node {
        coordinates: start,
        cost: heuristic(map, start, goal),
    };

    let mut came_from = HashMap::<Node, Node>::new();

    let mut g_score: HashMap<Node, u32> = HashMap::from([(start, 0)]);

    // we don't keep an f_score as it is incoorporated in the Node
    let mut open_set = BinaryHeap::from([start]);

    while let Some(node) = open_set.pop() {
        if node.coordinates == goal {
            return Some((came_from, node));
        }

        let neighbors = get_neighbor_directions(map, node.coordinates);

        for neighbor in neighbors.iter().filter_map(|&x| x) {
            let tentative_g_score =
                g_score.get(&node).unwrap() + distance(map, node.coordinates, neighbor);

            let f_score = tentative_g_score + heuristic(map, neighbor, goal);

            let neighbor_node = Node {
                coordinates: neighbor,
                cost: f_score,
            };

            if tentative_g_score < *g_score.get(&neighbor_node).unwrap_or(&u32::MAX) {
                g_score.insert(neighbor_node, tentative_g_score);
                came_from.insert(neighbor_node, node);
                // we _could_ check if the open_set already contains the neighbor node, but
                // this is so costly that it's easier to insert with a higher cost and then it'll get either ignored
                // or discarded when the score is considered too high
                open_set.push(neighbor_node);
            }
        }
    }

    None
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();

            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn fall_bytes(input: &str, size: usize, take: usize) -> PartSolution {
    let falling_bytes = parse_input(input);

    let mut grid = Grid::new(vec![vec![Cell::Empty; size]; size]);

    for &(x, y) in falling_bytes.iter().take(take) {
        grid[y][x] = Cell::Corrupted;
    }

    match a_star(&grid, (0, 0).into(), (size - 1, size - 1).into()) {
        Some((came_from, node)) => (reconstruct_path(came_from, node).len() - 1).into(),
        None => PartSolution::None,
    }
}

fn fall_bytes_until(input: &str, size: usize) -> PartSolution {
    let falling_bytes = parse_input(input);

    let mut grid = Grid::new(vec![vec![Cell::Empty; size]; size]);

    for (x, y) in falling_bytes {
        grid[y][x] = Cell::Corrupted;

        if a_star(&grid, (0, 0).into(), (size - 1, size - 1).into()).is_none() {
            return PartSolution::String(format!("{},{}", x, y));
        }
    }

    PartSolution::None
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        fall_bytes(input, 71, 1024)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        fall_bytes_until(input, 71)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{fall_bytes, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(416, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(22, fall_bytes(&read_file("examples", &DAY), 7, 12));
        }
    }

    mod part_2 {

        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::{PartSolution, Parts};

        use crate::{fall_bytes_until, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::String("50,23".into()),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                fall_bytes_until(&read_file("examples", &DAY), 7),
                "6,1".into()
            );
        }
    }
}
