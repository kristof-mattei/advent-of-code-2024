use std::fmt::Debug;
use std::ops::Range;

use self::parse::{IndividualSeeds, RangeOfSeeds};
use crate::shared::{Day, PartSolution};

mod parse;

#[derive(Debug, PartialEq, Eq)]
struct AlmanacMap(Vec<DestinationSourceRange>);

impl AlmanacMap {
    fn map(&self, seed: u64) -> u64 {
        for range in &self.0 {
            if let Some(mapped_seed) = range.map(seed) {
                return mapped_seed;
            }
        }

        seed
    }

    fn remap_seed_ranges(&self, seed_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        // run the seed_range through the mapping ONCE, splitting it into processed and unprocessed
        let mut processed = vec![]; // these have been split off and remapped

        let mut todo = seed_ranges;

        while let Some(seed_range) = todo.pop() {
            let mut processed_inner = vec![];
            let mut todo_inner = vec![];

            for dsr in &self.0 {
                let map_source = dsr.1..(dsr.1 + dsr.2);

                if seed_range.end <= map_source.start || seed_range.start >= map_source.end {
                    // no overlap
                } else if seed_range.start < map_source.start {
                    if seed_range.end <= map_source.end {
                        // seed_range: |___________|
                        //        map:       |__________|
                        //             S1    M1    S2    M2
                        // S1..M1
                        // M1..S2 (remap)
                        // or
                        // seed_range: |___________|
                        //        map:       |_____|
                        //             S1    M1    S2,M2
                        // S1..M1
                        todo_inner.push(seed_range.start..map_source.start);
                        // M1..S2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(map_source.start)
                                ..dsr.map_to_destination(seed_range.end),
                        );
                    } else {
                        // seed_range: |_________________|
                        //        map:       |_____|
                        //             S1    M1    M2    S2
                        // S1..M1
                        todo_inner.push(seed_range.start..map_source.start);
                        // M1..M2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(map_source.start)
                                ..dsr.map_to_destination(map_source.end),
                        );
                        // M2..S2
                        todo_inner.push(map_source.end..seed_range.end);
                    }
                } else if seed_range.start > map_source.start {
                    if seed_range.end <= map_source.end {
                        // seed_range:       |_____|
                        //        map: |_________________|
                        //             M1    S1    S2    M2
                        // S1..S2 (remap)
                        // or
                        // seed_range:       |_____|
                        //        map: |___________|
                        //             M1    S1    S2,M2
                        // S1..S2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(seed_range.start)
                                ..dsr.map_to_destination(seed_range.end),
                        );
                    } else {
                        // seed_range:       |___________|
                        //        map: |___________|
                        //             M1    S1    M2    S2
                        // S1..M2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(seed_range.start)
                                ..dsr.map_to_destination(map_source.end),
                        );
                        // M2..S2
                        todo_inner.push(map_source.end..seed_range.end);
                    }
                } else if seed_range.start == map_source.start {
                    if seed_range.end <= map_source.end {
                        // seed_range: |_____|
                        //        map: |___________|
                        //             S1,M1 S2    M2
                        // S1..S2 (remap)
                        // or
                        // seed_range: |_____|
                        //        map: |_____|
                        //             S1,M1 S2,M2
                        // S1..S2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(map_source.start)
                                ..dsr.map_to_destination(seed_range.end),
                        );
                    } else {
                        // seed_range: |___________|
                        //        map: |_____|
                        //             S1,M1 M2    S2
                        // M1..M2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(map_source.start)
                                ..dsr.map_to_destination(map_source.end),
                        );
                        // M2..S2
                        todo_inner.push(map_source.end..seed_range.end);
                    }
                } else {
                    // nothing happened
                    panic!("Uncovered case");
                }

                if !processed_inner.is_empty() {
                    break;
                }
            }

            if processed_inner.is_empty() {
                // we got through all the maps without processing anything, so the range is fully processed
                processed.push(seed_range);
            } else {
                // we got a hit on this map
                // we bail to avoid duplicate non-mapped pieces

                // processed pieces, retain them
                processed.append(&mut processed_inner);

                // enqueue the cut of pieces for re-processing
                todo.append(&mut todo_inner);
            }
        }

        processed
    }
}

