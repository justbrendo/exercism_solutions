use std::u32;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match code.chars().rev().try_fold(Vec::new(), |mut acc, c| {
        if c.is_whitespace() {
            Ok(acc)
        } else if let Some(digit) = c.to_digit(10) {
            acc.push(digit);
            Ok(acc)
        } else {
            Err(false)
        }
    }) {
        Ok(digits) => {
            if digits.len() < 2 {
                return false;
            }

            let sum: u32 = digits
                .iter()
                .enumerate()
                .map(|(index, &digit)| {
                    let mut result = digit;
                    if index % 2 == 1 {
                        result *= 2;
                        if result > 9 {
                            result -= 9;
                        }
                    }
                    result
                })
                .sum();

            if sum % 10 == 0 {
                true
            } else {
                false
            }
        }

        Err(_) => {
            return false;
        }
    }
}
