pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let sum: u32 = digits
        .iter()
        .fold(0, |acc, num| acc + num.pow(digits.len() as u32));

    sum == num
}
