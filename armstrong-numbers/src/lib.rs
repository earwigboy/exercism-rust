pub fn is_armstrong_number(num: u32) -> bool {
    let str_repr = num.to_string();
    let l = str_repr.chars().count() as u32;

    let x = str_repr
        .chars()
        .map(|i| i.to_digit(10).unwrap().pow(l))
        .sum::<u32>();
    x == num
}
