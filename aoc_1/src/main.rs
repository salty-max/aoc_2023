use std::{
    env, fs,
    io::{self, BufRead},
};

fn get_double_digit_from_string(s: &str) -> Option<u32> {
    let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() >= 2 {
        let first_digit = digits[0].to_digit(10)?;
        let last_digit = digits[digits.len() - 1].to_digit(10)?;

        Some(first_digit * 10 + last_digit)
    } else if digits.len() == 1 {
        let digit = digits[0].to_digit(10)?;

        Some(digit * 10 + digit)
    } else {
        None
    }
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let input_file_path = cwd.join("input.txt");

    let file = fs::File::open(input_file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut res = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let double_digit = get_double_digit_from_string(&line);

                match double_digit {
                    Some(number) => res += number,
                    None => continue,
                }
            }
            Err(_) => continue,
        }
    }

    println!("{res}");
}
