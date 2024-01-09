use anyhow::Result;
use ranges::*;
//mod brute::*;
mod ranges;

/// While I did succeed at developing a brute force method, I failed at
/// developing an elegant, range based solution.  I stole it.
fn main() -> Result<()> {
    let input = include_str!("../puzzle_input.txt");

    part_one(input)?;
    part_two(input)?;
    Ok(())
}
