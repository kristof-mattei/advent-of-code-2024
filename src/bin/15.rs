use std::ops::Neg;

use advent_of_code_2024::shared::{PartSolution, Parts};
use part_1::solve_grid;
use part_2::solve_doubled_grid;

advent_of_code_2024::solution!(1_497_888);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_n_positions<const N: usize>(
        self,
        position: (usize, usize),
        max_rows: usize,
        max_columns: usize,
    ) -> Option<(usize, usize)> {
        let negative_n = isize::try_from(N).unwrap().neg();

        match self {
            Direction::Up => position
                .0
                .checked_add_signed(negative_n)
                .map(|r| (r, position.1)),
            Direction::Right => {
                if position.1 + N < max_columns {
                    Some((position.0, position.1 + N))
                } else {
                    None
                }
            },
            Direction::Down => {
                if position.0 + N < max_rows {
                    Some((position.0 + N, position.1))
                } else {
                    None
                }
            },
            Direction::Left => position
                .1
                .checked_add_signed(negative_n)
                .map(|c| (position.0, c)),
        }
    }

    fn next_position(
        self,
        position: (usize, usize),
        max_rows: usize,
        max_columns: usize,
    ) -> Option<(usize, usize)> {
        self.next_n_positions::<1>(position, max_rows, max_columns)
    }

    fn next_double_position(
        self,
        position: (usize, usize),
        max_rows: usize,
        max_columns: usize,
    ) -> Option<(usize, usize)> {
        self.next_n_positions::<2>(position, max_rows, max_columns)
    }
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => Err("Invalid character"),
        }
    }
}

mod part_1 {
    use advent_of_code_2024::shared::grids::grid::Grid;
    use advent_of_code_2024::shared::grids::GridIter;
    use advent_of_code_2024::shared::PartSolution;

    use crate::Direction;

    #[derive(PartialEq, Eq, Clone, Copy)]
    enum Cell {
        Empty,
        Box,
        Wall,
        Robot,
    }

    impl TryFrom<char> for Cell {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '@' => Ok(Cell::Robot),
                '.' => Ok(Cell::Empty),
                '#' => Ok(Cell::Wall),
                'O' => Ok(Cell::Box),
                _ => Err("Invalid character"),
            }
        }
    }

    impl std::fmt::Debug for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let c = match self {
                Cell::Empty => '.',
                Cell::Box => 'O',
                Cell::Wall => '#',
                Cell::Robot => '@',
            };

            write!(f, "{}", c)
        }
    }

    fn parse_field_line(line: &str) -> Vec<Cell> {
        let mut cells = Vec::with_capacity(line.len());

        for cell in line.chars().map(|c| Cell::try_from(c).expect("Bad input")) {
            cells.push(cell);
        }

        cells
    }

    fn parse_input(input: &str) -> (Grid<Cell>, Vec<Direction>) {
        let mut field = Vec::<Vec<Cell>>::new();
        let mut moves = Vec::<Direction>::new();

        let mut after_linebreak = false;

        for line in input.trim().lines() {
            if line.is_empty() {
                after_linebreak = true;
            }

            if after_linebreak {
                for direction in line
                    .chars()
                    .map(|c| Direction::try_from(c).expect("Bad input"))
                {
                    moves.push(direction);
                }
            } else {
                field.push(parse_field_line(line));
            }
        }

        let grid = Grid::new(field);

        (grid, moves)
    }

    fn move_robot(grid: &mut Grid<Cell>, moves: &[Direction]) {
        let mut robot_position = grid
            .row_column_index_value_iter()
            .find(|p| matches!(p, Cell::Robot))
            .expect("Robot gone?");

        for direction in moves {
            // we move the robot in direction.
            // if it's a wall, we don't move.
            // if it's empty, we move
            // if it's a block, we go into the same direction and see if we find an empty spot until we run out of space
            // and shift ourselves and ALL blocks to direction

            let next_robot_position = direction.next_position(
                robot_position,
                grid.get_row_length(),
                grid.get_column_length(),
            );

            let Some(next_robot_position) = next_robot_position else {
                continue;
            };

            match grid[next_robot_position.0][next_robot_position.1] {
                Cell::Empty => {
                    // Move robot
                    grid[robot_position.0][robot_position.1] = Cell::Empty;
                    grid[next_robot_position.0][next_robot_position.1] = Cell::Robot;

                    robot_position = next_robot_position;
                },
                Cell::Box => {
                    // in the direction we're going, see if there is an empty spot at the end
                    let mut next = next_robot_position;

                    while let Some(next_next) = direction.next_position(
                        next,
                        grid.get_row_length(),
                        grid.get_column_length(),
                    ) {
                        match grid[next_next.0][next_next.1] {
                            Cell::Box => {
                                next = next_next;
                            },
                            Cell::Empty => {
                                // Move robot
                                grid[robot_position.0][robot_position.1] = Cell::Empty;
                                grid[next_robot_position.0][next_robot_position.1] = Cell::Robot;

                                // Mark last Empty as Box, and now it looks like the robot pushed the whole train
                                grid[next_next.0][next_next.1] = Cell::Box;

                                robot_position = next_robot_position;
                                break;
                            },
                            Cell::Wall => {
                                // can't do anything
                                break;
                            },
                            Cell::Robot => panic!("2 robots?"),
                        }
                    }
                },
                Cell::Wall => {
                    continue;
                },
                Cell::Robot => panic!("Only 1 robot allowed, and we're the robot"),
            }
        }
    }

    fn calculate_gps_positions(grid: &Grid<Cell>) -> PartSolution {
        let mut gps_positions = 0;

        for ((row_index, column_index), cell) in grid.row_column_index_value_iter() {
            if matches!(cell, Cell::Box) {
                gps_positions += 100 * row_index + column_index;
            }
        }

        gps_positions.into()
    }

    pub(super) fn solve_grid(input: &str) -> PartSolution {
        let (mut grid, moves) = parse_input(input);

        move_robot(&mut grid, &moves);

        calculate_gps_positions(&grid)
    }
}