impl From<Vec<DestinationSourceRange>> for AlmanacMap {
    fn from(mut vec: Vec<DestinationSourceRange>) -> Self {
        vec.sort_by_key(|r| r.1);
        AlmanacMap(vec)
    }
}

#[derive(PartialEq, Eq)]
struct DestinationSourceRange(u64, u64, u64);

impl DestinationSourceRange {
    fn map(&self, seed: u64) -> Option<u64> {
        if (self.1..self.1 + self.2).contains(&seed) {
            Some(self.map_to_destination(seed))
        } else {
            None
        }
    }

    fn map_to_destination(&self, seed: u64) -> u64 {
        seed + self.0 - self.1
    }
}

impl std::fmt::Debug for DestinationSourceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestinationSourceRange")
            .field("Destination", &((self.0)..(self.0 + self.2)))
            .field("Source", &((self.1)..(self.1 + self.2)))
            .field("Range", &self.2)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Maps(Vec<AlmanacMap>);

impl From<Vec<AlmanacMap>> for Maps {
    fn from(vec: Vec<AlmanacMap>) -> Self {
        Maps(vec)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    maps: Maps,
}

impl Almanac {
    fn all_the_way(&self, mut seed: u64) -> u64 {
        for map in &self.maps.0 {
            seed = map.map(seed);
        }

        seed
    }

    fn remap_seed_ranges_all_the_way(&self, mut seed_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        for map in &self.maps.0 {
            seed_ranges = map.remap_seed_ranges(seed_ranges);
        }

        seed_ranges
    }
}

pub struct Solution {}

impl Day for Solution {
    fn get_input(&self) -> &str {
        include_str!("input.txt")
    }

    fn get_example(&self) -> &str {
        include_str!("example.txt")
    }

    fn part_1(&self, input: &str) -> PartSolution {
        let (almanac, seeds) = parse::parse_lines::<IndividualSeeds>(input);

        let min_location = seeds
            .0
            .into_iter()
            .map(|s| almanac.all_the_way(s))
            .min()
            .expect("No min found");

        (min_location).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (almanac, mut range_of_seeds) = parse::parse_lines::<RangeOfSeeds>(input);

        let mut result_seed_ranges =
            almanac.remap_seed_ranges_all_the_way(range_of_seeds.0.clone());

        range_of_seeds.0.sort_by_key(|r| r.start);
        result_seed_ranges.sort_by_key(|r| r.start);

        let mut min = u64::MAX;

        'outer: for seed_range in result_seed_ranges {
            for original_seed_range in &range_of_seeds.0 {
                if original_seed_range.contains(&(seed_range.start - 1)) {
                    min = seed_range.start;
                    break 'outer;
                }

                if original_seed_range.contains(&(seed_range.end - 1)) {
                    min = original_seed_range.start;
                    break 'outer;
                }
            }
        }

        min.into()
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use parse::parse_lines;

        use crate::day_05::{parse, Almanac, DestinationSourceRange, IndividualSeeds, Solution};
        use crate::shared::Day;

        #[test]
        fn outcome() {
            let s = Solution {};
            assert_eq!(309_796_150, s.part_1(s.get_input()));
        }

        #[test]
        fn example() {
            let lines = Solution {}.get_example();

            let (almanac, seeds) = parse_lines::<IndividualSeeds>(lines);

            assert_eq!(IndividualSeeds(vec![79, 14, 55, 13]), seeds);

            assert_eq!(
                almanac,
                Almanac {
                    maps: vec![
                        vec![
                            DestinationSourceRange(50, 98, 2),
                            DestinationSourceRange(52, 50, 48)
                        ]
                        .into(),
                        vec![
                            DestinationSourceRange(0, 15, 37),
                            DestinationSourceRange(37, 52, 2),
                            DestinationSourceRange(39, 0, 15)
                        ]
                        .into(),
                        vec![
                            DestinationSourceRange(49, 53, 8),
                            DestinationSourceRange(0, 11, 42),
                            DestinationSourceRange(42, 0, 7),
                            DestinationSourceRange(57, 7, 4)
                        ]
                        .into(),
                        vec![
                            DestinationSourceRange(88, 18, 7),
                            DestinationSourceRange(18, 25, 70)
                        ]
                        .into(),
                        vec![
                            DestinationSourceRange(45, 77, 23),
                            DestinationSourceRange(81, 45, 19),
                            DestinationSourceRange(68, 64, 13)
                        ]
                        .into(),
                        vec![
                            DestinationSourceRange(0, 69, 1),
                            DestinationSourceRange(1, 0, 69)
                        ]
                        .into(),
                        vec![
                            DestinationSourceRange(60, 56, 37),
                            DestinationSourceRange(56, 93, 4)
                        ]
                        .into()
                    ]
                    .into(),
                },
            );
        }

