use anyhow::Result;

fn part_one(_puzzle_input: &str) -> Result<()> {
    let timer = std::time::Instant::now();
    let result = 0;

    println!("Part One: {}  -- {:?}", result, timer.elapsed());
    Ok(())
}
fn main() -> Result<()> {
    let puzzle_input = include_str!("../../data/day_17.txt");
    part_one(puzzle_input)?;
    Ok(())
}
