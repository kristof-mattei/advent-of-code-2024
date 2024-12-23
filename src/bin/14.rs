use advent_of_code_2024::shared::{PartSolution, Parts};
use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code_2024::solution!(214_400_550, 8149);

static ROBOT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl std::fmt::Debug for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "p={},{} v={},{}",
            self.position.0, self.position.1, self.velocity.0, self.velocity.1
        )
    }
}

fn parse_robot(robot: &str) -> Robot {
    let button_captures = ROBOT_REGEX.captures(robot).unwrap();

    let robot_position_x = button_captures
        .get(1)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .unwrap();
    let robot_position_y = button_captures
        .get(2)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .unwrap();
    let velocity_x = button_captures
        .get(3)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .unwrap();
    let velocity_y = button_captures
        .get(4)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .unwrap();

    Robot {
        position: (robot_position_x, robot_position_y),
        velocity: (velocity_x, velocity_y),
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();

    for line in input.trim().lines() {
        robots.push(parse_robot(line));
    }

    robots
}

fn calculate_safety_factor(input: &str, width: isize, height: isize) -> PartSolution {
    let mut robots = parse_input(input);

    for _second in 0..100 {
        for Robot { position, velocity } in &mut robots {
            position.0 += velocity.0;
            position.0 = position.0.rem_euclid(width);
            position.1 += velocity.1;
            position.1 = position.1.rem_euclid(height);
        }
    }

    let (q1, q2, q3, q4) = count_robots_in_quadrants(&robots, width, height);

    (q1 * q2 * q3 * q4).into()
}

fn find_with_lowest_variance(input: &str) -> PartSolution {
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;

    let mut robots = parse_input(input);

    let mut variances = Vec::new();

    for _ in 1..=WIDTH * HEIGHT + 1 {
        for Robot { position, velocity } in &mut robots {
            position.0 += velocity.0;
            position.0 = position.0.rem_euclid(WIDTH);
            position.1 += velocity.1;
            position.1 = position.1.rem_euclid(HEIGHT);
        }

        variances.push(variance(&robots));
    }

    let try_number_with_variance: Vec<_> = variances
        .iter()
        .enumerate()
        .map(|(r#try, &variance)| (r#try + 1, variance))
        .collect();

    let min = try_number_with_variance
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    min.0.into()
}

#[expect(clippy::cast_precision_loss)]
fn variance(robots: &[Robot]) -> f64 {
    let positions: Vec<_> = robots
        .iter()
        .map(|robot| (robot.position.0 + robot.position.1) as f64)
        .collect();

    let mean = positions.iter().sum::<f64>() / positions.len() as f64;

    let diffs: Vec<f64> = positions
        .iter()
        .map(|&p| {
            let diff = p - mean;
            diff * diff
        })
        .collect();

    diffs.iter().sum::<f64>() / diffs.len() as f64
}

fn count_robots_in_quadrants(
    robots: &[Robot],
    width: isize,
    height: isize,
) -> (usize, usize, usize, usize) {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    // ordinal, defining index at which we ignore numbers, NOT the amount per half per dimension
    let width_separator = width / 2;
    let height_separator = height / 2;

    for Robot { position, .. } in robots {
        if position.0 == width_separator || position.1 == height_separator {
            continue;
        }

        let is_left = position.0 < width_separator;
        let is_top = position.1 < height_separator;

        match (is_top, is_left) {
            (true, false) => q1 += 1,
            (true, true) => q2 += 1,
            (false, true) => q3 += 1,
            (false, false) => q4 += 1,
        }
    }

    (q1, q2, q3, q4)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_safety_factor(input, 101, 103)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        find_with_lowest_variance(input)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{calculate_safety_factor, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                214_400_550,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                12,
                calculate_safety_factor(&read_file("examples", &DAY), 11, 7)
            );
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(8149, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }
    }
}
