pub fn build_proverb(list: &[&str]) -> String {
    let mut string: Vec<String> = Vec::new();
    let mut last = String::from("");

    let mut previous: &str = "";
    for (counter, i) in list.iter().enumerate() {
        if counter == 0 {
            last = format!("And all for the want of a {}.", i);
        } else {
            string.push(format!("For want of a {} the {} was lost.", previous, i))
        }
        previous = i;
    }
    string.push(last);
    string.join("\n")
}

/*
pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|window| format!("For want of a {} the {} was lost.\n", window[0], window[1]))
        .chain([list.first().map_or(String::new(), |item| {
            format!("And all for the want of a {}.", item)
        })])
        .collect::<Vec<String>>()
        .join("")
}
*/
