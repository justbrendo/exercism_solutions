use core::num;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.

fn to_base(mut num: u32, base: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.push(num % base);
        num /= base;
    }
    digits.reverse();
    digits
}

pub fn convert(number: &[u32], from_base: u32, new_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase)
    }

    if new_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut digits = number.to_vec();

    // Remove leading 0s
    while digits.first() == Some(&0) {
        digits.remove(0);
    }

    if digits.is_empty() {
        return Ok(vec![0]);
    }

    let mut base_10 = 0;
    let len = digits.len();
    
    // Convert Number to Base 10
    for (index, digit) in digits.iter().enumerate() {
        if digit >= &from_base {
            return Err(Error::InvalidDigit(*digit));
        }
        let inverted_index = (len - index -1) as u32;
        base_10 += digit * from_base.pow(inverted_index);
    }

    Ok(to_base(base_10, new_base))
}
