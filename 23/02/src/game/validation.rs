use crate::game::{Color, CubeSet, Game, Round};
use std::collections::HashMap;

pub type CubeLimits = HashMap<Color, u32>;

impl Game {
    pub fn is_possible(&self, limits: &CubeLimits) -> bool {
        self.rounds.iter().all(|round| {
            let Round(sets) = round;
            sets.iter().all(|set| {
                let CubeSet(color, count) = set;
                match limits.get(color) {
                    Some(limit) => count <= limit,
                    None => true,
                }
            })
        })
    }
}
