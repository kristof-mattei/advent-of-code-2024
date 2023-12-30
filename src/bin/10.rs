use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(6956, 455);

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Tile {
    part_of_loop: bool,
    pattern: Pattern,
}

#[derive(Copy, Clone)]
enum Pattern {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
}

impl Tile {
    fn from_board_value(value: char, start_piece: Pattern) -> Result<Tile, &'static str> {
        match value {
            '|' => Ok(Tile {
                part_of_loop: false,
                pattern: Pattern::Vertical,
            }),
            '-' => Ok(Tile {
                part_of_loop: false,
                pattern: Pattern::Horizontal,
            }),
            'L' => Ok(Tile {
                part_of_loop: false,
                pattern: Pattern::NorthEast,
            }),
            'J' => Ok(Tile {
                part_of_loop: false,
                pattern: Pattern::NorthWest,
            }),
            '7' => Ok(Tile {
                part_of_loop: false,
                pattern: Pattern::SouthWest,
            }),
            'F' => Ok(Tile {
                part_of_loop: false,
                pattern: Pattern::SouthEast,
            }),
            '.' => Ok(Tile {
                part_of_loop: false,
                pattern: Pattern::Ground,
            }),
            'S' => Ok(Tile {
                part_of_loop: true,
                pattern: start_piece,
            }),
            _ => Err("Invalid character"),
        }
    }
}

fn split_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>()
}

fn parse_lines(lines: Vec<Vec<char>>, start_piece: Pattern) -> Vec<Vec<Tile>> {
    lines
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|c| Tile::from_board_value(c, start_piece).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_start_piece(map: &Vec<Vec<char>>) -> ((usize, usize), Pattern) {
    for (i, line) in map.iter().enumerate() {
        for (j, column) in line.iter().enumerate() {
            if column != &'S' {
                continue;
            }

            let left = (j.checked_sub(1))
                .map(|jj| ['-', 'L', 'F'].contains(&line[jj]))
                .unwrap_or_default();

            let right = (j + 1 < line.len())
                .then(|| ['-', 'J', '7'].contains(&line[j + 1]))
                .unwrap_or_default();

            let up = (i.checked_sub(1))
                .map(|ii| ['|', 'F', '7'].contains(&map[ii][j]))
                .unwrap_or_default();

            let down = (i + 1 < map.len())
                .then(|| ['|', 'L', 'J'].contains(&map[i + 1][j]))
                .unwrap_or_default();

            let start = match (up, down, left, right) {
                // coming from top to bottom
                (true, true, false, false) => Pattern::Vertical,

                // coming from top to left
                (true, false, true, false) => Pattern::NorthWest,

                // top to right
                (true, false, false, true) => Pattern::NorthEast,

                // coming from below to left
                (false, true, true, false) => Pattern::SouthWest,

                // coming from below to right
                (false, true, false, true) => Pattern::SouthEast,

                // coming from left to right
                (false, false, true, true) => Pattern::Horizontal,
                _ => panic!("Invalid S"),
            };

            return ((i, j), start);
        }
    }

    panic!("Start not found");
}

fn get_any_start_direction(map: &[Vec<Tile>], start: &(usize, usize)) -> Direction {
    let tile = &map[start.0][start.1];

    assert!(tile.part_of_loop, "Start tile should be part of the loop");

    match &tile.pattern {
        Pattern::Vertical | Pattern::SouthEast => Direction::Up,
        Pattern::Horizontal | Pattern::SouthWest => Direction::Right,
        Pattern::NorthWest => Direction::Down,
        Pattern::NorthEast => Direction::Left,
        Pattern::Ground => panic!("We never start on ground"),
    }
}

fn next_direction(map: &[Vec<Tile>], from: &Direction, start: &(usize, usize)) -> Direction {
    let current = &map[start.0][start.1].pattern;

    match (from, current) {
        (Direction::Up, Pattern::Vertical)
        | (Direction::Right, Pattern::NorthWest)
        | (Direction::Left, Pattern::NorthEast) => Direction::Up,
        (Direction::Up, Pattern::SouthEast)
        | (Direction::Right, Pattern::Horizontal)
        | (Direction::Down, Pattern::NorthEast) => Direction::Right,
        (Direction::Up, Pattern::SouthWest)
        | (Direction::Down, Pattern::NorthWest)
        | (Direction::Left, Pattern::Horizontal) => Direction::Left,
        (Direction::Right, Pattern::SouthWest)
        | (Direction::Down, Pattern::Vertical)
        | (Direction::Left, Pattern::SouthEast) => Direction::Down,
        _ => panic!("Invalid directions"),
    }
}

fn next_coordinates(
    next: &Direction,
    (current_row_index, current_column_index): &(usize, usize),
) -> (usize, usize) {
    match next {
        Direction::Up => (current_row_index - 1, *current_column_index),
        Direction::Right => (*current_row_index, current_column_index + 1),
        Direction::Down => (current_row_index + 1, *current_column_index),
        Direction::Left => (*current_row_index, current_column_index - 1),
    }
}

fn mark_coordinates_as_part_of_loop(map: &mut [Vec<Tile>], coordinates: (usize, usize)) {
    let tile = &mut map[coordinates.0][coordinates.1];

    if matches!(tile.pattern, Pattern::Ground) {
        panic!("Ground cannot be part of loop");
    }

    tile.part_of_loop = true;
}

fn mark_loop(map: &mut [Vec<Tile>], start: (usize, usize)) -> (Vec<(usize, usize)>, usize) {
    let mut from = get_any_start_direction(map, &start);

    let mut current = start;

    let mut steps = 1;

    let mut coordinates = vec![];

    mark_coordinates_as_part_of_loop(map, start);

    loop {
        let next = next_direction(map, &from, &current);

        if next != from {
            coordinates.push(current);
        }

        let next_coordinates = next_coordinates(&next, &current);

        mark_coordinates_as_part_of_loop(map, next_coordinates);

        steps += 1;

        current = next_coordinates;

        from = next;

        if current == start {
            coordinates.push(current);
            break;
        }
    }

    (coordinates, steps)
}

pub(crate) fn shoelace(coordinates: &[(usize, usize)], perimeter: usize) -> usize {
    let mut size: isize = 0;

    for coord in coordinates.windows(2) {
        // [ a b ]
        // [ c d ]

        let a: isize = coord[0].0.try_into().unwrap();
        let b: isize = coord[0].1.try_into().unwrap();
        let c: isize = coord[1].0.try_into().unwrap();
        let d: isize = coord[1].1.try_into().unwrap();

        size += (a * d) - (b * c);
    }

    TryInto::<usize>::try_into((size / 2).abs()).unwrap() - (perimeter / 2) + 1
}

fn count_enclosed(map: &mut [Vec<Tile>], start: (usize, usize)) -> usize {
    let (coordinates, perimeter) = mark_loop(map, start);

    shoelace(&coordinates, perimeter)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let lines = split_input(input);

        let (start, start_piece) = find_start_piece(&lines);

        let mut map = parse_lines(lines, start_piece);

        let (_, steps) = mark_loop(&mut map, start);

        ((steps) / 2).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let lines = split_input(input);

        let (start, start_piece) = find_start_piece(&lines);

        let mut map = parse_lines(lines, start_piece);

        count_enclosed(&mut map, start).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::solution::{read_file, read_file_part};
        use advent_of_code_2023::shared::Parts;

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

        use advent_of_code_2023::shared::solution::{read_file, read_file_part};
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(455, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                4,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 5))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                8,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 6))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                10,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 7))
            );
        }
    }
}