mod part_2 {
    use std::cmp::Reverse;

    use advent_of_code_2024::shared::grids::grid::Grid;
    use advent_of_code_2024::shared::grids::GridIter;
    use advent_of_code_2024::shared::PartSolution;

    use crate::Direction;

    #[derive(PartialEq, Eq, Clone, Copy)]
    enum Cell {
        Empty,
        BoxLeft,
        BoxRight,
        Wall,
        Robot,
    }

    impl std::fmt::Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let c = match self {
                Cell::Empty => '.',
                Cell::BoxLeft => '[',
                Cell::BoxRight => ']',
                Cell::Wall => '#',
                Cell::Robot => '@',
            };

            write!(f, "{}", c)
        }
    }

    struct BlockMove {
        left_piece_original: (usize, usize),
        left_piece_new: (usize, usize),
        right_piece_original: (usize, usize),
        right_piece_new: (usize, usize),
    }

    fn parse_double_field_line(line: &str) -> Vec<Cell> {
        let mut cells = Vec::with_capacity(line.len());

        for (c1, c2) in line.chars().map(|c| match c {
            '@' => (Cell::Robot, Cell::Empty),
            '.' => (Cell::Empty, Cell::Empty),
            '#' => (Cell::Wall, Cell::Wall),
            'O' => (Cell::BoxLeft, Cell::BoxRight),
            _ => panic!("Bad input"),
        }) {
            cells.push(c1);
            cells.push(c2);
        }

        cells
    }

    fn parse_input_doubled(input: &str) -> (Grid<Cell>, Vec<Direction>) {
        let mut field = Vec::<Vec<Cell>>::new();
        let mut moves = Vec::<Direction>::new();

        let mut after_linebreak = false;

        for line in input.trim().lines() {
            if line.is_empty() {
                after_linebreak = true;
            }

            if after_linebreak {
                for direction in line
                    .chars()
                    .map(|c| Direction::try_from(c).expect("Bad input"))
                {
                    moves.push(direction);
                }
            } else {
                field.push(parse_double_field_line(line));
            }
        }

        let grid = Grid::new(field);

        (grid, moves)
    }

    fn move_robot_doubled_grid(grid: &mut Grid<Cell>, moves: &[Direction]) {
        let mut robot_position = grid
            .row_column_index_value_iter()
            .find(|p| matches!(p, Cell::Robot))
            .expect("Robot gone?");

        for direction in moves {
            let next_robot_position = direction.next_position(
                robot_position,
                grid.get_row_length(),
                grid.get_column_length(),
            );

            let Some(next_robot_position) = next_robot_position else {
                continue;
            };

            match grid[next_robot_position.0][next_robot_position.1] {
                Cell::Empty => {
                    // Move robot
                    grid[robot_position.0][robot_position.1] = Cell::Empty;
                    grid[next_robot_position.0][next_robot_position.1] = Cell::Robot;

                    robot_position = next_robot_position;
                },
                Cell::BoxLeft | Cell::BoxRight => {
                    // if direction is left / right, we basically just push again
                    match direction {
                        Direction::Left | Direction::Right => {
                            let Some(instructions) =
                                calculate_left_right_moves(grid, next_robot_position, *direction)
                            else {
                                continue;
                            };

                            for BlockMove {
                                left_piece_original,
                                right_piece_original,
                                left_piece_new: new_left,
                                right_piece_new: new_right,
                            } in instructions
                            {
                                grid[left_piece_original.0][left_piece_original.1] = Cell::Empty;
                                grid[right_piece_original.0][right_piece_original.1] = Cell::Empty;

                                grid[new_left.0][new_left.1] = Cell::BoxLeft;
                                grid[new_right.0][new_right.1] = Cell::BoxRight;
                            }

                            grid[robot_position.0][robot_position.1] = Cell::Empty;
                            grid[next_robot_position.0][next_robot_position.1] = Cell::Robot;
                            robot_position = next_robot_position;

                            // {
                            //     // Move robot
                            //     grid[robot_position.0][robot_position.1] = Cell::Empty;
                            //     grid[next_robot_position.0][next_robot_position.1] = Cell::Robot;
                            //     robot_position = next_robot_position;
                            // }
                        },
                        Direction::Up | Direction::Down => {
                            let Some(mut instructions) =
                                calculate_up_down_moves(grid, next_robot_position, *direction)
                            else {
                                continue;
                            };

                            if *direction == Direction::Up {
                                instructions.sort_by_key(
                                    |&BlockMove {
                                         left_piece_original,
                                         ..
                                     }| left_piece_original.0,
                                );
                            } else {
                                instructions.sort_by_key(
                                    |&BlockMove {
                                         left_piece_original,
                                         ..
                                     }| {
                                        Reverse(left_piece_original.0)
                                    },
                                );
                            }

                            for BlockMove {
                                left_piece_original,
                                right_piece_original,
                                left_piece_new,
                                right_piece_new,
                            } in instructions
                            {
                                grid[left_piece_original.0][left_piece_original.1] = Cell::Empty;
                                grid[right_piece_original.0][right_piece_original.1] = Cell::Empty;

                                grid[left_piece_new.0][left_piece_new.1] = Cell::BoxLeft;
                                grid[right_piece_new.0][right_piece_new.1] = Cell::BoxRight;
                            }

                            grid[robot_position.0][robot_position.1] = Cell::Empty;
                            grid[next_robot_position.0][next_robot_position.1] = Cell::Robot;
                            robot_position = next_robot_position;
                        },
                    }
                },
                Cell::Wall => {
                    continue;
                },
                Cell::Robot => panic!("Only 1 robot allowed, and we're the robot"),
            }
        }
    }

    fn calculate_gps_positions_doubled_grid(grid: &Grid<Cell>) -> PartSolution {
        let mut gps_positions = 0;

        for ((row_index, column_index), cell) in grid.row_column_index_value_iter() {
            if matches!(cell, Cell::BoxLeft) {
                gps_positions += 100 * row_index + column_index;
            }
        }

        gps_positions.into()
    }

    fn calculate_up_down_moves(
        grid: &Grid<Cell>,
        position: (usize, usize),
        direction: Direction,
    ) -> Option<Vec<BlockMove>> {
        // expand block
        let (original_left, original_right) =
            if matches!(grid[position.0][position.1], Cell::BoxLeft) {
                ((position.0, position.1), (position.0, position.1 + 1))
            } else {
                ((position.0, position.1 - 1), (position.0, position.1))
            };

        // try and move the pieces up or down
        // if the next position is wall or robot (!), fail
        // if the next position is empty, save (TODO!)
        // if the next position is a piece, kk

        // can we move both pieces in the direction?
        let (Some(left_moved), Some(right_moved)) = (
            direction.next_position(
                original_left,
                grid.get_row_length(),
                grid.get_column_length(),
            ),
            direction.next_position(
                original_right,
                grid.get_row_length(),
                grid.get_column_length(),
            ),
        ) else {
            return None;
        };

        match (
            grid[left_moved.0][left_moved.1],
            grid[right_moved.0][right_moved.1],
        ) {
            (Cell::BoxRight, Cell::Empty) | (Cell::BoxLeft, Cell::BoxRight) => {
                let mut left_descendant_moves =
                    calculate_up_down_moves(grid, left_moved, direction)?;

                left_descendant_moves.push(BlockMove {
                    left_piece_original: original_left,
                    right_piece_original: original_right,

                    left_piece_new: left_moved,
                    right_piece_new: right_moved,
                });

                Some(left_descendant_moves)
            },
            (Cell::Empty, Cell::BoxLeft) => {
                let mut right_descendants_moves =
                    calculate_up_down_moves(grid, right_moved, direction)?;

                right_descendants_moves.push(BlockMove {
                    left_piece_original: original_left,
                    right_piece_original: original_right,

                    left_piece_new: left_moved,
                    right_piece_new: right_moved,
                });

                Some(right_descendants_moves)
            },
            (Cell::BoxRight, Cell::BoxLeft) => {
                let mut ll = calculate_up_down_moves(grid, left_moved, direction)?;

                let mut rr = calculate_up_down_moves(grid, right_moved, direction)?;

                ll.append(&mut rr);

                ll.push(BlockMove {
                    left_piece_original: original_left,
                    right_piece_original: original_right,
                    left_piece_new: left_moved,
                    right_piece_new: right_moved,
                });

                Some(ll)
            },
            (Cell::Empty, Cell::Empty) => Some(vec![BlockMove {
                left_piece_original: original_left,
                right_piece_original: original_right,
                left_piece_new: left_moved,
                right_piece_new: right_moved,
            }]),
            _ => None,
        }
    }

    fn calculate_left_right_moves(
        grid: &mut Grid<Cell>,
        position: (usize, usize),
        direction: Direction,
    ) -> Option<Vec<BlockMove>> {
        // in the direction we're going, see if there is an empty spot at the end
        let mut block_piece_position = position;

        while let Some(next_block_piece_position) = direction.next_double_position(
            block_piece_position,
            grid.get_row_length(),
            grid.get_column_length(),
        ) {
            match grid[next_block_piece_position.0][next_block_piece_position.1] {
                Cell::Empty => {
                    if direction == Direction::Right {
                        let mut instructions = vec![];

                        // reverse, as we want to move the blocks from right to left to prevent overwriting previous one
                        for column in (position.1..next_block_piece_position.1).step_by(2).rev() {
                            instructions.push(BlockMove {
                                left_piece_original: (position.0, column),
                                left_piece_new: (position.0, column + 1),
                                right_piece_original: (position.0, column + 1),
                                right_piece_new: (position.0, column + 2),
                            });
                        }

                        return Some(instructions);
                    } else if direction == Direction::Left {
                        let mut instructions = vec![];

                        for column in (next_block_piece_position.1 + 1..position.1).step_by(2) {
                            instructions.push(BlockMove {
                                left_piece_original: (position.0, column),
                                left_piece_new: (position.0, column - 1),
                                right_piece_original: (position.0, column + 1),
                                right_piece_new: (position.0, column),
                            });
                        }

                        return Some(instructions);
                    }

                    panic!()

                    // return Some(position);
                },
                Cell::BoxLeft | Cell::BoxRight => {
                    block_piece_position = next_block_piece_position;
                },
                Cell::Wall => {
                    // can't do anything
                    return None;
                },
                Cell::Robot => panic!("2 robots?"),
            }
        }

        None
    }

    pub(super) fn solve_doubled_grid(input: &str) -> PartSolution {
        let (mut grid, directions) = parse_input_doubled(input);

        move_robot_doubled_grid(&mut grid, &directions);

        calculate_gps_positions_doubled_grid(&grid)
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        solve_grid(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        solve_doubled_grid(input)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(1_497_888, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                2028,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                10092,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }
    }

    mod part_2 {

        use advent_of_code_2024::shared::solution::{read_file, read_file_part};
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(1_522_420, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_2() {
            assert_eq!(
                9021,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 2))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                535,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 3))
            );
        }

        #[test]
        fn example_4() {
            assert_eq!(
                618,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 4))
            );
        }

        #[test]
        fn example_5() {
            assert_eq!(
                1224,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 5))
            );
        }
    }
}
