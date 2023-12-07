use advent_of_code_2023::shared::{Day, PartSolution};

advent_of_code_2023::solution!(7);

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self, _input: &str) -> PartSolution {
        PartSolution::None
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        PartSolution::None
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use advent_of_code_2023::shared::{solution::read_file, Day, PartSolution};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_1(&read_file("inputs", DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_1(&read_file("examples", DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::{solution::read_file, Day, PartSolution};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("inputs", DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("examples", DAY))
            );
        }
    }
}
