use std::collections::BTreeSet;

use advent_of_code_2024::shared::grids::grid::Grid;
use advent_of_code_2024::shared::grids::GridIter;
use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!(4973, 1482);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Open,
    Obstruction,
    Guard(Direction),
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "."),
            Self::Obstruction => write!(f, "#"),
            Self::Guard(d) => {
                write!(f, "{:?}", d)
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };

        write!(f, "{}", debug)
    }
}

impl Direction {
    fn rotate_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn parse_input(input: &str) -> Grid<Cell> {
    let mut parsed = Vec::new();

    for line in input.lines() {
        parsed.push(
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Open,
                    '#' => Cell::Obstruction,
                    '^' => Cell::Guard(Direction::Up),
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<Cell>>(),
        );
    }

    Grid::new(parsed)
}

fn count_guard_positions(input: &str) -> PartSolution {
    let mut grid = parse_input(input);
    let mut traveled = BTreeSet::<(usize, usize)>::new();
    // starting direction is always up
    let mut direction = Direction::Up;
    let (mut guard_row_index, mut guard_column_index) =
        grid.find(&Cell::Guard(direction)).expect("Bad input");

    // starting position needs to be recorded too
    traveled.insert((guard_row_index, guard_column_index));

    while let Some((r, c, d)) = travel(&mut grid, guard_row_index, guard_column_index, direction) {
        guard_row_index = r;
        guard_column_index = c;
        direction = d;

        traveled.insert((guard_row_index, guard_column_index));
    }

    traveled.len().into()
}

fn travel(
    grid: &mut Grid<Cell>,
    guard_row_index: usize,
    guard_column_index: usize,
    direction: Direction,
) -> Option<(usize, usize, Direction)> {
    // guard is at (guard_row_index, guard_column_index) and facing direction

    // unset the current space, we're moving
    grid[guard_row_index][guard_column_index] = Cell::Open;

    match direction {
        Direction::Up => {
            // calculate up. If out of bounds, return none
            // if empty, move, update grid and return
            // if obstacle, rotate, invoke self

            if let Some(new_guard_row_index) = guard_row_index.checked_add_signed(-1) {
                if matches!(
                    grid[new_guard_row_index][guard_column_index],
                    Cell::Obstruction
                ) {
                    let new_direction = direction.rotate_clockwise();

                    // can't move in that direction, travel again, but pointing in new direction
                    grid[guard_row_index][guard_column_index] = Cell::Guard(new_direction);
                    return travel(grid, guard_row_index, guard_column_index, new_direction);
                }

                // shortcut, cell is empty, return new position, maintain direction
                grid[new_guard_row_index][guard_column_index] = Cell::Guard(direction);
                return Some((new_guard_row_index, guard_column_index, direction));
            }

            // OOB
            None
        },
        Direction::Right => {
            // calculate right. If out of bounds, return none
            // if empty, move, update grid and return
            // if obstacle, rotate, invoke self

            if let Some(new_guard_column_index) = (guard_column_index + 1
                < grid.get_column_length())
            .then_some(guard_column_index + 1)
            {
                if matches!(
                    grid[guard_row_index][new_guard_column_index],
                    Cell::Obstruction
                ) {
                    let new_direction = direction.rotate_clockwise();

                    // can't move in that direction, travel again, but pointing in new direction
                    grid[guard_row_index][guard_column_index] = Cell::Guard(new_direction);
                    return travel(grid, guard_row_index, guard_column_index, new_direction);
                }

                // shortcut, cell is empty, return new position, maintain direction
                grid[guard_row_index][new_guard_column_index] = Cell::Guard(direction);
                return Some((guard_row_index, new_guard_column_index, direction));
            }

            // OOB
            None
        },
        Direction::Down => {
            // calculate down. If out of bounds, return none
            // if empty, move, update grid and return
            // if obstacle, rotate, invoke self

            if let Some(new_guard_row_index) =
                (guard_row_index + 1 < grid.get_row_length()).then_some(guard_row_index + 1)
            {
                if matches!(
                    grid[new_guard_row_index][guard_column_index],
                    Cell::Obstruction
                ) {
                    let new_direction = direction.rotate_clockwise();

                    // can't move in that direction, travel again, but pointing in new direction
                    grid[guard_row_index][guard_column_index] = Cell::Guard(new_direction);
                    return travel(grid, guard_row_index, guard_column_index, new_direction);
                }

                // shortcut, cell is empty, return new position, maintain direction
                grid[new_guard_row_index][guard_column_index] = Cell::Guard(direction);
                return Some((new_guard_row_index, guard_column_index, direction));
            }

            // OOB
            None
        },
        Direction::Left => {
            // calculate left. If out of bounds, return none
            // if empty, move, update grid and return
            // if obstacle, rotate, invoke self

            if let Some(new_guard_column_index) = guard_column_index.checked_add_signed(-1) {
                if matches!(
                    grid[guard_row_index][new_guard_column_index],
                    Cell::Obstruction
                ) {
                    let new_direction = direction.rotate_clockwise();

                    // can't move in that direction, travel again, but pointing in new direction
                    grid[guard_row_index][guard_column_index] = Cell::Guard(new_direction);
                    return travel(grid, guard_row_index, guard_column_index, new_direction);
                }

                // shortcut, cell is empty, return new position, maintain direction
                grid[guard_row_index][new_guard_column_index] = Cell::Guard(direction);
                return Some((guard_row_index, new_guard_column_index, direction));
            }

            // OOB
            None
        },
    }
}

fn count_possibl_obstacle_positions(input: &str) -> PartSolution {
    let mut grid = parse_input(input);

    let original = grid.clone();

    // starting direction is always up
    let mut direction = Direction::Up;

    let (mut guard_row_index, mut guard_column_index) =
        grid.find(&Cell::Guard(direction)).expect("Bad input");

    let (start_guard_row_index, start_guard_column_index) = (guard_row_index, guard_column_index);

    let mut tried_infinite = BTreeSet::new();

    let mut infinite = 0usize;

    while let Some((r, c, d)) = travel(&mut grid, guard_row_index, guard_column_index, direction) {
        guard_row_index = r;
        guard_column_index = c;
        direction = d;

        // we can now put an obstacle here, restart and see if it's an endless loop
        // one optimization is that we can check whether, once turned (i.e. direction) it abandons the board
        if (guard_row_index, guard_column_index)
            != (start_guard_row_index, start_guard_column_index)
            && tried_infinite.insert((guard_row_index, guard_column_index))
        {
            // we can put an obstacle here
            let mut new_grid = original.clone();
            new_grid[guard_row_index][guard_column_index] = Cell::Obstruction;

            if is_infinite(new_grid) {
                infinite += 1;
            }
        }
    }

    infinite.into()
}

fn is_infinite(mut grid: Grid<Cell>) -> bool {
    let mut traveled = BTreeSet::<(usize, usize, Direction)>::new();

    // starting direction is always up
    let mut direction = Direction::Up;

    let (mut guard_row_index, mut guard_column_index) =
        grid.find(&Cell::Guard(direction)).expect("Bad input");

    // starting position needs to be recorded too
    traveled.insert((guard_row_index, guard_column_index, direction));

    while let Some((r, c, d)) = travel(&mut grid, guard_row_index, guard_column_index, direction) {
        guard_row_index = r;
        guard_column_index = c;
        direction = d;

        // if we have been at this spot, in this direction it's infinite
        if !traveled.insert((guard_row_index, guard_column_index, d)) {
            return true;
        }
    }

    false
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        count_guard_positions(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        count_possibl_obstacle_positions(input)
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
            assert_eq!(4973, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(41, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(1482, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(6, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
