pub fn raindrops(n: u32) -> String {
    let mut string = String::new();
    if n % 3 == 0 {
        string.push_str("Pling");
    }
    if n % 5 == 0 {
        string.push_str("Plang");
    }
    if n % 7 == 0 {
        string.push_str("Plong");
    }

    if string.is_empty() {
        string.push_str(&n.to_string());
    }

    string
}

// pub fn raindrops(n: u32) -> String {
//     let lookup = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];
//     let string: String = lookup.iter().filter_map(|(factor, raindrop)| {
//         match n % factor {
//             0 => Some(*raindrop),
//             _ => None
//         }
//     }).collect();
//     match string.is_empty() {
//         true => n.to_string(),
//         false => string
//     }
// }

// pub fn raindrops(n: u32) -> String {
//     let is_divisible = |d| n % d == 0;
//     let mut sound = String::new();
//     if is_divisible(3) { sound += "Pling" };
//     if is_divisible(5) { sound += "Plang" };
//     if is_divisible(7) { sound += "Plong" };

//     if sound.is_empty() {
//         return n.to_string();
//     } else {
//         return sound;
//     }
// }

// fn map_drop(n: u32) -> Option<&'static str> {
//     match n {
//         3 => Some("Pling"),
//         5 => Some("Plang"),
//         7 => Some("Plong"),
//         _ => None,
//     }
// }

// pub fn raindrops(n: u32) -> String {
//     let drop_string = [3, 5, 7]
//         .iter()
//         .filter(|&&x| n % x == 0)
//         .map(|&x| map_drop(x).unwrap().to_string())
//         .fold("".to_string(), |acc, s| acc + &s);

//     match drop_string.as_str() {
//         "" => format!("{}", n),
//         _ => drop_string,
//     }
// }
