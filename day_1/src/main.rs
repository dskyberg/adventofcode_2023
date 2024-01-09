use anyhow::{anyhow, Result};

fn find_first_digit(data: &str) -> Option<(char, usize)> {
    for (idx, val) in data.chars().enumerate() {
        if val.is_ascii_digit() {
            return Some((val, idx));
        }
    }
    None
}

fn find_last_digit_before(data: &str, before: usize) -> Option<char> {
    let last = data.len() - before;
    for (idx, val) in data.chars().rev().enumerate() {
        if idx >= last {
            eprintln!("No last digit before first digit");
            return None;
        }
        if val.is_ascii_digit() {
            return Some(val);
        }
    }
    None
}

fn make_number(first: char, last: char) -> Result<usize> {
    let val = format!("{}{}", first, last).parse::<usize>()?;

    Ok(val)
}

fn starts_with(value: &Vec<char>, target: &Vec<char>) -> bool {
    if target.len() > value.len() {
        return false;
    }
    for idx in 0..target.len() {
        if value[idx] != target[idx] {
            return false;
        }
    }
    true
}

/// Turn number words into digits.  So, 'one' becomes '1', etc.
/// WARNING!!  'eightwo' should become '82', not '8wo'.
fn replace_words(in_row: &str, words: &[Vec<char>]) -> Result<String> {
    let mut row_vec = in_row.chars().collect::<Vec<char>>();
    let mut result = Vec::<char>::new();

    while !row_vec.is_empty() {
        let mut found = false;
        for (word_idx, _) in words.iter().enumerate() {
            if starts_with(&row_vec, &words[word_idx]) {
                let val = char::from_digit(word_idx as u32, 10).unwrap();
                result.push(val);
                found = true;
                break;
            }
        }
        if !found {
            result.push(row_vec[0]);
        }
        if !row_vec.is_empty() {
            row_vec = row_vec[1..].to_vec();
        }
    }

    let row: String = result.iter().collect();
    Ok(row)
}

fn part_one() -> Result<()> {
    let data_file = include_str!("../puzzle_input.txt");
    let data = data_file.split('\n').collect::<Vec<&str>>();
    let mut total: usize = 0;
    for row in data {
        let (first, idx) = find_first_digit(row).ok_or(anyhow!("Failed to find first"))?;
        let last = find_last_digit_before(row, idx).ok_or(anyhow!("Failed to find last"))?;
        let val = make_number(first, last).expect("oops");
        total += val;
    }
    println!("Part one: {}", total);
    Ok(())
}

fn part_two() -> Result<()> {
    let words: Vec<Vec<char>> = vec![
        "zero".chars().collect::<Vec<char>>(),
        "one".chars().collect::<Vec<char>>(),
        "two".chars().collect::<Vec<char>>(),
        "three".chars().collect::<Vec<char>>(),
        "four".chars().collect::<Vec<char>>(),
        "five".chars().collect::<Vec<char>>(),
        "six".chars().collect::<Vec<char>>(),
        "seven".chars().collect::<Vec<char>>(),
        "eight".chars().collect::<Vec<char>>(),
        "nine".chars().collect::<Vec<char>>(),
    ];
    let data_file = include_str!("../puzzle_input.txt");
    let data = data_file.split('\n').collect::<Vec<&str>>();
    let mut total: usize = 0;
    for orig in data {
        let orig = orig.trim();
        if orig.is_empty() {
            continue;
        }
        let row = replace_words(orig, &words)?;
        let (first, idx) = find_first_digit(&row).ok_or(anyhow!("Failed to find first"))?;
        let last = find_last_digit_before(&row, idx).ok_or(anyhow!("Failed to find last"))?;
        let val = make_number(first, last).expect("oops");
        total += val;
    }
    println!("Part two: {}", total);
    Ok(())
}

fn main() -> Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
