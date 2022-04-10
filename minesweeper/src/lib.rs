use std::{char::from_digit, convert::TryFrom};
const MINE_BYTE: u8 = b'*';
const MINE_CHAR: char = '*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(y, line)| {
            // iterate over the y axis
            line.chars() // convert line to chars, chars is an iterator
                .enumerate() // enumerate over chars, gets the value and the x coordinate
                .map(|(x, c)| match c {
                    MINE_CHAR => MINE_CHAR, // don't bother doing anything for MINE_CHAR, has no numeric value
                    _ => match count_mines(minefield, x, y) {
                        0 => ' ',
                        x => from_digit(u32::try_from(x).unwrap(), 10).unwrap(),
                    },
                })
                .collect()
        })
        .collect()
}

fn count_mines(minefield: &[&str], x: usize, y: usize) -> usize {
    // look in the surrounding cells
    // saturating_sub used so the coordinates don't go below 0
    (y.saturating_sub(1)..=y + 1) // saturating_sub, safe subtraction. E.g. 5.saturating_sub(7) = 0 (doesn't overflow)
        .filter_map(|y| minefield.get(y)) // filters everything where the map function returns Some(x).
        .flat_map(|line| (x.saturating_sub(1)..=x + 1).filter_map(move |x| line.as_bytes().get(x))) // move transfers ownership of variables used inside the closure to the closure.
        .filter(|&&c| c == MINE_BYTE) // filter out only mines
        .count() // count all the mines
}
