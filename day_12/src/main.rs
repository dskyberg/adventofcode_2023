/// Thanks to:
/// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day12.rs
use anyhow::{anyhow, Result};
use utils::parse_nums;

#[derive(Clone, Debug, PartialEq)]
struct Record<'a> {
    pattern: &'a [u8],
    conditions: Vec<usize>,
}
impl<'a> std::fmt::Display for Record<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pattern = String::from_utf8(self.pattern.to_vec()).unwrap();
        let nums = self
            .conditions
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .join(",");

        writeln!(f, "{} {}", pattern, nums)
    }
}

impl<'a> Record<'a> {
    fn parse(value: &'a str) -> Result<Self> {
        let (pattern, nums) = value
            .split_once(' ')
            .ok_or(anyhow!("Failed to parse Record"))?;
        let conditions = parse_nums::<usize>(nums, ',')?;
        Ok(Record {
            pattern: pattern.as_bytes(),
            conditions,
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Record>> {
    let mut records = vec![];
    for line in input.lines() {
        records.push(Record::parse(line)?)
    }

    Ok(records)
}

fn solve(records: &Vec<Record>) -> Result<usize> {
    let result = 0;
    let mut pattern = Vec::new();
    let mut conditions = Vec::new();
    // Exact size is not too important as long as there's enough space.
    let mut broken: Vec<usize> = vec![0; 20];
    let _table: Vec<usize> = vec![0; 2000];

    for record in records {
        println!("Record: {}", record);

        // Add a trailing '.' so that we don't have to check bounds when testing the last pattern.
        // This has no effect on the number of possible combinations.
        pattern.extend_from_slice(record.pattern);
        pattern.push(b'.');
        conditions.extend_from_slice(&record.conditions);

        // Calculate prefix sum of the number of broken springs and unknowns before each index
        // to quickly check if a range can contain a broken spring without checking every element.
        // For example `.??..??...?##` becomes `[0, 0, 1, 2, 2, 2, 3, 4, 4, 4, 4, 5, 6, 7, 7]`.
        let mut sum = 0;
        broken.push(0);

        for (i, &b) in pattern.iter().enumerate() {
            if b != b'.' {
                sum += 1;
            }
            broken[i + 1] = sum;
        }
        println!("Brokens: {:?}", &broken);

        // Determine how many spaces each pattern can slide around to speed things up.
        // We only need to check at most that many spaces for each pattern.
        let wiggle = pattern.len() - conditions.iter().sum::<usize>() - conditions.len() + 1;
        println!("Wiggle: {}", &wiggle);
    }
    Ok(result)
}

fn part_one(records: &Vec<Record>) -> Result<()> {
    let start = std::time::Instant::now();
    let result = solve(records)?;

    println!("Part One: {} -- {:?}", &result, start.elapsed());
    Ok(())
}
fn main() -> Result<()> {
    let input = include_str!("../puzzle_input.txt");
    let records = parse_input(input)?;
    part_one(&records)?;

    Ok(())
}
