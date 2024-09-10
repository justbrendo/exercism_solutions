use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {

    let lower = word.to_lowercase();
    let mut solution: HashSet<&str> = HashSet::new();

    let mut x: Vec<char> = word.chars().map(|c| c.to_lowercase().next().unwrap()).collect();
    x.sort();

    for w in possible_anagrams.iter() {
        if w.to_lowercase() != lower {
            let mut sw: Vec<char> = w.chars().map(|c| c.to_lowercase().next().unwrap()).collect();
            sw.sort();
            if x.eq(&sw) {
                solution.insert(w);
            }
        }
    }

    return solution;
}
