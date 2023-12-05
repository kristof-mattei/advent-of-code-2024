use std::fmt::Debug;

use crate::shared::{Day, PartSolution};

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
}

impl From<Vec<DestinationSourceRange>> for AlmanacMap {
    fn from(vec: Vec<DestinationSourceRange>) -> Self {
        AlmanacMap(vec)
    }
}

#[derive(PartialEq, Eq)]
struct DestinationSourceRange(u64, u64, u64);

impl DestinationSourceRange {
    fn map(&self, seed: u64) -> Option<u64> {
        if (self.1..self.1 + self.2).contains(&seed) {
            Some(seed - self.1 + self.0)
        } else {
            None
        }
    }
}

impl std::fmt::Debug for DestinationSourceRange {
    #[allow(clippy::range_minus_one)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestinationSourceRange")
            .field("Destination", &((self.0)..=(self.0 + self.2 - 1)))
            .field("Source", &((self.1)..=(self.1 + self.2 - 1)))
            .field("Range", &self.2)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: AlmanacMap,
    soil_to_fertilizer_map: AlmanacMap,
    fertilizer_to_water_map: AlmanacMap,
    water_to_light_map: AlmanacMap,
    light_to_temperature_map: AlmanacMap,
    temperature_to_humidity_map: AlmanacMap,
    humidity_to_location_map: AlmanacMap,
}

impl Almanac {
    fn seed_to_soil(&self, seed: u64) -> u64 {
        self.seed_to_soil_map.map(seed)
    }

    fn soil_to_fertilizer_map(&self, seed: u64) -> u64 {
        self.soil_to_fertilizer_map.map(seed)
    }

    fn fertilizer_to_water_map(&self, seed: u64) -> u64 {
        self.fertilizer_to_water_map.map(seed)
    }

    fn water_to_light_map(&self, seed: u64) -> u64 {
        self.water_to_light_map.map(seed)
    }

    fn light_to_temperature_map(&self, seed: u64) -> u64 {
        self.light_to_temperature_map.map(seed)
    }

    fn temperature_to_humidity_map(&self, seed: u64) -> u64 {
        self.temperature_to_humidity_map.map(seed)
    }

    fn humidity_to_location_map(&self, seed: u64) -> u64 {
        self.humidity_to_location_map.map(seed)
    }

    fn all_the_way(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil(seed);
        let fertilizer = self.soil_to_fertilizer_map(soil);
        let water = self.fertilizer_to_water_map(fertilizer);
        let light = self.water_to_light_map(water);
        let temperature = self.light_to_temperature_map(light);
        let humidity = self.temperature_to_humidity_map(temperature);

        self.humidity_to_location_map(humidity)
    }
}

fn parse_lines(lines: &[&str]) -> Almanac {
    let mut seeds = Vec::new();
    let mut seed_to_soil_map = None;
    let mut soil_to_fertilizer_map = None;
    let mut fertilizer_to_water_map = None;
    let mut water_to_light_map = None;
    let mut light_to_temperature_map = None;
    let mut temperature_to_humidity_map = None;
    let mut humidity_to_location_map = None;

    for group in lines.split(|line| line.is_empty()) {
        assert!(!group.is_empty());

        match group[0] {
            "seed-to-soil map:" => seed_to_soil_map = process_map(&group[1..]).into(),
            "soil-to-fertilizer map:" => soil_to_fertilizer_map = process_map(&group[1..]).into(),
            "fertilizer-to-water map:" => fertilizer_to_water_map = process_map(&group[1..]).into(),
            "water-to-light map:" => water_to_light_map = process_map(&group[1..]).into(),
            "light-to-temperature map:" => {
                light_to_temperature_map = process_map(&group[1..]).into();
            },
            "temperature-to-humidity map:" => {
                temperature_to_humidity_map = process_map(&group[1..]).into();
            },
            "humidity-to-location map:" => {
                humidity_to_location_map = process_map(&group[1..]).into();
            },
            s if s.starts_with("seeds:") => {
                seeds = group[0]
                    .split(' ')
                    .skip(1)
                    .map(|s| s.parse::<u64>().expect("Invalid seed"))
                    .collect();
            },
            _ => panic!("Invalid group"),
        }
    }

    Almanac {
        seeds,
        seed_to_soil_map: seed_to_soil_map.expect("Missing map"),
        soil_to_fertilizer_map: soil_to_fertilizer_map.expect("Missing map"),
        fertilizer_to_water_map: fertilizer_to_water_map.expect("Missing map"),
        water_to_light_map: water_to_light_map.expect("Missing map"),
        light_to_temperature_map: light_to_temperature_map.expect("Missing map"),
        temperature_to_humidity_map: temperature_to_humidity_map.expect("Missing map"),
        humidity_to_location_map: humidity_to_location_map.expect("Missing map"),
    }
}

