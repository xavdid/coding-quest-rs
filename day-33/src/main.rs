use std::collections::{HashMap, HashSet};

// J intentionally excluded
const ASCII_LOWER: [char; 25] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
];

fn build_grid(raw_key: &str) -> HashMap<char, (i32, i32)> {
    let mut chars: HashSet<char> = HashSet::new();

    let cipher_key = format!(
        "{}{}",
        // insert returns whether the item was already in the set, so it works great as a filter
        raw_key
            .chars()
            .filter(|c| chars.insert(*c))
            .collect::<String>(),
        ASCII_LOWER
            .iter()
            .filter(|c| chars.insert(**c))
            .collect::<String>()
    );

    let mut positions: HashMap<char, (i32, i32)> = HashMap::new();

    assert_eq!(cipher_key.len(), 25);

    let mut row = 0;
    let mut col = 0;
    for c in cipher_key.chars() {
        positions.insert(c, (row, col));
        if col == 4 {
            col = 0;
            row += 1;
        } else {
            col += 1;
        }
    }

    positions
}

// chat gpt; I didn't know how to make iterators
fn chunks(input: &str) -> impl Iterator<Item = (char, char)> + '_ {
    assert_eq!(input.len() % 2, 0);
    let mut chars = input.chars();
    std::iter::from_fn(move || {
        let first = chars.next()?;
        let second = chars.next()?;
        Some((first, second))
    })
}

// https://stackoverflow.com/a/57342011/1825390
fn wrap_5(i: i32) -> i32 {
    (i - 1).rem_euclid(5)
}

fn main() {
    let mut blocks = include_str!("input.txt").split("\n\n");

    let raw_key = blocks.next().unwrap()[12..].trim();
    let message = blocks.next().unwrap()[9..].trim();

    let letter_to_pos = build_grid(raw_key);
    let pos_to_letter: HashMap<(i32, i32), char> =
        letter_to_pos.iter().map(|(k, v)| (*v, *k)).collect();

    let mut result = String::new();
    for word in message.split_ascii_whitespace() {
        for (l, r) in chunks(word) {
            let (l_pos, r_pos) = (
                letter_to_pos.get(&l).unwrap(),
                letter_to_pos.get(&r).unwrap(),
            );

            if l_pos.0 == r_pos.0 {
                // same row, shift left
                result.push(*pos_to_letter.get(&(l_pos.0, wrap_5(l_pos.1))).unwrap());
                result.push(*pos_to_letter.get(&(r_pos.0, wrap_5(r_pos.1))).unwrap());
            } else if l_pos.1 == r_pos.1 {
                // same col, shift up
                result.push(*pos_to_letter.get(&(wrap_5(l_pos.0), l_pos.1)).unwrap());
                result.push(*pos_to_letter.get(&(wrap_5(r_pos.0), r_pos.1)).unwrap());
            } else {
                // trade corners
                result.push(*pos_to_letter.get(&(l_pos.0, r_pos.1)).unwrap());
                result.push(*pos_to_letter.get(&(r_pos.0, l_pos.1)).unwrap());
            }
        }
        result.push(' ');
    }
    println!("{result}");
    assert_eq!(result.trim(), "please pick up some milk on thex wayx home");
}
