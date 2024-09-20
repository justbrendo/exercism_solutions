pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut series = Vec::new();
    for w  in digits.chars().collect::<Vec<_>>().windows(len) {
        series.push(w.iter().collect());
    }
    series
}
