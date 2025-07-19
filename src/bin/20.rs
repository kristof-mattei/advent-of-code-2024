use advent_of_code_2024::shared::grids::GridIter as _;
use advent_of_code_2024::shared::grids::grid::Grid;
use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!(1511, 1_020_507_usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Start => 'S',
            Cell::End => 'E',
        };

        write!(f, "{}", c)
    }
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            _ => Err("Invalid input"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Coordinates> {
    let parsed = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.try_into().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let grid = Grid::new(parsed);

    let start = grid
        .row_column_index_value_iter()
        .find(|c| matches!(*c, Cell::Start))
        .unwrap()
        .into();

    let end = grid
        .row_column_index_value_iter()
        .find(|c| matches!(*c, Cell::End))
        .unwrap()
        .into();

    let mut coordinates = vec![start];

    let mut current = start;
    let mut last = current;

    while current != end {
        let new = get_neighbors(&grid, current)
            .iter()
            .filter_map(|&coordinates| coordinates)
            .filter(|coordinates| {
                !matches!(
                    grid[coordinates.row_index][coordinates.column_index],
                    Cell::Wall
                )
            })
            .find(|&coordinates| coordinates != last)
            .unwrap();

        coordinates.push(new);

        last = current;
        current = new;
    }

    coordinates
}

fn distance(c1: Coordinates, c2: Coordinates) -> usize {
    c1.row_index.abs_diff(c2.row_index) + c1.column_index.abs_diff(c2.column_index)
}

fn find_cheats(coordinates: &[Coordinates], goal: usize, max_cheat_time: usize) -> usize {
    let mut cheat = 0;

    for start in 0..coordinates.len() {
        for end in start + goal..coordinates.len() {
            let distance = distance(coordinates[start], coordinates[end]);

            if distance <= max_cheat_time && distance <= end - start - goal {
                cheat += 1;
            }
        }
    }

    cheat
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

fn get_neighbors(grid: &Grid<Cell>, coordinates: Coordinates) -> [Option<Coordinates>; 4] {
    const DIRECTIONS: [Direction; 4] = [
        Direction::South,
        Direction::North,
        Direction::West,
        Direction::East,
    ];

    let mut neighbors: [Option<Coordinates>; 4] = [None; 4];

    for d in 0..DIRECTIONS.len() {
        if let Some(new_row_column_index) = DIRECTIONS[d].apply(grid, coordinates) {
            neighbors[d] = Some(new_row_column_index);
        }
    }

    neighbors
}

fn count_cheats(input: &str, min_win: usize, cheating_time: usize) -> PartSolution {
    let coordinates = parse_input(input);

    let cheats = find_cheats(&coordinates, min_win, cheating_time);

    cheats.into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        count_cheats(input, 100, 2)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        count_cheats(input, 100, 20)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution, count_cheats};

        #[test]
        fn outcome() {
            assert_eq!(1511, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(1, count_cheats(&read_file("examples", &DAY), 64, 2));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution, count_cheats};

        #[test]
        fn outcome() {
            assert_eq!(
                1_020_507_usize,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(3, count_cheats(&read_file("examples", &DAY), 75, 20));
        }
    }
}
