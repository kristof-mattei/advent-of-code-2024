use std::{env, fs};

use super::day::Day;

/// Helper function that reads a text file to a string.
/// # Panics
///
/// if the file does not exist or cannot be read
#[must_use]
pub fn read_file(folder: &str, day: Day) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
/// # Panics
///
/// if the file does not exist or cannot be read
#[must_use]
pub fn read_file_part(folder: &str, day: Day, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{day}-{part}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        /// The current day.
        const DAY: $crate::shared::day::Day = $crate::day!($day);

        #[allow(dead_code)]
        fn main() {
            println!("{}", file!());
            // use advent_of_code::template::runner::*;

            let input = $crate::shared::solution::read_file("inputs", DAY);
            let s = Solution {};

            println!("{:?}", s.part_1(&input));
            println!("{:?}", s.part_2(&input));
        }
    };
    ($day:expr, $solution_1:expr) => {
        /// The current day.
        const DAY: $crate::shared::day::Day = $crate::day!($day);

        #[allow(dead_code)]
        fn main() {
            // use advent_of_code::template::runner::*;

            let input = $crate::shared::solution::read_file("inputs", DAY);
            let s = Solution {};

            assert_eq!($solution_1, s.part_1(&input));
            // run_part(part_one, &input, DAY, 1);
            // run_part(part_two, &input, DAY, 2);
        }
    };
    ($day:expr, $solution_1:expr, $solution_2:expr) => {
        /// The current day.
        const DAY: $crate::shared::day::Day = $crate::day!($day);

        #[allow(dead_code)]
        fn main() {
            // use advent_of_code::template::runner::*;

            println!("{}", file!());
            let input = $crate::shared::solution::read_file("inputs", DAY);
            let s = Solution {};

            assert_eq!($solution_1, s.part_1(&input));
            assert_eq!($solution_2, s.part_2(&input));
            // run_part(part_one, &input, DAY, 1);
            // run_part(part_two, &input, DAY, 2);
        }
    };
}
