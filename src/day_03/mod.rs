use crate::shared::{Day, PartSolution};

enum Cell {
    Number(u32),
    Symbol(),
    Empty(),
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty()),
            '0'..='9' => {
                // 48 is the ascii value of '0'
                let n = u32::try_from(c).unwrap() - 48;

                Ok(Cell::Number(n))
            },
            _ => Ok(Cell::Symbol()),
        }
    }
}

fn parse_lines(lines: &[&str]) -> Schematic {
    let mut parsed = Vec::new();

    for line in lines {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(Cell::try_from(c).unwrap());
        }

        parsed.push(row);
    }

    parsed
}

type Schematic = Vec<Vec<Cell>>;

fn parse_number_from(row: &[Cell], start_index: usize) -> Option<(usize, u32)> {
    let (length, number) = row[start_index..]
        .iter()
        .map_while(|c| {
            if let Cell::Number(n) = c {
                Some(*n)
            } else {
                None
            }
        })
        .fold((0, 0), |(length, value), c| (length + 1, value * 10 + c));

    if length > 0 {
        Some((length, number))
    } else {
        None
    }
}

fn cells_to_check(
    schematic: &Schematic,
    row_index: usize,
    column_index: usize,
    length: usize,
) -> Vec<(usize, usize)> {
    let mut cells = vec![];

    let row_index_minus_one = row_index.checked_sub(1);

    let before_number_index = column_index.checked_sub(1);

    let row_index_plus_one = if row_index + 1 == schematic.len() {
        None
    } else {
        Some(row_index + 1)
    };

    // no plus one here as length is cardinal and indexes are ordinal
    let after_number_index = if column_index + length == schematic[row_index].len() {
        None
    } else {
        Some(column_index + length)
    };

    if let Some(row_above_index) = row_index_minus_one {
        for column_index in before_number_index.unwrap_or(column_index)
            ..=after_number_index.unwrap_or(column_index + length - 1)
        {
            cells.push((row_above_index, column_index));
        }
    }

    if let Some(row_below_index) = row_index_plus_one {
        for column_index in before_number_index.unwrap_or(column_index)
            ..=after_number_index.unwrap_or(column_index + length - 1)
        {
            cells.push((row_below_index, column_index));
        }
    }

    if let Some(b) = before_number_index {
        cells.push((row_index, b));
    }

    if let Some(a) = after_number_index {
        cells.push((row_index, a));
    }

    cells
}

fn sum_all_part_numbers(schematic: &Schematic) -> u32 {
    let mut sum = 0;

    for row_index in 0..schematic.len() {
        let mut column_index = 0;

        while column_index < schematic[row_index].len() {
            if let Some((length, number)) = parse_number_from(&schematic[row_index], column_index) {
                // check all cells at (row - 1)

                let cells_to_check = cells_to_check(schematic, row_index, column_index, length);

                let sum_of_symbols = cells_to_check
                    .iter()
                    .filter(|(r, c)| matches!(schematic[*r][*c], Cell::Symbol()))
                    .count();

                if sum_of_symbols > 0 {
                    println!(
                        "Found number {} at ({}, {}) for length {} with {} symbols",
                        number, row_index, column_index, length, sum_of_symbols
                    );

                    sum += number;
                }

                column_index += length;
            } else {
                column_index += 1;
            }
        }
    }

    sum
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        sum_all_part_numbers(&parsed).into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let _parsed = parse_lines(&lines);

        todo!()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use super::super::parse_lines;
        use super::super::Solution;
        use super::get_example;
        use crate::{day_03::sum_all_part_numbers, shared::Day};

        #[test]
        fn outcome() {
            assert_eq!(527_364, (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            let sum = sum_all_part_numbers(&parsed);

            assert_eq!(4361, sum);
        }
    }

    mod part_2 {
        use crate::shared::Day;
        use crate::shared::PartSolution;

        use super::super::parse_lines;
        use super::super::Solution;
        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(5_756_764), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            // assert_eq!(Vec::<u32>::new(), parsed);
        }
    }
}