fn process_map(lines: &[&str]) -> AlmanacMap {
    let mut almanac_map = Vec::new();
    for line in lines {
        let x = line
            .split(' ')
            .map(|s| s.parse().expect("Invalid number"))
            .collect::<Vec<u64>>();

        assert!(x.len() == 3);

        almanac_map.push(DestinationSourceRange(x[0], x[1], x[2]));
    }

    almanac_map.into()
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        let min_location = parsed
            .seeds
            .iter()
            .map(|s| parsed.all_the_way(*s))
            .min()
            .expect("No min found");

        min_location.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let _parsed = parse_lines(&lines);

        PartSolution::None
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use super::super::{parse_lines, Solution};
        use super::get_example;
        use crate::{
            day_05::{Almanac, DestinationSourceRange},
            shared::Day,
        };

        #[test]
        fn outcome() {
            assert_eq!(309_796_150, (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            assert_eq!(
                Almanac {
                    seeds: vec![79, 14, 55, 13],
                    seed_to_soil_map: vec![
                        DestinationSourceRange(50, 98, 2),
                        DestinationSourceRange(52, 50, 48)
                    ]
                    .into(),
                    soil_to_fertilizer_map: vec![
                        DestinationSourceRange(0, 15, 37),
                        DestinationSourceRange(37, 52, 2),
                        DestinationSourceRange(39, 0, 15)
                    ]
                    .into(),
                    fertilizer_to_water_map: vec![
                        DestinationSourceRange(49, 53, 8),
                        DestinationSourceRange(0, 11, 42),
                        DestinationSourceRange(42, 0, 7),
                        DestinationSourceRange(57, 7, 4)
                    ]
                    .into(),
                    water_to_light_map: vec![
                        DestinationSourceRange(88, 18, 7),
                        DestinationSourceRange(18, 25, 70)
                    ]
                    .into(),
                    light_to_temperature_map: vec![
                        DestinationSourceRange(45, 77, 23),
                        DestinationSourceRange(81, 45, 19),
                        DestinationSourceRange(68, 64, 13)
                    ]
                    .into(),
                    temperature_to_humidity_map: vec![
                        DestinationSourceRange(0, 69, 1),
                        DestinationSourceRange(1, 0, 69)
                    ]
                    .into(),
                    humidity_to_location_map: vec![
                        DestinationSourceRange(60, 56, 37),
                        DestinationSourceRange(56, 93, 4)
                    ]
                    .into(),
                },
                parsed
            );
        }

        #[test]
        fn example_seed_to_soil() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            assert_eq!(81, parsed.seed_to_soil(79));
            assert_eq!(14, parsed.seed_to_soil(14));
            assert_eq!(57, parsed.seed_to_soil(55));
            assert_eq!(13, parsed.seed_to_soil(13));
        }

        #[test]
        fn example_seed_to_location() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            assert_eq!(81, parsed.seed_to_soil(79));
            assert_eq!(81, parsed.soil_to_fertilizer_map(81));
            assert_eq!(81, parsed.fertilizer_to_water_map(81));
            assert_eq!(74, parsed.water_to_light_map(81));
            assert_eq!(78, parsed.light_to_temperature_map(74));
            assert_eq!(78, parsed.temperature_to_humidity_map(78));
            assert_eq!(82, parsed.humidity_to_location_map(78));
        }

        #[test]
        fn seed_maps() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            let locations = parsed
                .seeds
                .iter()
                .map(|s| parsed.all_the_way(*s))
                .collect::<Vec<u64>>();

            assert_eq!(vec![82, 43, 86, 35], locations);
        }
    }

    mod part_2 {
        use super::super::{parse_lines, Solution};
        use super::get_example;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::None, (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let _parsed = parse_lines(&lines);

            // assert_eq!(Vec::<u32>::new(), parsed);
        }
    }
}
