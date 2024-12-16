use advent_of_code_2024::shared::grids::grid::Grid;
use advent_of_code_2024::shared::grids::{
    GridIter, HorizontalVerticalDiagonalDirection, Neighbors,
};
use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!(2447, 1868);

fn count_xmas(input: &str) -> PartSolution {
    let g = Grid::new(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );

    let mut solution = 0;

    for ((row_index, column_index), c) in g.row_column_index_value_iter() {
        if 'X' == *c {
            // good starting point, trace neighbors
            let neighbors = g.hvd_neighbors(row_index, column_index);

            for ((_, _), direction) in neighbors {
                if is_merry(&g, row_index, column_index, &direction, &['M', 'A', 'S']) {
                    solution += 1;
                }
            }
        }
    }

    PartSolution::U32(solution)
}

fn is_merry(
    grid: &Grid<char>,
    row_index: usize,
    column_index: usize,
    direction: &HorizontalVerticalDiagonalDirection,
    to_match: &[char],
) -> bool {
    if to_match.is_empty() {
        return true;
    }

    if let Some((row_index, column_index, direction)) =
        follow(grid, row_index, column_index, direction)
    {
        if grid[row_index][column_index] == to_match[0] {
            return is_merry(grid, row_index, column_index, &direction, &to_match[1..]);
        }
    }

    false
}

fn follow(
    grid: &Grid<char>,
    row_index: usize,
    column_index: usize,
    direction: &HorizontalVerticalDiagonalDirection,
) -> Option<(usize, usize, HorizontalVerticalDiagonalDirection)> {
    let up = row_index.checked_sub(1);
    let down = {
        let down = row_index + 1;

        (down < grid.get_row_length()).then_some(down)
    };

    let left = column_index.checked_sub(1);
    let right = {
        let right = column_index + 1;

        (right < grid.get_column_length()).then_some(right)
    };

    match direction {
        HorizontalVerticalDiagonalDirection::Up => {
            up.map(|up| (up, column_index, HorizontalVerticalDiagonalDirection::Up))
        },
        HorizontalVerticalDiagonalDirection::UpRight => {
            if let (Some(up), Some(right)) = (up, right) {
                Some((up, right, HorizontalVerticalDiagonalDirection::UpRight))
            } else {
                None
            }
        },
        HorizontalVerticalDiagonalDirection::Right => {
            right.map(|right| (row_index, right, HorizontalVerticalDiagonalDirection::Right))
        },
        HorizontalVerticalDiagonalDirection::DownRight => {
            if let (Some(down), Some(right)) = (down, right) {
                Some((down, right, HorizontalVerticalDiagonalDirection::DownRight))
            } else {
                None
            }
        },
        HorizontalVerticalDiagonalDirection::Down => down.map(|down| {
            (
                down,
                column_index,
                HorizontalVerticalDiagonalDirection::Down,
            )
        }),
        HorizontalVerticalDiagonalDirection::DownLeft => {
            if let (Some(down), Some(left)) = (down, left) {
                Some((down, left, HorizontalVerticalDiagonalDirection::DownLeft))
            } else {
                None
            }
        },
        HorizontalVerticalDiagonalDirection::Left => {
            left.map(|left| (row_index, left, HorizontalVerticalDiagonalDirection::Left))
        },
        HorizontalVerticalDiagonalDirection::UpLeft => {
            if let (Some(up), Some(left)) = (up, left) {
                Some((up, left, HorizontalVerticalDiagonalDirection::UpLeft))
            } else {
                None
            }
        },
    }
}

fn count_max_as_x(input: &str) -> PartSolution {
    let g = Grid::new(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );

    let mut solution = 0;

    for ((row_index, column_index), c) in g.row_column_index_value_iter() {
        if 'A' == *c {
            // we can have
            // M M or S M or S S or M S
            //  A      A      A      A
            // S S    S M    M M    M S

            // clockwise, from left, top
            let to_check = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

            // clockwise, from left, top
            let mut expected = ['M', 'M', 'S', 'S'];

            for _ in 0..expected.len() {
                let mut matches = 0;

                for (&(mod_x, mod_y), expected_value) in to_check.iter().zip(expected) {
                    if let (Some(row_index), Some(column_index)) = (
                        row_index
                            .checked_add_signed(mod_x)
                            .filter(|&r| r < g.get_row_length()),
                        column_index
                            .checked_add_signed(mod_y)
                            .filter(|&c| c < g.get_column_length()),
                    ) {
                        // there is a value there
                        if g[row_index][column_index] == expected_value {
                            matches += 1;
                            // ... go on
                        }
                    } else {
                        break;
                    }
                }

                if matches == expected.len() {
                    solution += 1;
                }

                expected.rotate_right(1);
            }
        }
    }

    PartSolution::U32(solution)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        count_xmas(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        count_max_as_x(input)
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
            assert_eq!(2447, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(18, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(1868, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(9, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
