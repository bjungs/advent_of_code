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

    pub fn get(&self, source_value: &u64) -> Option<u64> {
        let RangeMap {
            source,
            destination,
        } = self;

        match source.contains(source_value) {
            true => Some(destination.start + source_value - source.start),
            false => None,
        }
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

    /// converts a seed value to a location value by following the maps in the correct order
    pub fn seed_to_location(&self, seed_value: &u64) -> u64 {
        let map_collections = [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temp,
            &self.temp_to_humidity,
            &self.humidity_to_location,
        ];

        let mut mapped_value: u64 = seed_value.clone();
        for collection in map_collections {
            for range_map in collection {
                if let Some(destination_value) = range_map.get(&mapped_value) {
                    mapped_value = destination_value;
                    break;
                }
            }
        }
        mapped_value
    }

    pub fn closest_seed(&self, seeds: &Vec<u64>) -> u64 {
        seeds
            .iter()
            .map(|seed| self.seed_to_location(seed))
            .min()
            .unwrap()
    }
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut almanac = Almanac::new();
        let mut current_map_name: Option<&str> = None;

        for line in input.lines().filter(|line| line.len() > 0) {
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
                    .try_push(&map_name, RangeMap::new(numbers[0], numbers[1], numbers[2]))
                    .unwrap();
            }
        }

        almanac
    }
}
