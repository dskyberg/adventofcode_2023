use crate::direction;

use super::{Direction, Point};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Grid<P, T> {
    pub bounds: Point<P>,
    pub curr: Point<P>,
    pub direction: Direction,
    pub cells: Vec<Vec<T>>,
}

impl<
        P: num::Integer + PartialOrd + Ord + Eq + Sized + Send + Sync + Copy + num::FromPrimitive,
        T: std::fmt::Debug + Sized + Send + Sync,
    > Grid<P, T>
{
    pub fn parse<F>(input: &str, convert: F) -> Result<Self>
    where
        F: Fn(&str) -> Result<T>,
    {
        let mut cells: Vec<Vec<T>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<T> = Vec::new();
            for s in line.split(',') {
                let t = convert(s)?;
                row.push(t);
            }
            cells.push(row);
        }
        let bounds = Point::<P>::from((cells[0].len(), cells.len()));
        let curr = Point::<P>::origin();
        let direction = Direction::default();
        Ok(Grid {
            bounds,
            cells,
            curr,
            direction,
        })
    }

    pub fn step(&mut self) -> Option<Point<P>> {
        let next = self.curr.step(self.direction);
        match next.bounded_z(&self.bounds) {
            true => {
                self.curr = next;
                Some(next)
            }
            false => None,
        }
    }

    pub fn left(&mut self) -> Option<Point<P>> {
        self.direction = Direction::West;
        self.step()
    }

    pub fn right(&mut self) -> Option<Point<P>> {
        self.direction = Direction::East;
        self.step()
    }

    pub fn up(&mut self) -> Option<Point<P>> {
        self.direction = Direction::North;
        self.step()
    }

    pub fn down(&mut self) -> Option<Point<P>> {
        self.direction = Direction::South;
        self.step()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let convert = |s: &str| s.parse::<u32>().map_err(|e| anyhow!("{}", e.to_string()));
        let input = r#"1,2,3,4,5
6,7,8,9,10"#;
        let result = Grid::<i32, u32>::parse(input, convert);
        assert!(result.is_ok());
        dbg!(result.unwrap());
    }
}
