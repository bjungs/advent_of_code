use crate::scratchcard::ScratchCardCollection;

pub(crate) fn solve(input: &str) -> u32 {
    ScratchCardCollection::from(input).count_winning()
}
