use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Map {
    pub instructions: Vec<char>,
    pub nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn navigate(&self, start: &str, end: &str) -> Result<usize> {
        let target_node_names = self.ends_with(start);
        let mut node_steps: Vec<usize> = Vec::new();

        for target_node in &target_node_names {
            let mut steps = 0;
            let mut instructions_iter = self.instructions.iter().cycle();
            let mut next_node_name = target_node;
            while !next_node_name.ends_with(end) {
                steps += 1;
                let curr_node = self.nodes.get(next_node_name).unwrap();
                let next_inst = instructions_iter.next().unwrap();
                next_node_name = match next_inst {
                    'L' => &curr_node.0,
                    'R' => &curr_node.1,
                    _ => unimplemented!(),
                };
            }
            node_steps.push(steps);
        }
        let lcm = least_common_multiple(&node_steps);
        Ok(lcm)
    }

    fn ends_with(&self, value: &str) -> Vec<String> {
        let nodes: Vec<String> = self
            .nodes
            .iter()
            .filter(|(key, _val)| key.ends_with(value))
            .map(|(x, _)| x.to_string())
            .collect();

        nodes
    }
}

fn least_common_multiple(nums: &[usize]) -> usize {
    let mut result = 1;
    for &num in nums {
        result = num * result / gcd(num, result);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn part_one(input: &str) -> Result<()> {
    let map = parse_input(input)?;
    let start = std::time::Instant::now();
    let steps = map.navigate("AAA", "ZZZ")?;
    println!("Part One: {} -- {:?}", steps, start.elapsed());
    Ok(())
}

fn part_two(input: &str) -> Result<()> {
    let map = parse_input(input)?;
    let steps = map.navigate("A", "Z")?;
    let start = std::time::Instant::now();
    println!("Part Two: {} -- {:?}", &steps, start.elapsed());
    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_8.txt");

    part_one(input)?;
    part_two(input)?;
    Ok(())
}

fn parse_input(input: &str) -> Result<Map> {
    let mut got_instructions = false;
    let mut skip = false;
    let mut map = Map::default();

    let re = Regex::new(r"^([\dA-Z]+) = \(([\dA-Z]+), ([\dA-Z]+)\)$").unwrap();
    for line in input.lines() {
        if !got_instructions {
            map.instructions = line.chars().collect::<Vec<char>>();
            got_instructions = true;
            continue;
        }
        if got_instructions && !skip {
            skip = true;
            continue;
        }
        let captures = re.captures(line).unwrap();
        assert_eq!(captures.len(), 4);

        let node = captures[1].to_string();
        let left = captures[2].to_string();
        let right = captures[3].to_string();
        if map.nodes.contains_key(&node) {
            return Err(anyhow!("node exists"));
        }
        map.nodes.insert(node, (left, right));
    }

    Ok(map)
}
