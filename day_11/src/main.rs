/// Thanks to https://github.com/clearlyMine/advent_rust/blob/main/year_2023/src/bin/day11.rs
/// for the clue on folding the coordinates.
use std::collections::HashMap;

use anyhow::Result;
use utils::Point;

#[derive(Debug, Clone)]
struct Universe {
    pub galaxies: Vec<Point>,
    pub max_x: usize,
    pub max_y: usize,
}

impl Universe {
    fn total_path_lens(&self) -> usize {
        self.galaxies
            .iter()
            .enumerate()
            .fold(0, |mut acc, (i, gal)| {
                for next_gal in &self.galaxies[i + 1..] {
                    acc += gal.manhattan_distance(next_gal);
                }
                acc
            })
    }
}
impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut points: Vec<Vec<char>> = vec![vec!['.'; self.max_x + 1]; self.max_y + 1];
        for g in &self.galaxies {
            points[g.y][g.x] = '#';
        }

        for row in points {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

/// Count the steps between each pair
/// Return the total steps
fn exec(part: &str, bytes: &[u8], expansion_factor: usize) -> Result<()> {
    let start = std::time::Instant::now();
    let universe = parse_input(bytes, expansion_factor)?;

    let result = universe.total_path_lens();
    println!("Part {}: {}: {:?}", part, &result, start.elapsed());
    Ok(())
}

fn main() -> Result<()> {
    let bytes = include_bytes!("../../data/day_11.txt");
    exec("One", bytes, 2)?;
    exec("Two", bytes, 1_000_000)?;
    Ok(())
}

/// Expand the given universe based on empty rows and cols.
/// This is called by [parse_input]
fn expand(
    universe: Universe,
    rows: &HashMap<usize, usize>,
    cols: &HashMap<usize, usize>,
    expansion_factor: usize,
) -> Result<Universe> {
    let mut row_map: HashMap<usize, usize> = HashMap::new();
    let mut col_map: HashMap<usize, usize> = HashMap::new();
    let mut max_y: usize = universe.max_y;
    let mut max_x = universe.max_x;

    // Create row col maps to manage the offsets
    let mut offset: usize = 0;
    for row in 0..=universe.max_y {
        if rows.contains_key(&row) {
            row_map.insert(row, row + offset * (expansion_factor - 1));
        } else {
            offset += 1;
            max_y += 1;
        }
    }
    offset = 0;
    for col in 0..=universe.max_x {
        if cols.contains_key(&col) {
            col_map.insert(col, col + offset * (expansion_factor - 1));
        } else {
            offset += 1;
            max_x += 1;
        }
    }

    // Map the Point x and y to the row_col maps
    let mut galaxies: Vec<Point> = Vec::new();
    for g in &universe.galaxies {
        let x = col_map.get(&g.x).unwrap();
        let y = row_map.get(&g.y).unwrap();
        galaxies.push((x, y).into());
    }

    // Return the expanded universe
    Ok(Universe {
        galaxies,
        max_x,
        max_y,
    })
}

/// Parse the input to a Universe, and then expand it
/// Returns the expanded universe.
fn parse_input(input: &[u8], expansion_factor: usize) -> Result<Universe> {
    let mut galaxies: Vec<Point> = Vec::new();
    let mut rows: HashMap<usize, usize> = HashMap::new();
    let mut cols: HashMap<usize, usize> = HashMap::new();

    let mut x: usize = 0;
    let mut y: usize = 0;

    for b in input {
        match *b {
            b'\n' => {
                x = 0;
                y += 1;
                continue;
            }
            b'#' => {
                galaxies.push((x, y).into());
                let v = rows.entry(y).or_default();
                *v += 1;
                let v = cols.entry(x).or_default();
                *v += 1;
                x += 1;
            }
            _ => {
                x += 1;
            }
        }
    }

    let max_y = y;
    let max_x: usize = (input.len() - y) / (y + 1) - 1;

    let u = Universe {
        galaxies,
        max_x,
        max_y,
    };

    expand(u, &rows, &cols, expansion_factor)
}
