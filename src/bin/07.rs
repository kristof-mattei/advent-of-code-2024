use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!(4_555_081_946_288_u64, 227_921_760_109_726_u64);

enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn execute(&self, left: u64, right: u64) -> u64 {
        match *self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            Operator::Concat => {
                let multiplier = multiplier(right);

                (left * multiplier) + right
            },
        }
    }
}

fn multiplier(v: u64) -> u64 {
    let mut power = 1;

    loop {
        if 10_u64.pow(power) > v {
            break;
        }

        power += 1;
    }

    10_u64.pow(power)
}

fn calculate_permutations(input: &str, operators: &[Operator]) -> PartSolution {
    let mut to_validate = vec![];
    for line in input.lines() {
        to_validate.push(parse(line));
    }

    let mut total = 0_u64;

    for (expected, operands) in to_validate {
        if try_all_permutations(expected, operands[0], &operands[1..], operators) > 0 {
            total += expected;
        }
    }

    total.into()
}

fn try_all_permutations(
    expected: u64,
    current: u64,
    operands: &[u64],
    operators: &[Operator],
) -> u64 {
    let mut permutations = 0;

    if operands.is_empty() {
        if expected == current {
            return 1;
        }

        return 0;
    }

    if current > expected {
        // we've overrun
        return 0;
    }

    for operator in operators {
        permutations += try_all_permutations(
            expected,
            operator.execute(current, operands[0]),
            &operands[1..],
            operators,
        );
    }

    permutations
}

fn parse(line: &str) -> (u64, Vec<u64>) {
    let (expected, operands) = line.split_once(':').expect("Bad input");
    let expected = expected.parse::<u64>().expect("Bad input");
    let operands = operands
        .trim()
        .split(' ')
        .map(|o| o.trim().parse::<u64>().expect("Bad input"))
        .collect::<Vec<u64>>();

    (expected, operands)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_permutations(input, &[Operator::Add, Operator::Multiply])
    }

    fn part_2(&self, input: &str) -> PartSolution {
        calculate_permutations(
            input,
            &[Operator::Add, Operator::Multiply, Operator::Concat],
        )
    }
}

#[cfg(test)]
mod test {

    mod part_1 {

        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                4_555_081_946_288_u64,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(3749, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::Parts as _;
        use advent_of_code_2024::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                227_921_760_109_726_u64,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(11387, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
