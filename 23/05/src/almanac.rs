use std::ops::Range;
use std::str;

mod map_stage;
mod range_map;

use crate::almanac::map_stage::MappingStage;
use crate::almanac::range_map::RangeMap;

#[derive(Debug, Default)]
pub struct Almanac {
    seed_to_soil: MappingStage,
    soil_to_fertilizer: MappingStage,
    fertilizer_to_water: MappingStage,
    water_to_light: MappingStage,
    light_to_temp: MappingStage,
    temp_to_humidity: MappingStage,
    humidity_to_location: MappingStage,
}

impl Almanac {
    fn get_stage_mut(&mut self, name: &str) -> Result<&mut MappingStage, String> {
        let stage= match name {
            "seed-to-soil" => &mut self.seed_to_soil,
            "soil-to-fertilizer" => &mut self.soil_to_fertilizer,
            "fertilizer-to-water" => &mut self.fertilizer_to_water,
            "water-to-light" => &mut self.water_to_light,
            "light-to-temperature" => &mut self.light_to_temp,
            "temperature-to-humidity" => &mut self.temp_to_humidity,
            "humidity-to-location" => &mut self.humidity_to_location,
            _ => {
                return Err(format!("Unknown stage {}", name));
            }
        };

        Ok(stage)
    }

    fn add_map(&mut self, stage_name: &str, map: RangeMap) -> Result<(), String> {
        self.get_stage_mut(stage_name)?.add_map(map);
        Ok(())
    }

    pub fn pipeline(&self) -> std::array::IntoIter<&MappingStage, 7> {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temp,
            &self.temp_to_humidity,
            &self.humidity_to_location,
        ].into_iter()
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
        let mut almanac = Almanac::default();
        let mut stage_name: Option<&str> = None;

        for line in input.lines().filter(|line| !line.is_empty()) {
            if let Some((map_name, _)) = line.split_once(" map:") {
                stage_name = Some(map_name);
                continue;
            }

            let stage_name = stage_name.expect("Map name is required");
            let numbers: Vec<u64> = line
                .split_whitespace()
                .take(3)
                .map(|value| value.parse::<u64>().expect("Values should be valid u64"))
                .collect::<Vec<_>>();

            assert_eq!(
                numbers.len(),
                3,
                "Three numbers are required per map: \"[destination] [source] [length]\"."
            );

            almanac
                .add_map(stage_name, RangeMap::new(numbers[0], numbers[1], numbers[2]))
                .unwrap();
        }

        almanac
    }
}
