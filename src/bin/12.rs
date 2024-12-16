use advent_of_code_2024::shared::grids::grid::Grid;
use advent_of_code_2024::shared::grids::{GridIter, Neighbors};
use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::HashSet;

advent_of_code_2024::solution!(1_573_474);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Cell(char);

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn parse_input(input: &str) -> Grid<Cell> {
    let v = input
        .trim()
        .lines()
        .map(|line| line.chars().map(Cell).collect::<Vec<Cell>>())
        .collect::<Vec<Vec<Cell>>>();

    Grid::new(v)
}

fn calculate_straights(input: &str) -> PartSolution {
    let grid = parse_input(input);

    let mut seen = HashSet::<(usize, usize)>::new();

    let mut total = 0;

    for ((row_index, column_index), _) in grid.row_column_index_value_iter() {
        if !seen.contains(&(row_index, column_index)) {
            let (area, _, corners) = explore_region(&grid, (row_index, column_index), &mut seen);

            total += area * corners;
        }
    }

    total.into()
}

fn calculate_fence_cost(input: &str) -> PartSolution {
    let grid = parse_input(input);

    let mut seen = HashSet::<(usize, usize)>::new();

    let mut total = 0;

    for ((row_index, column_index), _) in grid.row_column_index_value_iter() {
        if !seen.contains(&(row_index, column_index)) {
            let (area, perimeter, _) = explore_region(&grid, (row_index, column_index), &mut seen);

            total += area * perimeter;
        }
    }

    total.into()
}

fn explore_region(
    grid: &Grid<Cell>,
    (row_index, column_index): (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
) -> (usize, usize, usize) {
    let symbol = grid[row_index][column_index].0;
    let mut to_explore_further = vec![(row_index, column_index)];

    let mut area = 0;
    let mut perimeter = 0;
    let mut corners = 0;

    while let Some((next_row_index, next_column_index)) = to_explore_further.pop() {
        if grid[next_row_index][next_column_index].0 == symbol
            && seen.insert((next_row_index, next_column_index))
        {
            area += 1;

            let mut neighbors = grid
                .hv_neighbors(next_row_index, next_column_index)
                .iter()
                .map(|(coordinates, _)| *coordinates)
                .collect::<Vec<(usize, usize)>>();

            let mut neighbor_perimeter = 4 - neighbors.len();

            for (neighbor_row_index, neighbor_column_index) in &neighbors {
                if grid[*neighbor_row_index][*neighbor_column_index].0 != symbol {
                    neighbor_perimeter += 1;
                }
            }

            perimeter += neighbor_perimeter;

            to_explore_further.append(&mut neighbors);

            corners += count_corners(grid, (next_row_index, next_column_index));
        }
    }

    // corners == sides
    (area, perimeter, corners)
}

fn count_corners(grid: &Grid<Cell>, (row_index, column_index): (usize, usize)) -> usize {
    let mut corners = 0;

    for w in [(-1isize, 0isize), (0, 1), (1, 0), (0, -1), (-1, 0)].windows(2) {
        let [(row_mod_1, column_mod_1), (row_mod_2, column_mod_2)] = w.try_into().unwrap();

        let side_1 = row_index
            .checked_add_signed(row_mod_1)
            .and_then(|row_index| grid.get(row_index))
            .and_then(|row| {
                column_index
                    .checked_add_signed(column_mod_1)
                    .and_then(|column_index| row.get(column_index))
            });

        let side_2 = row_index
            .checked_add_signed(row_mod_2)
            .and_then(|row_index| grid.get(row_index))
            .and_then(|row| {
                column_index
                    .checked_add_signed(column_mod_2)
                    .and_then(|column_index| row.get(column_index))
            });

        let corner = row_index
            .checked_add_signed(row_mod_1)
            .and_then(|r| r.checked_add_signed(row_mod_2))
            .and_then(|row_index| grid.get(row_index))
            .and_then(|row| {
                column_index
                    .checked_add_signed(column_mod_1)
                    .and_then(|c| c.checked_add_signed(column_mod_2))
                    .and_then(|column_index| row.get(column_index))
            });

        let cell = Some(&grid[row_index][column_index]);

        if cell != side_1 && cell != side_2 || (cell == side_1 && cell == side_2 && cell != corner)
        {
            corners += 1;
        }
    }

    corners
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_fence_cost(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        calculate_straights(input)
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
            assert_eq!(1_573_474, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                140,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                772,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                1930,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 3))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::{read_file, read_file_part};
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                259_755_538_429_618_u64,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                1206,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 3))
            );
        }
    }
}
