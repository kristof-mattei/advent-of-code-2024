use std::collections::BTreeMap;

use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!(6_359_213_660_505_u64, 6_381_624_803_796_u64);

struct File(u64, u64);

enum Node {
    File(File),
    Free(Vec<File>, u64),
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::File(File(fileno, n)) => {
                for _ in 0..*n {
                    write!(f, "{}", fileno)?;
                }

                Ok(())
            },
            Node::Free(vec, available) => {
                for File(fileno, n) in vec {
                    for _ in 0..*n {
                        write!(f, "{}", fileno)?;
                    }
                }

                for _ in 0..*available {
                    write!(f, ".")?;
                }

                Ok(())
            },
        }
    }
}

fn parse_input(input: &str) -> (BTreeMap<usize, u64>, Vec<Node>) {
    let mut free_space = BTreeMap::<usize, u64>::new();

    let filesystem = input
        .trim()
        .chars()
        .enumerate()
        .map(|(index, c)| {
            let number = c.to_digit(10).expect("Bad input").into();

            if index % 2 == 0 {
                // file
                Node::File(File(u64::try_from(index / 2).expect("Input OOB"), number))
            } else {
                assert!(free_space.insert(index, number).is_none());

                // free space
                Node::Free(vec![], number)
            }
        })
        .collect::<Vec<Node>>();

    (free_space, filesystem)
}

fn calculate_checksum(filesystem: &[Node]) -> u64 {
    let mut checksum: u64 = 0;

    let mut position = 0;

    for node in filesystem {
        match node {
            Node::File(File(fileno, size)) => {
                for i in position..(position + size) {
                    checksum += i * fileno;
                }

                position += size;
            },
            Node::Free(replacements, available) => {
                for File(fileno, size) in replacements {
                    for i in position..(position + size) {
                        checksum += i * fileno;
                    }

                    position += size;
                }

                position += available;
            },
        }
    }

    checksum
}

fn defragment(input: &str) -> PartSolution {
    let (_, mut filesystem) = parse_input(input);

    // first free block is at 1
    let mut free_block = 1;

    'outer: loop {
        match filesystem.pop() {
            Some(Node::Free(replacements, available)) => {
                // check if we've started to consume this block
                if replacements.is_empty() {
                    // we've not, drop it
                    continue;
                }

                filesystem.push(Node::Free(replacements, available));
                break;
            },
            Some(Node::File(File(fileno, mut size))) => {
                // we now insert as many as we can at the current `last_free_block` position, decreasing it's available, moving on when needed
                while size > 0 {
                    let Node::Free(replacements, available) = &mut filesystem[free_block] else {
                        panic!();
                    };

                    if *available == 0 {
                        // this one's full
                        if let Some(next_free_block) = filesystem
                            .iter()
                            .skip(free_block + 1)
                            .position(|b| matches!(b, Node::Free(_, _)))
                        {
                            free_block = next_free_block + free_block + 1;
                            continue;
                        }

                        // no more free blocks, put back current File at the end and end it
                        filesystem.push(Node::File(File(fileno, size)));
                        break 'outer;
                    }

                    let take = if *available < size { *available } else { size };

                    size -= take;
                    *available -= take;

                    replacements.push(File(fileno, take));
                }
            },
            None => {
                panic!("We should never have an empty file system")
            },
        }
    }

    calculate_checksum(&filesystem).into()
}

fn defragment_while_files(input: &str) -> PartSolution {
    let (mut free_space, mut filesystem) = parse_input(input);

    let mut file_index_from_right = filesystem.len() - 1;
    // !!!! We can move a file just once, meaning once we passed it, we don't need to re-process it
    loop {
        match filesystem.get(file_index_from_right) {
            Some(Node::Free(_, _)) => {},
            Some(&Node::File(File(fileno, file_size))) => {
                // find the lowest node that has space available
                if let Some((&free_index, _)) =
                    free_space.iter().find(|&(&index, &available_size)| {
                        available_size >= file_size && index < file_index_from_right
                    })
                {
                    assert!(free_space.remove(&free_index).is_some());

                    // at this index there is a free block that can fit our file
                    let Node::Free(replacements, available_size) = filesystem
                        .get_mut(free_index)
                        .expect("free space only refers to Free nodes")
                    else {
                        panic!();
                    };

                    replacements.push(File(fileno, file_size));

                    *available_size -= file_size;

                    if *available_size > 0 {
                        assert!(free_space.insert(free_index, *available_size).is_none());
                    }

                    // replace the current file with Free space as we moved it
                    filesystem[file_index_from_right] = Node::Free(vec![], file_size);
                }
            },
            None => {
                panic!("We should never have an empty file system")
            },
        }

        if file_index_from_right > 0 {
            file_index_from_right -= 1;
        } else {
            break;
        }
    }

    calculate_checksum(&filesystem).into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        defragment(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        defragment_while_files(input)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {

        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                6_359_213_660_505_u64,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(1928, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            let result = Solution {}.part_2(&read_file("inputs", &DAY));

            assert_eq!(6_381_624_803_796_u64, result);
        }

        #[test]
        fn example() {
            assert_eq!(2858, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
