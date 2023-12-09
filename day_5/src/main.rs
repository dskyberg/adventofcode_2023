use ranges::*;
//mod brute::*;
mod ranges;

/// While I did succeed at developing a brute force method, I failed at
/// developing an elegant, range based solution.  I stole it.
fn main() -> Result<(), String> {
    let input = include_str!("../../data/day_5.txt");
    // Answer: 331445006
    part_one(input)?;
    // Answer: 6472060
    part_two(input)?;
    Ok(())
}
