use crate::game::{Color, CubeSet, Game, Round};
use std::convert::TryFrom;

impl TryFrom<&str> for Game {
    type Error = String;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str.split_once(": ") {
            Some((id_section, game_section)) => {
                let (_, id_str) = id_section.split_once(' ').ok_or("Invalid Game ID")?;
                let id = id_str.parse::<usize>().map_err(|_| "Invalid Game ID")?;

                game_section
                    .split("; ")
                    .map(Round::try_from)
                    .collect::<Result<_, _>>()
                    .map(|rounds| Game { id, rounds })
            }
            None => Err(format!(
                "Invalid Game \"{str}\". Expected pattern is: \"{}\"",
                "Game {id}: {Round}; {Round}; ...; {Round}",
            )),
        }
    }
}

impl TryFrom<&str> for Round {
    type Error = String;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.split(", ")
            .map(CubeSet::try_from)
            .collect::<Result<_, _>>()
            .map(Round)
    }
}

impl TryFrom<&str> for CubeSet {
    type Error = String;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str.split_once(' ') {
            Some((left, right)) => {
                let amount = left
                    .parse::<u32>()
                    .map_err(|_| format!("Amount \"{left}\" is not valid"))?;

                let color = Color::try_from(right)?;

                Ok(CubeSet(color, amount))
            }
            None => Err(format!(
                "Invalid CubeSet \"{str}\". Expected pattern is: \"{}\"",
                "{amount} {color}"
            )),
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(format!("Invalid Color \"{str}\"")),
        }
    }
}
