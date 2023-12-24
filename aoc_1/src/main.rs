use std::fs;

const WORD_TO_DIGIT_MAP: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn get_combined_digits(line: &str) -> u32 {
    let mut positions: Vec<(usize, char)> = Vec::new();

    // Find positions of spelled-out numbers
    for &(word, digit) in &WORD_TO_DIGIT_MAP {
        let mut idx = 0;
        while let Some(start_idx) = line[idx..].find(word) {
            let position = start_idx + idx;
            positions.push((position, digit));
            idx = position + 1;
        }
    }

    // Add positions of existing numerical digits
    positions.extend(line.char_indices().filter_map(|(idx, ch)| {
        if ch.is_ascii_digit() {
            Some((idx, ch))
        } else {
            None
        }
    }));

    // Handle no digits case
    if positions.is_empty() {
        return 0;
    }

    // Sort positions by index
    positions.sort_unstable_by_key(|&(idx, _)| idx);

    // Extract first and last digit based on combined order
    let first = positions[0].1;
    let last = positions[positions.len() - 1].1;
    format!("{}{}", first, last).parse::<u32>().unwrap_or(0)
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let sum: u32 = content.lines().map(get_combined_digits).sum();

    println!("{}", sum); // Corrected the print statement
}
