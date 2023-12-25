use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::Instant;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\p{L}+)([=-])([\d]?)").unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Equals,
    Minus,
}

impl TryFrom<&str> for Operation {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            "=" => Ok(Self::Equals),
            "-" => Ok(Self::Minus),
            _ => Err(anyhow::anyhow!(format!("Not an operation: {}", value))),
        }
    }
}

fn hash(value: &str) -> u32 {
    value.bytes().fold(0, |acc, b| {
        if b == b'\n' {
            acc
        } else {
            (acc + b as u32) * 17 % 256
        }
    })
}

fn parse_operation(op: &str) -> Result<(u32, String, Operation, Option<u32>)> {
    let captures = RE.captures(op).ok_or(anyhow!("Failed to parse regex"))?;
    let label = captures
        .get(1)
        .ok_or(anyhow!("Missing hash"))?
        .as_str()
        .to_string();
    let hash = hash(&label);
    let op = Operation::try_from(captures.get(2).ok_or(anyhow!("Missing operatin"))?.as_str())?;
    let value = match captures.get(3) {
        Some(s) => {
            let s = s.as_str();
            if s.is_empty() {
                None
            } else {
                Some(s.parse::<u32>()?)
            }
        }
        _ => None,
    };
    Ok((hash, label, op, value))
}

fn part_one(input: &str) -> Result<()> {
    let timer = Instant::now();
    let result = input.split(',').fold(0, |acc, s| acc + hash(s));

    println!("Part One: {} -- {:?}", result, timer.elapsed());
    Ok(())
}

fn parse_steps(boxes: &mut [IndexMap<String, u32>], input: &str) -> Result<()> {
    let steps = input.split(',').collect::<Vec<&str>>();
    for step in steps {
        let (hash, label, op, value) = parse_operation(step)?;
        let map = boxes.get_mut(hash as usize).unwrap();
        match op {
            Operation::Equals => {
                map.insert(label, value.ok_or(anyhow!("value isn't set"))?);
            }
            Operation::Minus => {
                if map.contains_key(&label) {
                    map.shift_remove(&label);
                }
            }
        }
    }
    Ok(())
}
fn calc_focusing_power(boxes: &[IndexMap<String, u32>]) -> Result<usize> {
    // One plus the box number of the lens in question.
    // The slot number of the lens within the box:
    //  - 1 for the first lens,
    //  - 2 for the second lens, and so on.
    // The focal length of the lens.
    let mut result = 0;
    for (box_num, im) in boxes.iter().enumerate() {
        for (slot_num, (_label, lens)) in im.iter().enumerate() {
            result += (1 + box_num) * (slot_num + 1) * *lens as usize;
        }
    }

    Ok(result)
}

fn part_two(input: &str) -> Result<()> {
    let timer = Instant::now();
    let mut boxes: Vec<IndexMap<String, u32>> = vec![IndexMap::<String, u32>::new(); 256];
    parse_steps(&mut boxes, input)?;
    let result = calc_focusing_power(&boxes)?;
    println!("Part Two: {} -- {:?}", &result, timer.elapsed());
    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_15.txt");
    part_one(input)?;
    part_two(input)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let result = parse_operation("rn=1").unwrap();
        println!("{:?}", result);

        let result = parse_operation("qp=3").unwrap();
        println!("{:?}", result);

        let result = parse_operation("cm=2").unwrap();
        println!("{:?}", result);
    }
}
