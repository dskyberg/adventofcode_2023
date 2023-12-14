/// Create an inclusive range that is row +/- 1, handling negatives.
/// Example: r
/// for row == 0: 0..=1
/// for row == 1: 0..=2
/// for row == 2: 1..3
use anyhow::{anyhow, Result};

fn usize_range(row: usize) -> std::ops::RangeInclusive<usize> {
    let lower = max(row as isize - 1, 0) as usize;
    lower..=row + 1
}

#[derive(Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn within_bounds(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        (x1..=x2).contains(&self.x) && (y1..=y2).contains(&self.y)
        //self.x >= x1 && self.x <= x2 && self.y >= y1 && self.y <= y2
    }

    /// Generate an inclusive range of self.y +/- 1
    /// This is used to filter the Vec of symbols and numbers later
    pub fn y_range(&self) -> RangeInclusive<usize> {
        usize_range(self.y)
    }
}

#[derive(Debug)]
struct Number {
    pub value: usize,
    pub start: Point,
    pub end: Point,
}

use std::{cmp::max, ops::RangeInclusive};

impl Number {
    /// Effectively returns the number of digits in this number.
    fn len(&self) -> usize {
        self.end.x - self.start.x + 1
    }
    /// Given a space around a number, see if the point
    /// is in the box
    fn adjacent(&self, point: &Point) -> bool {
        let x1 = max(self.start.x as isize - 1, 0) as usize;
        let y1 = max(self.start.y as isize - 1, 0) as usize;
        let x2 = self.end.x + 1;
        let y2 = self.end.y + 1;
        point.within_bounds(x1, y1, x2, y2)
    }
}

#[derive(Debug)]
struct Symbol {
    pub value: char,
    pub location: Point,
}

impl Symbol {
    fn is_gear(&self) -> bool {
        self.value == '*'
    }
}

#[derive(Debug)]
struct Schematics {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Schematics {
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
            symbols: Vec::new(),
        }
    }

    pub fn push_number(&mut self, number: Number) {
        self.numbers.push(number);
    }
    pub fn push_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn symbol_adjacent(&self, number: &Number) -> bool {
        for symbol in self
            .symbols
            .iter()
            .filter(|s| number.start.y_range().contains(&s.location.y))
        {
            if number.adjacent(&symbol.location) {
                return true;
            }
        }
        false
    }

    fn find_parts(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        for number in &self.numbers {
            if self.symbol_adjacent(number) {
                result.push(number.value);
            }
        }
        result
    }

    /// See if 2 numbers touch a gear
    fn find_gears(&self) -> Result<Vec<Vec<usize>>> {
        let mut number_pairs: Vec<Vec<usize>> = Vec::new();
        for symbol in self.symbols.iter().filter(|s| s.is_gear()) {
            let mut numbers: Vec<usize> = Vec::new();
            for number in self
                .numbers
                .iter()
                .filter(|number| symbol.location.y_range().contains(&number.start.y))
            {
                if number.adjacent(&symbol.location) {
                    numbers.push(number.value)
                }
            }
            if numbers.len() == 2 {
                number_pairs.push(numbers);
            }
        }
        Ok(number_pairs)
    }
}

fn read_number(data: &[u8], cursor: usize, x: usize, y: usize) -> Result<Number> {
    // Read until the end of the number
    let start = Point::new(x, y);
    let mut look_ahead = cursor;
    while data[look_ahead].is_ascii_digit() {
        look_ahead += 1;
    }
    let end = Point::new(look_ahead - cursor + start.x - 1, y);
    let value = std::str::from_utf8(&data[cursor..look_ahead])?.parse::<usize>()?;
    Ok(Number { value, start, end })
}

fn read_schematic(data: &[u8]) -> Result<Schematics> {
    let mut schematics = Schematics::new();

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut cursor = 0;
    while cursor < data.len() {
        if data[cursor] == b'\n' {
            y += 1;
            x = 0;
            cursor += 1;
            continue;
        }

        if data[cursor] == b'.' {
            x += 1;
            cursor += 1;
            continue;
        }

        if data[cursor].is_ascii_digit() {
            let number = read_number(data, cursor, x, y)?;
            cursor += number.len();
            x += number.len();
            schematics.push_number(number);
            continue;
        }
        // Looks like a symbol
        let value = data[cursor] as char;
        let location = Point::new(x, y);
        let symbol = Symbol { value, location };
        schematics.push_symbol(symbol);
        x += 1;
        cursor += 1;
    }
    Ok(schematics)
}

fn part_one(schematics: &Schematics) -> Result<()> {
    let parts = schematics.find_parts();
    let total: usize = parts.iter().sum();

    println!("Part One: {}", total);
    Ok(())
}

fn part_two(schematics: &Schematics) -> Result<()> {
    let gears = schematics.find_gears()?;
    let mut total = 0;
    for pair in gears {
        if pair.len() != 2 {
            return Err(anyhow!("Wrong number of gears"));
        }
        total += pair[0] * pair[1];
    }
    println!("Part Two: {}", total);
    Ok(())
}

fn main() -> Result<()> {
    let data = include_bytes!("../../data/day_3.txt");
    let schematics = read_schematic(data)?;
    part_one(&schematics)?;
    part_two(&schematics)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let data = "467$..114..".as_bytes();
        let schematic = read_schematic(data).expect("Fail");
        println!("{:?}", &schematic);
    }

    #[test]
    fn test_row_range() {
        assert_eq!(usize_range(0), 0..=1);
        assert_eq!(usize_range(1), 0..=2);
        assert_eq!(usize_range(2), 1..=3);
    }
}
