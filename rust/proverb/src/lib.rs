pub fn build_proverb(list: &[&str]) -> String {
    let mut r = list
        .windows(2)
        .map(|window| {
            let [x, y] = window else {unreachable!()};
            format!("For want of a {} the {} was lost.", x, y)
        })
        .collect::<Vec<_>>()
        .join("\n");
    if list.len() > 0 {
        if !r.is_empty() {
            r.push_str("\n");
        }
        r.push_str(&format!(
            "And all for the want of a {}.",
            list.first().unwrap()
        ));
    }
    r
}
