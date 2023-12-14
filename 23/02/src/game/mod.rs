mod from_str;
pub mod validation;

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round(Vec<CubeSet>);

#[derive(Debug)]
pub struct CubeSet(Color, u32);

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}
