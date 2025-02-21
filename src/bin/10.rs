use advent_of_code_2024::shared::grids::grid::Grid;
use advent_of_code_2024::shared::grids::{GridIter, Neighbors};
use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::HashSet;

advent_of_code_2024::solution!(674usize, 1372);

#[derive(PartialEq, Eq)]
enum Cell {
    Number(u32),
    Empty,
}

fn parse_input(input: &str) -> Grid<Cell> {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).map_or(Cell::Empty, Cell::Number))
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>(),
    )
}

fn count_longest_paths(input: &str) -> PartSolution {
    let grid = parse_input(input);

    let starts = grid
        .row_column_index_value_iter()
        .filter_map(|((row_index, column_index), value)| match value {
            &Cell::Number(0) => Some((row_index, column_index)),
            _ => None,
        })
        .collect::<Vec<(usize, usize)>>();

    let mut completed_paths = 0;

    for (row_index, column_index) in starts {
        let mut got_to_nine = HashSet::new();
        follow_0_to_9(&grid, row_index, column_index, 0, &mut got_to_nine);

        completed_paths += got_to_nine.len();
    }

    completed_paths.into()
}

fn follow_0_to_9(
    grid: &Grid<Cell>,
    row_index: usize,
    column_index: usize,
    number: u32,
    got_to_nine_coordinates: &mut HashSet<(usize, usize)>,
) {
    if number == 9 {
        got_to_nine_coordinates.insert((row_index, column_index));
        return;
    }

    for ((neighbor_row_index, neighbor_column_index), _) in
        grid.hv_neighbors(row_index, column_index)
    {
        if grid[neighbor_row_index][neighbor_column_index] == Cell::Number(number + 1) {
            follow_0_to_9(
                grid,
                neighbor_row_index,
                neighbor_column_index,
                number + 1,
                got_to_nine_coordinates,
            );
        }
    }
}

fn follow_all_paths(input: &str) -> PartSolution {
    let grid = parse_input(input);

    let starts = grid
        .row_column_index_value_iter()
        .filter_map(|((row_index, column_index), value)| match value {
            &Cell::Number(0) => Some((row_index, column_index)),
            _ => None,
        })
        .collect::<Vec<(usize, usize)>>();

    let mut completed_paths = 0;

    for (row_index, column_index) in starts {
        let this_one = follow_all_0_to_9(&grid, row_index, column_index, 0);
        completed_paths += this_one;
    }

    completed_paths.into()
}

fn follow_all_0_to_9(grid: &Grid<Cell>, row_index: usize, column_index: usize, number: u32) -> u32 {
    let mut nexts = 0;

    if number == 9 {
        return 1;
    }

    for ((neighbor_row_index, neighbor_column_index), _) in
        grid.hv_neighbors(row_index, column_index)
    {
        if grid[neighbor_row_index][neighbor_column_index] == Cell::Number(number + 1) {
            nexts += follow_all_0_to_9(grid, neighbor_row_index, neighbor_column_index, number + 1);
        }
    }

    nexts
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        count_longest_paths(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        follow_all_paths(input)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use advent_of_code_2024::shared::Parts;
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(674usize, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(
                2,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                4,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                3,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 3))
            );
        }

        #[test]
        fn example_4() {
            assert_eq!(
                36,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 4))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts;
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(1372u32, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_2() {
            assert_eq!(
                13,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 2))
            );
        }
    }
}
