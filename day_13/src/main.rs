use anyhow::Result;

/// Transpose rows and cols
/// Example
fn pivot(value: &[String]) -> Vec<String> {
    let rows = value.len();
    let cols = value[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| value[row][col].clone()).collect())
        .collect::<Vec<Vec<T>>>()
}

fn reduce(input: &str) -> u32 {
    let len = input.len() as u32 - 1;
    input
        .as_bytes()
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, c)| match c {
            b'#' => acc + 2_u32.pow(len - idx as u32),
            _ => acc,
        })
}

fn part_one(input: &str) -> Result<()> {
    let timer = std::time::Instant::now();
    let result = 0;

    let mut rows: Vec<u32> = vec![0; 20];
    let mut cols: Vec<u32> = vec![0; 20];
    let mut x: usize = 0;
    let mut y: usize = 0;

    // Accumulate rows, for pivoting
    let mut acc: Vec<&[u8]> = vec![];

    for line in input.lines() {
        // Accumulate for pivoting
        let bytes = line.as_bytes();
        rows[y] = reduce(line);
        y += 1;
        acc.push(bytes);

        if line.is_empty() {
            let pivots = pivot(&acc);
            for p in pivots {
                cols[x] = reduce(&p);
                x += 1;
            }
        }
    }

    println!("Part One: {} -- {:?}", result, timer.elapsed());
    Ok(())
}
fn main() -> Result<()> {
    let input = include_str!("../../data/day_13.txt");
    part_one(input)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let input = "..#.#".as_bytes();
        let len = input.len() as u32 - 1;
        let this = input.chars().enumerate().fold(0, |acc, (idx, c)| match c {
            '#' => acc + 2_u32.pow(len - idx as u32),
            _ => acc,
        });
        assert_eq!(this, 0b00101u32);
    }
}
