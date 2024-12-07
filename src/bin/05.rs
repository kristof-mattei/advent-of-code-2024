use std::collections::{BTreeMap, BTreeSet};

use advent_of_code_2024::shared::{PartSolution, Parts};

advent_of_code_2024::solution!(4281);

fn validate_page_updates(input: &str) -> PartSolution {
    let ParseInputResult {
        pages_before,
        pages_after,
        all_updates,
    } = parse_input(input);

    let mut count = 0;

    for updates in all_updates {
        let mut invalid = false;

        let mut before_ = BTreeSet::new();
        let mut after_ = updates.iter().copied().collect::<BTreeSet<_>>();

        for page in &updates {
            after_.remove(page);

            dbg!(&after_);
            dbg!(&before_);

            dbg!(&pages_after.get(page));
            dbg!(&pages_before.get(page));

            let after_disjoint = pages_after
                .get(page)
                .map_or(true, |a| a.is_disjoint(&before_));

            let before_disjoint = pages_before
                .get(page)
                .map_or(true, |a| a.is_disjoint(&after_));

            if after_disjoint && before_disjoint {
                // good
            } else {
                invalid = true;
                break;
            }

            before_.insert(*page);
        }

        if invalid {
            println!("{:?} is invalid", updates);
        } else {
            println!("{:?} is valid", updates);
            count += updates[updates.len() / 2];
        }
    }

    PartSolution::U32(count)
}

fn fix_invalid_page_updates(input: &str) -> PartSolution {
    let ParseInputResult {
        pages_before,
        pages_after,
        all_updates,
    } = parse_input(input);

    let mut count = 0;

    for updates in all_updates {
        let mut invalid = false;

        let mut before = BTreeSet::new();
        let mut after = updates.iter().copied().collect::<BTreeSet<_>>();

        for page in &updates {
            after.remove(page);

            let after_disjoint = pages_after
                .get(page)
                .map_or(true, |a| a.is_disjoint(&before));

            let before_disjoint = pages_before
                .get(page)
                .map_or(true, |a| a.is_disjoint(&after));

            if after_disjoint && before_disjoint {
                // good
            } else {
                invalid = true;
                break;
            }

            before.insert(*page);
        }

        if invalid {
            // fixup
            let updated = fixup(updates, &pages_before, &pages_after);

            count += updated[updated.len() / 2];
        }
    }

    PartSolution::U32(count)
}

fn fixup(
    mut original: Vec<u32>,
    pages_before: &BTreeMap<u32, BTreeSet<u32>>,
    pages_after: &BTreeMap<u32, BTreeSet<u32>>,
) -> Vec<u32> {
    let mut updated = Vec::with_capacity(original.len());

    let mut before = BTreeSet::new();
    let mut after = original.iter().copied().collect::<BTreeSet<_>>();

    while !original.is_empty() {
        let position = original
            .iter()
            .position(|page| {
                let mut after = after.clone();
                after.remove(page);

                let all_before_match = pages_before.get(page).map_or(true, |a| before.is_subset(a));

                let all_after_match = pages_after.get(page).map_or(true, |a| after.is_subset(a));

                let before_disjoint = pages_before
                    .get(page)
                    .map_or(true, |a| a.is_disjoint(&after));

                let after_disjoint = pages_after
                    .get(page)
                    .map_or(true, |a| a.is_disjoint(&before));

                before_disjoint && after_disjoint && all_before_match && all_after_match
            })
            .expect("There should be one");

        let next = original.remove(position);
        updated.push(next);
        before.insert(next);
        after.remove(&next);
    }

    updated
}

struct ParseInputResult {
    pages_before: BTreeMap<u32, BTreeSet<u32>>,
    pages_after: BTreeMap<u32, BTreeSet<u32>>,
    all_updates: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> ParseInputResult {
    let mut after_empty_line = false;

    let mut pages_before = BTreeMap::<u32, BTreeSet<u32>>::new();
    let mut pages_after = BTreeMap::<u32, BTreeSet<u32>>::new();

    let mut all_updates = vec![];

    for line in input.lines() {
        if line.is_empty() {
            after_empty_line = true;
            continue;
        }

        if after_empty_line {
            all_updates.push(
                line.split(',')
                    .map(|piece| piece.parse::<u32>().expect("Bad input"))
                    .collect::<Vec<u32>>(),
            );
        } else {
            let (before, after) = line
                .split_once('|')
                .map(|(l, r)| {
                    (
                        l.parse::<u32>().expect("Bad input"),
                        r.parse::<u32>().expect("Bad input"),
                    )
                })
                .expect("Bad input");

            pages_before
                .entry(after)
                .and_modify(|s| {
                    s.insert(before);
                })
                .or_insert_with(|| [before].into());

            pages_after
                .entry(before)
                .and_modify(|s| {
                    s.insert(after);
                })
                .or_insert_with(|| [after].into());
        }
    }

    ParseInputResult {
        pages_before,
        pages_after,
        all_updates,
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        validate_page_updates(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        fix_invalid_page_updates(input)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use std::collections::BTreeSet;

        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(4281, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(143, (Solution {}).part_1(&read_file("examples", &DAY)));
        }

        #[test]
        fn is_disjoint() {
            assert!(BTreeSet::<u32>::new().is_disjoint(&BTreeSet::<u32>::new()));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(5466, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(123, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
