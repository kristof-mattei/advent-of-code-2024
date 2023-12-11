use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(6956);

type Pipes = Vec<Vec<Pipe>>;

#[derive(PartialEq, Eq)]
enum Pipe {
    Pipe(PipePiece),
    Start(Option<PipePiece>),
}

#[derive(PartialEq, Eq)]
enum PipePiece {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
}

impl TryFrom<char> for Pipe {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::Pipe(PipePiece::Vertical)),
            '-' => Ok(Pipe::Pipe(PipePiece::Horizontal)),
            'L' => Ok(Pipe::Pipe(PipePiece::NorthEast)),
            'J' => Ok(Pipe::Pipe(PipePiece::NorthWest)),
            '7' => Ok(Pipe::Pipe(PipePiece::SouthWest)),
            'F' => Ok(Pipe::Pipe(PipePiece::SouthEast)),
            '.' => Ok(Pipe::Pipe(PipePiece::Ground)),
            'S' => Ok(Pipe::Start(None)),
            _ => Err("Invalid character"),
        }
    }
}

fn parse_lines(input: &str) -> Pipes {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<Pipe>>()
        })
        .collect::<Vec<Vec<Pipe>>>()
}

fn remap_start(map: &mut Vec<Vec<Pipe>>) -> (usize, usize) {
    for (i, line) in map.iter().enumerate() {
        for (j, column) in line.iter().enumerate() {
            if !matches!(column, Pipe::Start(_)) {
                continue;
            }

            let left = (j.checked_sub(1))
                .map(|jj| {
                    [
                        Pipe::Pipe(PipePiece::Horizontal),
                        Pipe::Pipe(PipePiece::NorthEast),
                        Pipe::Pipe(PipePiece::SouthEast),
                    ]
                    .contains(&line[jj])
                })
                .unwrap_or_default();

            let right = (j + 1 < line.len())
                .then(|| {
                    [
                        Pipe::Pipe(PipePiece::Horizontal),
                        Pipe::Pipe(PipePiece::NorthWest),
                        Pipe::Pipe(PipePiece::SouthWest),
                    ]
                    .contains(&line[j + 1])
                })
                .unwrap_or_default();

            let up = (i.checked_sub(1))
                .map(|ii| {
                    [
                        Pipe::Pipe(PipePiece::Vertical),
                        Pipe::Pipe(PipePiece::SouthEast),
                        Pipe::Pipe(PipePiece::SouthWest),
                    ]
                    .contains(&map[ii][j])
                })
                .unwrap_or_default();

            let down = (i + 1 < map.len())
                .then(|| {
                    [
                        Pipe::Pipe(PipePiece::Vertical),
                        Pipe::Pipe(PipePiece::NorthEast),
                        Pipe::Pipe(PipePiece::NorthWest),
                    ]
                    .contains(&map[i + 1][j])
                })
                .unwrap_or_default();

            let start = match (up, down, left, right) {
                // coming from top to bottom
                (true, true, false, false) => PipePiece::Vertical,

                // coming from top to left
                (true, false, true, false) => PipePiece::NorthWest,

                // top to right
                (true, false, false, true) => PipePiece::NorthEast,

                // coming from below to left
                (false, true, true, false) => PipePiece::SouthWest,

                // coming from below to right
                (false, true, false, true) => PipePiece::SouthEast,

                // coming from left to right
                (false, false, true, true) => PipePiece::Horizontal,
                _ => panic!("Invalid S"),
            };

            map[i][j] = Pipe::Start(Some(start));

            return (i, j);
        }
    }

    panic!("Start not found");
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn get_any_start_direction(map: &[Vec<Pipe>], start: &(usize, usize)) -> Direction {
    match &map[start.0][start.1] {
        Pipe::Pipe(o) | Pipe::Start(Some(o)) => match o {
            PipePiece::Vertical | PipePiece::SouthEast => Direction::North,
            PipePiece::Horizontal | PipePiece::SouthWest => Direction::East,
            PipePiece::NorthWest => Direction::South,
            PipePiece::NorthEast => Direction::West,
            PipePiece::Ground => panic!("We never start on ground"),
        },
        Pipe::Start(None) => panic!("Invalid coordinates"),
    }
}

fn next_direction(map: &[Vec<Pipe>], from: &Direction, start: &(usize, usize)) -> Direction {
    match &map[start.0][start.1] {
        Pipe::Pipe(o) | Pipe::Start(Some(o)) => match (from, o) {
            (Direction::North, PipePiece::Vertical)
            | (Direction::East, PipePiece::NorthWest)
            | (Direction::West, PipePiece::NorthEast) => Direction::North,
            (Direction::North, PipePiece::SouthEast)
            | (Direction::East, PipePiece::Horizontal)
            | (Direction::South, PipePiece::NorthEast) => Direction::East,
            (Direction::North, PipePiece::SouthWest)
            | (Direction::South, PipePiece::NorthWest)
            | (Direction::West, PipePiece::Horizontal) => Direction::West,
            (Direction::East, PipePiece::SouthWest)
            | (Direction::South, PipePiece::Vertical)
            | (Direction::West, PipePiece::SouthEast) => Direction::South,
            _ => panic!("Invalid directions"),
        },
        Pipe::Start(None) => panic!("Invalid coordinates"),
    }
}

fn next_coordinates(
    _map: &[Vec<Pipe>],
    next: &Direction,
    (current_row_index, current_column_index): &(usize, usize),
) -> (usize, usize) {
    match next {
        Direction::North => (current_row_index - 1, *current_column_index),
        Direction::East => (*current_row_index, current_column_index + 1),
        Direction::South => (current_row_index + 1, *current_column_index),
        Direction::West => (*current_row_index, current_column_index - 1),
    }
}

fn find_furthest(map: &[Vec<Pipe>], start: (usize, usize)) -> usize {
    let mut from = get_any_start_direction(map, &start);

    let mut current = start;

    let mut steps = 0;

    loop {
        let next = next_direction(map, &from, &current);

        let next_coordinates = next_coordinates(map, &next, &current);

        steps += 1;

        current = next_coordinates;

        from = next;

        if current == start {
            break;
        }
    }

    steps / 2
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let mut map = parse_lines(input);

        let start = remap_start(&mut map);

        find_furthest(&map, start).into()
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        None.into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::{
            solution::{read_file, read_file_part},
            Parts,
        };

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(6956, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                4,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                4,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                8,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 3))
            );
        }

        #[test]
        fn example_4() {
            assert_eq!(
                8,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 4))
            );
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::{solution::read_file, PartSolution, Parts};

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
