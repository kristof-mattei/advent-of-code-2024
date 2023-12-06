use std::fmt::Debug;
use std::ops::Range;

use super::{Almanac, AlmanacMap};
use crate::day_05::DestinationSourceRange;

pub(super) trait IntoSeed {
    fn into(raw_seeds: Vec<u64>) -> Self;
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct IndividualSeeds(pub(super) Vec<u64>);
impl IntoSeed for IndividualSeeds {
    fn into(raw_seed: Vec<u64>) -> Self {
        Self(raw_seed)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RangeOfSeeds(pub(super) Vec<Range<u64>>);

impl IntoSeed for RangeOfSeeds {
    fn into(raw_seeds: Vec<u64>) -> Self {
        Self(parse_seed_range(&raw_seeds))
    }
}

fn parse_seed_range(seeds_with_range: &[u64]) -> Vec<Range<u64>> {
    seeds_with_range
        .chunks(2)
        .map(|s| s[0]..s[0] + s[1])
        .collect::<Vec<Range<u64>>>()
}

pub(super) fn parse_lines<S: IntoSeed>(lines: &str) -> (Almanac, S) {
    let mut seeds = None;
    let mut seed_to_soil_map = None;
    let mut soil_to_fertilizer_map = None;
    let mut fertilizer_to_water_map = None;
    let mut water_to_light_map = None;
    let mut light_to_temperature_map = None;
    let mut temperature_to_humidity_map = None;
    let mut humidity_to_location_map = None;

    for group in lines
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
    {
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
                let values = group[0]
                    .split(' ')
                    .skip(1)
                    .map(|s| s.parse::<u64>().expect("Invalid seed"))
                    .collect();

                seeds = Some(S::into(values));
            },
            _ => panic!("Invalid group"),
        }
    }

    (
        Almanac {
            maps: vec![
                seed_to_soil_map.expect("Missing map"),
                soil_to_fertilizer_map.expect("Missing map"),
                fertilizer_to_water_map.expect("Missing map"),
                water_to_light_map.expect("Missing map"),
                light_to_temperature_map.expect("Missing map"),
                temperature_to_humidity_map.expect("Missing map"),
                humidity_to_location_map.expect("Missing map"),
            ]
            .into(),
        },
        seeds.expect("Missing seeds"),
    )
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
