/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // filter out whitespace
    let codez: Vec<char> = code.chars().filter(|x| !x.is_whitespace()).collect();

    // check for non-numeric chars and zero length
    if codez.len() <= 1 || codez.iter().filter(|x| x.is_ascii_digit()).count() < codez.len() {
        return false;
    }

    let total: u32 = codez
        .iter()
        .rev()
        .enumerate()
        .map(|(n, x)| {
            if (n + 1) % 2 == 0 {
                let mut product = x.to_digit(10).unwrap() * 2;
                if product > 9 {
                    product -= 9;
                }
                product
            } else {
                x.to_digit(10).unwrap()
            }
        })
        .sum();

    total % 10 == 0
}

// pub fn is_valid(code: &str) -> bool {
//     code.chars()
//         .rev()
//         .filter(|c| !c.is_whitespace())
//         .try_fold((0, 0), |(sum, c), d| {
//             d.to_digit(10)
//                 .map(|n| if c % 2 == 1 { n * 2 } else { n })
//                 .map(|n| if n > 9 { n - 9 } else { n })
//                 .map(|n| (n + sum, c + 1))
//         }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
// }
