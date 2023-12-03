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
        self.x >= x1 && self.x <= x2 && self.y >= y1 && self.y <= y2
    }
}

#[derive(Debug)]
struct Number {
    pub value: usize,
    pub start: Point,
    pub end: Point,
}

use std::cmp::max;

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
    pub location: Point,
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
        for symbol in &self.symbols {
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
}

fn read_number(data: &[u8], cursor: usize, x: usize, y: usize) -> Result<Number, String> {
    // Read until the end of the number
    let start = Point::new(x, y);
    let mut look_ahead = cursor;
    while data[look_ahead].is_ascii_digit() {
        look_ahead += 1;
    }
    let end = Point::new(look_ahead - cursor + start.x - 1, y);
    let value = std::str::from_utf8(&data[cursor..look_ahead])
        .map_err(|e| e.to_string())?
        .parse::<usize>()
        .map_err(|e| e.to_string())?;
    Ok(Number { value, start, end })
}

fn read_schematic(data: &[u8]) -> Result<Schematics, String> {
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
        let _value = data[cursor] as char;
        let location = Point::new(x, y);
        let symbol = Symbol { location };
        schematics.push_symbol(symbol);
        x += 1;
        cursor += 1;
    }
    Ok(schematics)
}

fn part_one(schematics: &Schematics) -> Result<(), String> {
    let parts = schematics.find_parts();
    let total: usize = parts.iter().sum();

    //println!("{:?}", &parts);
    println!("Total: {}", total);
    Ok(())
}

fn main() -> Result<(), String> {
    let data = include_bytes!("../../data/day_3.txt");
    let schematics = read_schematic(data)?;
    part_one(&schematics)?;
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
}
