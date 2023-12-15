use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(30575, 37478);

#[derive(PartialEq, Eq)]
enum What {
    Ash,
    Rock,
}

impl TryFrom<char> for What {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(What::Rock),
            '.' => Ok(What::Ash),
            _ => Err("What?"),
        }
    }
}

impl std::fmt::Display for What {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            What::Ash => '.',
            What::Rock => '#',
        };

        write!(f, "{}", c)
    }
}

fn parse_input(input: &str) -> Vec<Vec<Vec<What>>> {
    let mut all = vec![];

    for group in input
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
    {
        all.push(
            group
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| c.try_into().unwrap())
                        .collect::<Vec<What>>()
                })
                .collect::<Vec<_>>(),
        );
    }

    all
}

#[derive(PartialEq, Eq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

fn compare_rows_r(pattern: &[Vec<What>], index1: usize, index2: usize) -> bool {
    // are they all equal?
    if pattern[index1]
        .iter()
        .zip(pattern[index2].iter())
        .all(|(l, r)| l == r)
    {
        // are we at the edge?
        if index1 == 0 || index2 + 1 == pattern.len() {
            return true;
        }

        return compare_rows_r(pattern, index1 - 1, index2 + 1);
    }

    false
}

fn compare_columns_r(pattern: &[Vec<What>], index1: usize, index2: usize) -> bool {
    // are they all equal?
    if pattern
        .iter()
        .map(|row| &row[index1])
        .zip(pattern.iter().map(|row| &row[index2]))
        .all(|(l, r)| l == r)
    {
        // are we at the edge?
        if index1 == 0 || index2 + 1 == pattern[0].len() {
            return true;
        }

        return compare_columns_r(pattern, index1 - 1, index2 + 1);
    }

    false
}

fn find_reflection(pattern: &[Vec<What>]) -> Reflection {
    // horizontal
    for row_index in 0..pattern.len() - 1 {
        if compare_rows_r(pattern, row_index, row_index + 1) {
            return Reflection::Horizontal(row_index + 1);
        }
    }

    // vertical
    for column_index in 0..pattern[0].len() - 1 {
        if compare_columns_r(pattern, column_index, column_index + 1) {
            return Reflection::Vertical(column_index + 1);
        }
    }

    panic!("Shouldn't get here")
}

fn find_reflection_but_not(
    pattern: &[Vec<What>],
    the_other_reflection: &Reflection,
) -> Option<Reflection> {
    // horizontal
    for row_index in 0..pattern.len() - 1 {
        if compare_rows_r(pattern, row_index, row_index + 1) {
            let r = Reflection::Horizontal(row_index + 1);
            if &r != the_other_reflection {
                return Some(r);
            }
        }
    }

    // vertical
    for column_index in 0..pattern[0].len() - 1 {
        if compare_columns_r(pattern, column_index, column_index + 1) {
            let r = Reflection::Vertical(column_index + 1);
            if &r != the_other_reflection {
                return Some(r);
            }
        }
    }

    None
}

fn add_reflections(patterns: &[Vec<Vec<What>>]) -> usize {
    let mut total = 0;

    for pattern in patterns {
        total += match find_reflection(pattern) {
            Reflection::Horizontal(v) => v * 100,
            Reflection::Vertical(v) => v,
        };
    }

    total
}

fn swap(what: &mut What) {
    *what = match what {
        What::Ash => What::Rock,
        What::Rock => What::Ash,
    };
}

fn add_new_reflections(mut patterns: Vec<Vec<Vec<What>>>) -> usize {
    let mut total = 0;

    for pattern in &mut patterns {
        // find old one
        let old_one = find_reflection(pattern);

        'outer: for r in 0..pattern.len() {
            for c in 0..pattern[r].len() {
                swap(&mut pattern[r][c]);

                let x = find_reflection_but_not(pattern, &old_one);

                if let Some(x) = x {
                    total += match x {
                        Reflection::Horizontal(v) => v * 100,
                        Reflection::Vertical(v) => v,
                    };

                    break 'outer;
                }

                swap(&mut pattern[r][c]);
            }
        }
    }

    total
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        add_reflections(&parsed).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        add_new_reflections(parsed).into()
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
            assert_eq!(30575, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(405, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(37478, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(400, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
