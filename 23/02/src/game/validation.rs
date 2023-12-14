use crate::game::{Color, CubeSet, Game, Round};
use std::collections::HashMap;

/// Maps an amount of cubes per [Color]
pub type CubeMap = HashMap<Color, u32>;

impl Game {
    /// Returns whether a game is possible given a limited amount of cubes per [Color].
    pub fn is_possible(&self, limits: &CubeMap) -> bool {
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

    /// Returns a mapping of the highest number of cubes drawn in a single [Round] for each [Color].
    pub fn max(&self) -> CubeMap {
        self.rounds
            .iter()
            .fold(CubeMap::new(), |mut max_cubes, round| {
                let Round(sets) = round;
                for set in sets.iter() {
                    let CubeSet(color, count) = set;
                    let min_curr_option = max_cubes.get(&color);

                    if min_curr_option.is_none()
                        || min_curr_option.is_some_and(|value| count > value)
                    {
                        max_cubes.insert(*color, *count);
                    }
                }
                max_cubes
            })
    }
}
