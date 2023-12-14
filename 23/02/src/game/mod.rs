mod from_str;
pub mod validation;

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round(Vec<CubeDraw>);

#[derive(Debug)]
pub struct CubeDraw(Color, u32);

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}
