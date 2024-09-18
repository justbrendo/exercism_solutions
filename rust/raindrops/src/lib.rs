pub fn raindrops(n: u32) -> String {
    let sequence = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let result = sequence.iter().filter_map(|(x, s)| { (n % x == 0).then(|| *s) }).collect::<String>();
    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
