use std::collections::BinaryHeap;
use std::hash::Hash;

use advent_of_code_2024::shared::grids::GridIter as _;
use advent_of_code_2024::shared::grids::grid::Grid;
use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::{HashMap, HashSet};

advent_of_code_2024::solution!(109_496, 551);

enum Cell {
    Start,
    End,
    Wall,
    Empty,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Wall => '#',
            Cell::Empty => '.',
        };

        write!(f, "{}", c)
    }
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Empty),
            _ => Err("Invalid input"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell::try_from(c).expect("Bad input"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
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
    fn apply<T>(self, grid: &Grid<T>, row_column_index: &Coordinates) -> Option<Coordinates> {
        let row_limit = grid.len();
        let column_limit = grid[0].len();

        let &Coordinates {
            ref row_index,
            ref column_index,
        } = row_column_index;

        match self {
            Direction::North => (row_index.checked_sub(1)).map(|up| (up, *column_index).into()),
            Direction::East => {
                let right = column_index + 1;
                if right < column_limit {
                    Some((*row_index, right).into())
                } else {
                    None
                }
            },
            Direction::South => {
                let down = row_index + 1;

                if down < row_limit {
                    Some((down, *column_index).into())
                } else {
                    None
                }
            },
            Direction::West => (column_index.checked_sub(1)).map(|left| (*row_index, left).into()),
        }
    }

    fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}

fn get_neighbor_directions(grid: &Grid<Cell>, current: &At) -> Vec<(Coordinates, Direction)> {
    let mut neighbors = Vec::with_capacity(3);

    if let Some(keep_going) = current.direction.apply(grid, &current.coordinates) {
        if matches!(
            grid[keep_going.row_index][keep_going.column_index],
            Cell::Empty | Cell::End
        ) {
            neighbors.push((keep_going, current.direction));
        }
    }

    let left_direction = current.direction.turn_left();

    if let Some(left) = left_direction.apply(grid, &current.coordinates) {
        if matches!(
            grid[left.row_index][left.column_index],
            Cell::Empty | Cell::End
        ) {
            neighbors.push((left, left_direction));
        }
    }

    let right_direction = current.direction.turn_right();

    if let Some(right) = right_direction.apply(grid, &current.coordinates) {
        if matches!(
            grid[right.row_index][right.column_index],
            Cell::Empty | Cell::End
        ) {
            neighbors.push((right, right_direction));
        }
    }

    neighbors
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct At {
    coordinates: Coordinates,
    direction: Direction,
}

#[derive(Eq, PartialEq, Clone)]
struct Journey {
    current: At,
    history: Vec<Coordinates>,
}

#[derive(Eq, PartialEq)]
struct JourneyWithCost {
    journey: Journey,
    cost: u64,
}

impl std::cmp::Ord for JourneyWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl std::cmp::PartialOrd for JourneyWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_cost(input: &str) -> PartSolution {
    let parsed = parse_input(input);

    let grid = Grid::new(parsed);

    let start: Coordinates = grid
        .row_column_index_value_iter()
        .find(|c| matches!(*c, Cell::Start))
        .unwrap()
        .into();

    let end: Coordinates = grid
        .row_column_index_value_iter()
        .find(|c| matches!(*c, Cell::End))
        .unwrap()
        .into();

    let mut heap = BinaryHeap::new();

    heap.push(JourneyWithCost {
        journey: Journey {
            current: At {
                coordinates: start,
                direction: Direction::East,
            },
            history: vec![],
        },
        cost: 0,
    });

    let mut seen = HashMap::<At, u64>::new();

    while let Some(JourneyWithCost { journey, cost }) = heap.pop() {
        if journey.current.coordinates == end {
            return cost.into();
        } else if seen.get(&journey.current).copied().unwrap_or(u64::MAX) > cost {
            seen.insert(journey.current, cost);

            for (neighbor_coordinates, neighbor_direction) in
                get_neighbor_directions(&grid, &journey.current)
            {
                let mut clone = journey.clone();
                clone.history.push(clone.current.coordinates);
                clone.current.coordinates = neighbor_coordinates;
                clone.current.direction = neighbor_direction;

                if neighbor_direction == journey.current.direction {
                    heap.push(JourneyWithCost {
                        journey: clone,
                        cost: cost + 1,
                    });
                } else {
                    heap.push(JourneyWithCost {
                        journey: clone,
                        cost: cost + 1001,
                    });
                }
            }
        } else {
            // ...
        }
    }

    panic!("No solution found")
}

fn count_all_points_on_lowest_cost_paths(input: &str) -> PartSolution {
    let parsed = parse_input(input);

    let grid = Grid::new(parsed);

    let start: Coordinates = grid
        .row_column_index_value_iter()
        .find(|c| matches!(*c, Cell::Start))
        .unwrap()
        .into();

    let end: Coordinates = grid
        .row_column_index_value_iter()
        .find(|c| matches!(*c, Cell::End))
        .unwrap()
        .into();

    let mut heap = BinaryHeap::new();

    heap.push(JourneyWithCost {
        journey: Journey {
            current: At {
                coordinates: start,
                direction: Direction::East,
            },
            history: vec![],
        },
        cost: 0,
    });

    let mut seen = HashMap::<At, u64>::new();

    let mut cheapest_path = None;

    let mut all_visited_spots = HashSet::new();

    while let Some(JourneyWithCost { journey, cost }) = heap.pop() {
        if cheapest_path.is_some_and(|cost_at_goal| cost > cost_at_goal) {
            return all_visited_spots.len().into();
        } else if journey.current.coordinates == end {
            cheapest_path = Some(cost);

            all_visited_spots.insert(journey.current.coordinates);

            for coordinates in journey.history {
                all_visited_spots.insert(coordinates);
            }
        } else if seen.get(&journey.current).copied().unwrap_or(u64::MAX) >= cost {
            seen.insert(journey.current, cost);

            for (neighbor_coordinates, neighbor_direction) in
                get_neighbor_directions(&grid, &journey.current)
            {
                let mut clone = journey.clone();
                clone.history.push(journey.current.coordinates);
                clone.current.coordinates = neighbor_coordinates;
                clone.current.direction = neighbor_direction;

                if neighbor_direction == journey.current.direction {
                    heap.push(JourneyWithCost {
                        journey: clone,
                        cost: cost + 1,
                    });
                } else {
                    heap.push(JourneyWithCost {
                        journey: clone,
                        cost: cost + 1001,
                    });
                }
            }
        } else {
            // ...
        }
    }

    all_visited_spots.len().into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_cost(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        count_all_points_on_lowest_cost_paths(input)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(109_496, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                7036,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                11048,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(551, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                45,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                64,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 2))
            );
        }
    }
}
