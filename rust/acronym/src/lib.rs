pub fn all_caps(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

pub fn all_lower(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase())
}


pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();

    let words: Vec<&str> = phrase.split(&[' ', '-']).filter(|&x| !x.is_empty()).collect();
    for w in words {
        if all_caps(w) | all_lower(w) {
            acronym.push(w.chars().next().unwrap());
        } else {
            w.chars().filter(|c| c.is_uppercase()).for_each(|c| acronym.push(c));
        }
    }

    acronym.to_uppercase()
}
