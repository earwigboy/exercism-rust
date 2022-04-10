#[derive(Debug, PartialEq)]
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
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
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
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    for &n in number {
        if n >= from_base {
            return Err(Error::InvalidDigit(n));
        }
    }

    if number.is_empty() {
        return Ok([0].to_vec());
    }

    let mut result = Vec::<u32>::new();

    // convert to base 10
    let base_10: u32 = (0..)
        .zip(number.iter().rev())
        .map(|(i, x)| x * from_base.pow(i))
        .sum();

    if base_10 == 0 {
        return Ok([0].to_vec());
    }

    // convert to base n
    println!("{}", base_10);
    let mut n = base_10;
    while n > 0 {
        let r = n.checked_rem(to_base).unwrap();
        result.push(r);
        match n.checked_div(to_base) {
            Some(a) => n = a,
            None => return Err(Error::InvalidOutputBase),
        };
    }

    result = result.iter().rev().copied().collect();

    Ok(result)
}
