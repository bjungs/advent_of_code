enum MapParameter {
    Seed(u32),
    Soil(u32),
    Fertilizer(u32),
    Water(u32),
    Light(u32),
    Temperature(u32),
    Humidity(u32),
    Location(u32),
}

use MapParameter::*;
use std::ops::{Range};

/// Maps between values of two Ranges of same size.
#[derive(Debug)]
pub struct RangeMap {
    source: Range<u32>,
    destination: Range<u32>,
}

impl RangeMap {
    pub fn new(source_start: u32, destination_start: u32, length: u32) -> Self {
        RangeMap {
            source: source_start..(source_start + length),
            destination: destination_start..(destination_start + length),
        }
    }

    pub fn get(&self, source_value: &u32) -> Option<u32> {
        let RangeMap {
            source,
            destination,
        } = self;

        Option::from(source.contains(source_value)).map(|_| {
            // apply the difference from start of source to the start of destination
            destination.start + source_value - source.start
        })
    }
}

#[derive(Debug)]
pub struct Almanac {
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temp: RangeMap,
    temp_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

impl Almanac {
    pub fn seed_to_location(&self, seed_value: &u32) -> Option<u32> {
       todo!()
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {

    }
}
