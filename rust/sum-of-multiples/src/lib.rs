use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter().flat_map(|item| {
        (1..limit)
            .map(move |i| item *i)
            .take_while(|&multiple| multiple < limit)
    }).collect::<HashSet<u32>>().iter().sum()
}
