use anyhow::Result;

fn reduce(input: &[u8]) -> u32 {
    let len = input.len() as u32 - 1;
    input.iter().enumerate().fold(0, |acc, (idx, c)| match c {
        b'#' => acc + 2_u32.pow(len - idx as u32),
        _ => acc,
    })
}

///Find the point at which rows reflect
fn line_of_reflection(values: &[u32]) -> Option<u32> {
    for i in 1..values.len() {
        let j = i - 1;
        if values[j] == values[i] {
            if j == 0 || i == values.len() - 1 {
                // Line
                return Some(i as u32);
            }
            let mut offset = 1;
            let mut matched = true;
            loop {
                if values[j - offset] != values[i + offset] {
                    matched = false;
                    break;
                }
                if j - offset == 0 {
                    break;
                }
                if i + offset == values.len() - 1 {
                    break;
                }
                offset += 1;
            }
            if matched {
                return Some(i as u32);
            }
        }
    }
    None
}

fn process_grid(grid: &[&[u8]]) -> u32 {
    let mut result = 0;
    let mut row_results: Vec<u32> = Vec::new();
    let mut col_results: Vec<u32> = Vec::new();

    for y in 0..grid.len() {
        row_results.push(reduce(grid[y]));
    }
    // For each column
    for x in 0..grid[0].len() {
        // Calculate the value
        col_results.push((0..grid.len()).fold(0, |acc, y| {
            if grid[y][x] == b'#' {
                acc + 2u32.pow((grid.len() - 1 - y) as u32)
            } else {
                acc
            }
        }));
    }

    if let Some(horizontal_line) = line_of_reflection(&row_results) {
        result += horizontal_line * 100;
    } else if let Some(vertical_line) = line_of_reflection(&col_results) {
        result += vertical_line;
    } else {
        eprintln!("No line of reflection found");
    }
    result
}

fn part_one(input: &str) -> Result<u32> {
    let timer = std::time::Instant::now();
    let mut result = 0;

    let mut grid: Vec<&[u8]> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            // End of a set of lines.  Process and cler the grid.
            result += process_grid(&grid);
            grid.clear();
            continue;
        }
        grid.push(line.as_bytes());
    }
    if !grid.is_empty() {
        // Process the last set of lines
        result += process_grid(&grid);
    }

    println!("Part One: {} -- {:?}", result, timer.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_13.txt");
    part_one(input)?;
    Ok(())
}