        #[test]
        fn example_seed_to_soil() {
            let lines = Solution {}.get_example();

            let (almanac, _) = parse_lines::<IndividualSeeds>(lines);

            assert_eq!(81, almanac.maps.0[0].map(79));
            assert_eq!(14, almanac.maps.0[0].map(14));
            assert_eq!(57, almanac.maps.0[0].map(55));
            assert_eq!(13, almanac.maps.0[0].map(13));
        }

        #[test]
        fn example_seed_to_location() {
            let lines = Solution {}.get_example();

            let (almanac, _) = parse_lines::<IndividualSeeds>(lines);

            assert_eq!(81, almanac.maps.0[0].map(79));
            assert_eq!(81, almanac.maps.0[1].map(81));
            assert_eq!(81, almanac.maps.0[2].map(81));
            assert_eq!(74, almanac.maps.0[3].map(81));
            assert_eq!(78, almanac.maps.0[4].map(74));
            assert_eq!(78, almanac.maps.0[5].map(78));
            assert_eq!(82, almanac.maps.0[6].map(78));
        }

        #[test]
        fn seed_maps() {
            let lines = Solution {}.get_example();

            let (almanac, seeds) = parse_lines::<IndividualSeeds>(lines);

            let locations = seeds
                .0
                .into_iter()
                .map(|s| almanac.all_the_way(s))
                .collect::<Vec<u64>>();

            assert_eq!(vec![82, 43, 86, 35], locations);
        }
    }

    mod part_2 {
        use std::ops::Range;

        use super::super::Solution;
        use crate::day_05::parse::parse_lines;
        use crate::day_05::{AlmanacMap, DestinationSourceRange, RangeOfSeeds};
        use crate::shared::Day;

        #[test]
        fn outcome() {
            assert_eq!(50_716_416, (Solution {}).part_2_with_input());
        }

        #[test]
        fn example_test_ranges() {
            let lines = Solution {}.get_example();

            let (_, range_of_seeds) = parse_lines::<RangeOfSeeds>(lines);

            assert_eq!(vec![79..93, 55..68], range_of_seeds.0);
        }

        #[test]
        fn example_test_min_ranges_naive() {
            let lines = Solution {}.get_example();

            let (almanac, range_of_seeds) = parse_lines::<RangeOfSeeds>(lines);

            let location = range_of_seeds
                .0
                .into_iter()
                .filter_map(|s| s.clone().map(|s| almanac.all_the_way(s)).min())
                .min()
                .expect("No min found");

            assert_eq!(location, 46);
        }

        #[test]
        fn example_test_overlap() {
            let map = AlmanacMap(vec![DestinationSourceRange(0, 10, 5)]);

            let remapped = map.remap_seed_ranges(vec![Range { start: 5, end: 15 }]);

            assert_eq!(remapped, vec![0..5, 5..10]);
        }

        #[test]
        fn example_test_overlap_multiple() {
            let map = AlmanacMap(vec![
                DestinationSourceRange(0, 10, 5),
                DestinationSourceRange(20, 5, 5),
            ]);

            let remapped = map.remap_seed_ranges(vec![Range { start: 5, end: 15 }]);

            assert_eq!(remapped, vec![0..5, 20..25]);
        }

        #[test]
        fn example_lowest_in_original() {
            assert_eq!(55, Solution {}.part_2_with_example());
        }
    }
}
