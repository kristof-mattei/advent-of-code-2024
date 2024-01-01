use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(3503);

enum Tile {
    Rock,
    Garden(bool),
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    let mut map = vec![];

    for line in input.lines() {
        let mut row = vec![];

        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Rock,
                '.' => Tile::Garden(false),
                'S' => Tile::Garden(true),
                _ => panic!("Seriously?"),
            };

            row.push(tile);
        }

        map.push(row);
    }

    map
}

fn steps(mut map: Vec<Vec<Tile>>) -> usize {
    for _ in 1usize..=64 {
        let mut current_gardens = vec![];
        let mut to_flip = vec![];

        for (row_index, row) in map.iter().enumerate() {
            for (column_index, tile) in row.iter().enumerate() {
                if matches!(tile, Tile::Garden(true)) {
                    current_gardens.push((row_index, column_index));
                }
            }
        }

        for (r, c) in current_gardens {
            map[r][c] = Tile::Garden(false);

            if r > 0 {
                to_flip.push((r - 1, c));
            }

            if c > 0 {
                to_flip.push((r, c - 1));
            }

            if r + 1 < map.len() {
                to_flip.push((r + 1, c));
            }

            if c + 1 < map[r].len() {
                to_flip.push((r, c + 1));
            }
        }

        for (r, c) in to_flip {
            if matches!(map[r][c], Tile::Garden(false)) {
                map[r][c] = Tile::Garden(true);
            }
        }
    }

    calculate_sum(&map)
}

fn calculate_sum(map: &[Vec<Tile>]) -> usize {
    map.iter()
        .map(|row| {
            row.iter().fold(0, |acc, curr| {
                if matches!(curr, Tile::Garden(true)) {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .sum()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let map = parse_input(input);

        steps(map).into()
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        None.into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(3503, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(42, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::{PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
