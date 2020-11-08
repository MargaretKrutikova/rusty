use itertools::Itertools;

fn get_multiples(limit: u32, multiple: u32) -> Vec<u32> {
    (1..)
        .map(|n| n * multiple)
        .take_while(|&number| number < limit)
        .collect()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&factor| *factor != 0)
        .map(|&factor| get_multiples(limit, factor))
        .concat()
        .into_iter()
        .unique()
        .sum()
}
