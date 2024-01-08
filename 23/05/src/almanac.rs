use std::cmp::{max, min};
use std::ops::Range;
use std::str;

/// Maps between values of two Ranges of same size.
#[derive(Debug)]
pub struct RangeMap {
    source: Range<u64>,
    destination: Range<u64>,
}

impl RangeMap {
    pub fn new(destination_start: u64, source_start: u64, length: u64) -> Self {
        RangeMap {
            source: source_start..(source_start + length),
            destination: destination_start..(destination_start + length),
        }
    }

    pub fn map(&self, value: &u64) -> Option<u64> {
        let RangeMap {
            source,
            destination,
        } = self;

        match source.contains(value) {
            true => Some(destination.start + value - source.start),
            false => None,
        }
    }

    pub fn map_range(&self, range: &Range<u64>) -> Option<Range<u64>> {
        match (self.map(&range.start), self.map(&(range.end - 1))) {
            (Some(start), Some(end)) => Some(start..(end + 1)),
            _ => None,
        }
    }

    pub fn intersection(&self, range: &Range<u64>) -> Option<Range<u64>> {
        let intersection = Range {
            start: max(range.start, self.source.start),
            end: min(range.end, self.source.end),
        };

        if intersection.start > intersection.end {
            // invalid intersection
            return None;
        }

        Some(intersection)
    }
}

#[derive(Debug)]
pub struct Almanac {
    seed_to_soil: Vec<RangeMap>,
    soil_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_water: Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temp: Vec<RangeMap>,
    temp_to_humidity: Vec<RangeMap>,
    humidity_to_location: Vec<RangeMap>,
}

impl Almanac {
    pub fn new() -> Self {
        Self {
            seed_to_soil: vec![],
            soil_to_fertilizer: vec![],
            fertilizer_to_water: vec![],
            water_to_light: vec![],
            light_to_temp: vec![],
            temp_to_humidity: vec![],
            humidity_to_location: vec![],
        }
    }

    fn try_push(&mut self, map_name: &str, range_map: RangeMap) -> Result<(), String> {
        let map = match map_name {
            "seed-to-soil" => &mut self.seed_to_soil,
            "soil-to-fertilizer" => &mut self.soil_to_fertilizer,
            "fertilizer-to-water" => &mut self.fertilizer_to_water,
            "water-to-light" => &mut self.water_to_light,
            "light-to-temperature" => &mut self.light_to_temp,
            "temperature-to-humidity" => &mut self.temp_to_humidity,
            "humidity-to-location" => &mut self.humidity_to_location,
            _ => {
                return Err(format!("Unknown map {}", map_name));
            }
        };
        map.push(range_map);
        Ok(())
    }

    pub fn pipeline(&self) -> [&Vec<RangeMap>; 7] {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temp,
            &self.temp_to_humidity,
            &self.humidity_to_location,
        ]
    }

    /// converts a seed value to a location value by following the maps in the correct order
    pub fn seed_to_location(&self, seed_value: u64) -> u64 {
        let mut mapped_value: u64 = seed_value;
        for stage in self.pipeline() {
            for range_map in stage {
                if let Some(destination_value) = range_map.map(&mapped_value) {
                    mapped_value = destination_value;
                    break;
                }
            }
        }
        mapped_value
    }

    pub fn closest_seed(&self, seeds: &[u64]) -> u64 {
        seeds
            .iter()
            .map(|&seed| self.seed_to_location(seed))
            .min()
            .unwrap()
    }

    pub fn closest_seed_range(&self, seed_ranges: &[Range<u64>]) -> u64 {
        let mut mapped = vec![];
        let mut unmapped = seed_ranges.to_vec();

        for stage in self.pipeline() {
            // at the beginning of a stage, consider all previously mapped values as unmapped
            unmapped.append(&mut mapped);
            let mut remainder = vec![];
            for map in stage {
                // try to process a range using the current RangeMap
                while let Some(range) = unmapped.pop() {
                    match map.intersection(&range) {
                        Some(intersection) => {
                            // if there is an intersection, then by definition it can be mapped
                            let mapped_intersection = map.map_range(&intersection).unwrap();
                            mapped.push(mapped_intersection);

                            // push back any left over ranges for the next map(s) to process
                            if range.start < intersection.start {
                                remainder.push(range.start..intersection.start)
                            }

                            if range.end > intersection.end {
                                remainder.push(intersection.end..range.end)
                            }
                        }
                        None => {
                            // this map cannot process the range, pass it along to the next map
                            remainder.push(range);
                        }
                    }
                }
                // at the end of a map, put the remainder back into the unmapped collection
                // so that the next RangeMap can try to map it
                unmapped.append(&mut remainder);
            }

            // at the end of a stage, consider every range as mapped
            mapped.append(&mut unmapped);
        }

        // at the end of the pipeline get the smallest start value
        mapped.iter().map(|range| range.start).min().unwrap()
    }
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut almanac = Almanac::new();
        let mut current_map_name: Option<&str> = None;

        for line in input.lines().filter(|line| !line.is_empty()) {
            if let Some((map_name, _)) = line.split_once(" map:") {
                current_map_name = Some(map_name);
                continue;
            }

            if let Some(map_name) = current_map_name {
                let numbers: Vec<u64> = line
                    .split_whitespace()
                    .take(3)
                    .map(|value| value.parse::<u64>().expect("Values should be valid u64"))
                    .collect::<Vec<_>>();

                almanac
                    .try_push(map_name, RangeMap::new(numbers[0], numbers[1], numbers[2]))
                    .unwrap();
            }
        }

        almanac
    }
}
