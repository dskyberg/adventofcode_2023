use anyhow::Result;
#[derive(Debug)]
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
    let races = generate_races("not test");

    solve("One", &races)?;
    let races = vec![Race {
        time: 60808676,
        record: 601116315591300,
    }];

    solve("Two", &races)?;
    Ok(())
}

fn generate_races(mode: &str) -> Vec<Race> {
    match mode {
        "test" => vec![
            Race { time: 7, record: 9 },
            Race {
                time: 15,
                record: 40,
            },
            Race {
                time: 30,
                record: 200,
            },
        ],
        _ => vec![
            Race {
                time: 60,
                record: 601,
            },
            Race {
                time: 80,
                record: 1163,
            },
            Race {
                time: 86,
                record: 1559,
            },
            Race {
                time: 76,
                record: 1300,
            },
        ],
    }
}
