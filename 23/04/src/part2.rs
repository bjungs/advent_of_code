use crate::scratchcard::ScratchCard;

pub(crate) fn solve(input: &str) -> u64 {
    let scratchcards: Vec<ScratchCard> = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .expect("format should be \"Card {number}: {lucky_numbers} | {game_numbers}\"")
        })
        .map(|(_, sc_input)| ScratchCard::from(sc_input))
        .collect();

    scratchcards
        .iter()
        .enumerate()
        .fold(
            Vec::new(),
            |mut sc_count_vec: Vec<(&ScratchCard, u64)>, (idx, sc)| {
                // set initial count
                let count = match sc_count_vec.get(idx) {
                    Some((_, count)) => *count,
                    None => {
                        sc_count_vec.push((&sc, 1));
                        1
                    }
                };

                // for every winning number, add 1 more of the next SCs for every copy of this SC
                let winning_numbers_count = sc.winning_numbers().iter().count();
                for winning_number_idx in 0..winning_numbers_count {
                    let next_sc_idx = idx + winning_number_idx + 1;
                    match sc_count_vec.get_mut(next_sc_idx) {
                        Some((_, next_count)) => {
                            *next_count += count;
                        }
                        None => {
                            sc_count_vec.insert(next_sc_idx, (sc, 1 + count));
                        }
                    }
                }

                sc_count_vec
            },
        )
        .iter()
        .map(|(_, count)| *count)
        .sum()
}
