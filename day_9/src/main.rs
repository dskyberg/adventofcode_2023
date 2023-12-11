use anyhow::Result;

fn part_one() -> Result<()> {
    let start = std::time::Instant::now();
    println!("Part One: -- {:?}", start.elapsed());

    Ok(())
}

fn part_two() -> Result<()> {
    let start = std::time::Instant::now();
    println!("Part Two: -- {:?}", start.elapsed());

    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_9.txt");
    let matrix = parse_input(input)?;
    println!("{:#?}", &matrix);
    part_one()?;
    part_two()?;

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Vec<isize>>> {
    let mut result: Vec<Vec<isize>> = Vec::new();
    for line in input.lines() {
        let values = line
            .split(' ')
            .map(|s| s.parse::<isize>().map_err(|e| e.into()))
            .collect::<Result<Vec<isize>>>()?;
        result.push(values);
    }

    Ok(result)
}
