use std::slice::Iter;
use std::vec::IntoIter;
use crate::almanac::range_map::RangeMap;

#[derive(Default, Debug)]
pub struct MappingStage {
    maps: Vec<RangeMap>
}

impl MappingStage {
    pub fn add_map(&mut self, range_map: RangeMap) {
        self.maps.push(range_map);
    }
}

impl IntoIterator for MappingStage {
    type Item = RangeMap;
    type IntoIter = IntoIter<RangeMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.maps.into_iter()
    }
}

impl<'a> IntoIterator for &'a MappingStage {
    type Item = &'a RangeMap;
    type IntoIter = Iter<'a, RangeMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.maps.iter()
    }
}
