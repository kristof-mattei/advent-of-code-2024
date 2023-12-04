use crate::shared::{Day, PartSolution};

enum Cell {
    Number(u32),
    Symbol(char),
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
            _ => Ok(Cell::Symbol(c)),
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

fn find_number_at(row: &[Cell], column_index: usize) -> Option<u32> {
    // first we check if we are at the start of our number
    // on the left it must either be Empty or Symbol, just not a number
    // this avoids cases like
    // 123
    //  *
    // 456
    // and we landed on the (0,1). We should then ignore the number as it will be covered by the left-top check

    // are we on a number? If not, return
    if !matches!(row[column_index], Cell::Number(_)) {
        return None;
    }

    // we're on a number, let's make sure we're on the start on the number
    let mut start = column_index;

    while start > 0 && matches!(row[start - 1], Cell::Number(_)) {
        start -= 1;
    }

    // okay we're at the start of a number
    let mut number = 0;

    for c in &row[start..] {
        if let Cell::Number(n) = c {
            number = number * 10 + n;
        } else {
            break;
        }
    }

    Some(number)
}

fn multiply_gear_numbers(schematic: &Schematic) -> u32 {
    let mut sum = 0;

    for (row_index, row) in schematic.iter().enumerate() {
        for column_index in 0..row.len() {
            if matches!(row[column_index], Cell::Symbol('*')) {
                let mut gear_ratios = vec![];
                let row_above = row_index.checked_sub(1).and_then(|ri| schematic.get(ri));
                let row_below = schematic.get(row_index + 1);

                let can_go_left = column_index > 0;
                let can_go_right = column_index + 1 < schematic[row_index].len();

                // left
                if can_go_left {
                    if let Some(n) = find_number_at(row, column_index - 1) {
                        gear_ratios.push(n);
                    }
                }

                // right
                if can_go_right {
                    if let Some(n) = find_number_at(row, column_index + 1) {
                        gear_ratios.push(n);
                    }
                }

                if let Some(row_above) = row_above {
                    // if we have a number right above us then that is the only number that can touch us
                    // as we go as far left/right as we can
                    if let Some(n) = find_number_at(row_above, column_index) {
                        gear_ratios.push(n);
                    } else {
                        // left-top
                        if can_go_left {
                            if let Some(n) = find_number_at(row_above, column_index - 1) {
                                gear_ratios.push(n);
                            }
                        }

                        // right-top
                        if can_go_right {
                            if let Some(n) = find_number_at(row_above, column_index + 1) {
                                gear_ratios.push(n);
                            }
                        }
                    }
                }

                if let Some(row_below) = row_below {
                    if let Some(n) = find_number_at(row_below, column_index) {
                        gear_ratios.push(n);
                    } else {
                        // left-bottom
                        if can_go_left {
                            if let Some(n) = find_number_at(row_below, column_index - 1) {
                                gear_ratios.push(n);
                            }
                        }

                        // right-bottom
                        if can_go_right {
                            if let Some(n) = find_number_at(row_below, column_index + 1) {
                                gear_ratios.push(n);
                            }
                        }
                    }
                }

                if gear_ratios.len() > 1 {
                    sum += gear_ratios.iter().product::<u32>();
                }
            }
        }
    }

    sum
}

fn sum_all_part_numbers(schematic: &Schematic) -> u32 {
    let mut sum = 0;

    for (row_index, row) in schematic.iter().enumerate() {
        for column_index in 0..row.len() {
            if matches!(row[column_index], Cell::Symbol(_)) {
                let row_above = row_index.checked_sub(1).and_then(|ri| schematic.get(ri));
                let row_below = schematic.get(row_index + 1);

                let can_go_left = column_index > 0;
                let can_go_right = column_index + 1 < schematic[row_index].len();

                // left
                if can_go_left {
                    if let Some(n) = find_number_at(row, column_index - 1) {
                        sum += n;
                    }
                }

                // right
                if can_go_right {
                    if let Some(n) = find_number_at(row, column_index + 1) {
                        sum += n;
                    }
                }

                if let Some(row_above) = row_above {
                    // if we have a number right above us then that is the only number that can touch us
                    // as we go as far left/right as we can
                    if let Some(n) = find_number_at(row_above, column_index) {
                        sum += n;
                    } else {
                        // left-top
                        if can_go_left {
                            if let Some(n) = find_number_at(row_above, column_index - 1) {
                                sum += n;
                            }
                        }

                        // right-top
                        if can_go_right {
                            if let Some(n) = find_number_at(row_above, column_index + 1) {
                                sum += n;
                            }
                        }
                    }
                }

                if let Some(row_below) = row_below {
                    if let Some(n) = find_number_at(row_below, column_index) {
                        sum += n;
                    } else {
                        // left-bottom
                        if can_go_left {
                            if let Some(n) = find_number_at(row_below, column_index - 1) {
                                sum += n;
                            }
                        }

                        // right-bottom
                        if can_go_right {
                            if let Some(n) = find_number_at(row_below, column_index + 1) {
                                sum += n;
                            }
                        }
                    }
                }
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

        let parsed = parse_lines(&lines);

        multiply_gear_numbers(&parsed).into()
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
        use super::super::{parse_lines, Solution};
        use super::get_example;
        use crate::day_03::sum_all_part_numbers;
        use crate::shared::Day;

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
        use super::super::{parse_lines, Solution};
        use super::get_example;
        use crate::day_03::multiply_gear_numbers;
        use crate::shared::Day;

        #[test]
        fn outcome() {
            assert_eq!(79_026_871, (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            assert_eq!(467_835, multiply_gear_numbers(&parsed));
        }
    }
}
