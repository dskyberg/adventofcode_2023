use anyhow::{anyhow, Result};
#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        let time = self.time as f64;
        // We are looking for winners, so increment the record distance by 1
        let distance = (self.record + 1) as f64;
        let tsq = time * time;
        let d4 = distance * 4.0;

        let lower_bound = (time - (tsq - d4).sqrt()).ceil() / 2.0;
        let upper_bound = (time + (tsq - d4).sqrt()).floor() / 2.0;
        (upper_bound.floor() - lower_bound.ceil()) as usize + 1
    }
}

fn solve(part: &str, races: &[Race]) -> Result<()> {
    let timer = std::time::Instant::now();

    let results = races
        .iter()
        .map(|race| race.ways_to_win())
        .collect::<Vec<usize>>();

    let result = results.iter().product::<usize>();

    println!("Part {}: {:?} -- {:?}", part, result, timer.elapsed());
    Ok(())
}

fn main() -> Result<()> {
    //    let races = generate_races("not test");
    let races = parse_input(include_str!("../puzzle_input.txt"))?;
    solve("One", &races)?;
    let races = vec![Race {
        time: 60808676,
        record: 601116315591300,
    }];

    solve("Two", &races)?;
    Ok(())
}

fn parse_input(puzzle_input: &str) -> Result<Vec<Race>> {
    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();
    for line in puzzle_input.lines() {
        if let Some((category, s)) = line.split_once(':') {
            match category {
                "Time" => {
                    times = s
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                }
                "Distance" => {
                    distances = s
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                }
                _ => return Err(anyhow!("Unexpected category {}", category)),
            };
        } else {
            println!("Hmmm... {}", line);
        }
    }
    let races: Vec<Race> = (0..times.len())
        .map(|idx| Race {
            time: times[idx],
            record: distances[idx],
        })
        .collect();
    Ok(races)
}
