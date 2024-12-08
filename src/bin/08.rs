use advent_of_code_2024::shared::grids::grid::Grid;
use advent_of_code_2024::shared::grids::GridIter;
use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::{HashMap, HashSet};

advent_of_code_2024::solution!(381, 1184);

#[derive(PartialEq, Eq)]
enum Cell {
    Antenna(char),
    Nothing,
    Antinode,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Antenna(c) => write!(f, "{}", c),
            Cell::Nothing => write!(f, "."),
            Cell::Antinode => write!(f, "#"),
        }
    }
}

struct Parsed {
    grid: Grid<Cell>,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

fn parse_into_grid_and_group_antennas(input: &str) -> Parsed {
    let parsed = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Nothing,
                    v => Cell::Antenna(v),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let grid = Grid::new(parsed);

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for ((row_index, column_index), cell) in grid.row_column_index_value_iter() {
        if let Cell::Antenna(c) = cell {
            antennas
                .entry(*c)
                .and_modify(|v| {
                    v.push((row_index, column_index));
                })
                .or_insert(vec![(row_index, column_index)]);
        }
    }

    Parsed { grid, antennas }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Times {
    Once,
    Infinite,
}

fn count_antinodes(input: &str, times: Times) -> PartSolution {
    let Parsed { mut grid, antennas } = parse_into_grid_and_group_antennas(input);

    let mut antinodes = HashSet::new();

    for (_key, value) in &antennas {
        let permutations = permutations(value);

        for ((mut r_1, mut c_1), (mut r_2, mut c_2)) in permutations {
            if times == Times::Infinite {
                // when infinite we count ourselves
                antinodes.insert((r_1, c_1));
                antinodes.insert((r_2, c_2));
            }

            let r_diff = checked_signed_diff(r_1, r_2).expect("OOB");
            let c_diff = checked_signed_diff(c_1, c_2).expect("OOB");

            loop {
                // and apply r_diff and c_diff to r_1 and c_1 to get antinode 1
                // notice no negation on r_diff and c_diff
                let r_1_antinode = r_1.checked_add_signed(r_diff);
                let c_1_antinode = c_1.checked_add_signed(c_diff);

                if let (Some(r_1_antinode), Some(c_1_antinode)) = (r_1_antinode, c_1_antinode) {
                    let cell = grid.get(r_1_antinode).and_then(|r| r.get(c_1_antinode));

                    if cell.is_some() {
                        antinodes.insert((r_1_antinode, c_1_antinode));

                        r_1 = r_1_antinode;
                        c_1 = c_1_antinode;
                    } else {
                        // OOB
                        break;
                    }
                } else {
                    break;
                }

                if times == Times::Once {
                    break;
                }
            }

            loop {
                // we now apply the opposite of r_diff and c_diff to r_2 and c_2 to get antinode 2
                // notice negation on r_diff and c_diff
                let r_2_antinode = r_2.checked_add_signed(-r_diff);
                let c_2_antinode = c_2.checked_add_signed(-c_diff);

                if let (Some(r_2_antinode), Some(c_2_antinode)) = (r_2_antinode, c_2_antinode) {
                    let cell = grid.get(r_2_antinode).and_then(|r| r.get(c_2_antinode));

                    if cell.is_some() {
                        antinodes.insert((r_2_antinode, c_2_antinode));
                        r_2 = r_2_antinode;
                        c_2 = c_2_antinode;
                    } else {
                        // OOB
                        break;
                    }
                } else {
                    break;
                }

                if times == Times::Once {
                    break;
                }
            }
        }
    }

    for antinode in &antinodes {
        grid[antinode.0][antinode.1] = Cell::Antinode;
    }

    println!("{}", grid);

    antinodes.len().into()
}

const fn checked_signed_diff(lhs: usize, rhs: usize) -> Option<isize> {
    #[expect(clippy::cast_possible_wrap)]
    let result = lhs.wrapping_sub(rhs) as isize;
    let overflow = (lhs >= rhs) == (result < 0);

    if !overflow {
        return Some(result);
    }

    None
}

fn permutations(v: &[(usize, usize)]) -> Vec<((usize, usize), (usize, usize))> {
    let mut permutations = vec![];

    for (outer_index, outer_value) in v.iter().enumerate() {
        for inner_value in v.iter().skip(outer_index + 1) {
            permutations.push((*outer_value, *inner_value));
        }
    }

    permutations
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        count_antinodes(input, Times::Once)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        count_antinodes(input, Times::Infinite)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {

        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{checked_signed_diff, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(381, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(14, (Solution {}).part_1(&read_file("examples", &DAY)));
        }

        #[test]
        fn test_wrapping_sub() {
            assert_eq!(checked_signed_diff(4, 5), Some(-1isize));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(1184, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(34, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
