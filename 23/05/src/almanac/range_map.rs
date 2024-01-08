use std::cmp::{max, min};
use std::ops::Range;

/// Maps values between two Ranges of same size.
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
