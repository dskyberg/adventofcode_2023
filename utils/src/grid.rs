use super::{Direction, Point};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Grid<P, T> {
    width: usize,
    height: usize,
    curr: Point<P>,
    pub direction: Direction,
    pub cells: Vec<T>,
}

impl<
        P: std::fmt::Display + num::Integer + num::PrimInt + num::FromPrimitive + Sync + Send,
        T: std::fmt::Debug + Sized + Send + Sync,
    > Grid<P, T>
{
    #[inline]
    pub fn bounds(&self) -> Point<P> {
        Point::<P>::from((self.width - 1, self.height - 1))
    }

    pub fn get_at(&self, x: usize, y: usize) -> Result<&T> {
        if x >= self.width || y >= self.height {
            return Err(anyhow!("Out of bounds: {},{}", x, y));
        }
        Ok(self.cells.get(y * self.height + x).expect("WTF?"))
    }

    pub fn get_at_mut(&mut self, x: usize, y: usize) -> Result<&mut T> {
        if x >= self.width || y >= self.height {
            return Err(anyhow!("Out of bounds: {},{}", x, y));
        }
        Ok(self.cells.get_mut(y * self.height + x).expect("WTF?"))
    }
    /// Return a reference to the value at the given point
    /// If the point can't be used to index, or is not within the
    /// rid bounds, an error is returned.
    pub fn get(&self, point: &Point<P>) -> Result<&T>
    where
        usize: std::convert::TryFrom<P>,
    {
        // Will throw if x or y are less than 0
        let index = point.indexible()?;

        // Since the x,y values have already been verified to be within the
        // bounds of the grid, just expect val.
        Ok(self.get_at(index.x, index.y).expect("WTF?"))
    }

    pub fn get_mut(&mut self, point: &Point<P>) -> Result<&mut T>
    where
        usize: std::convert::TryFrom<P>,
    {
        // Will throw if x or y are less than 0
        let index = point.indexible()?;

        // Since the x,y values have already been verified to be within the
        // bounds of the grid, just expect val.
        Ok(self.get_at_mut(index.x, index.y).expect("WTF?"))
    }

    pub fn get_curr(&self) -> Result<&T>
    where
        usize: std::convert::TryFrom<P>,
    {
        self.get(&self.curr)
    }

    pub fn from_cells(cells: Vec<T>, width: usize, height: usize) -> Self {
        let curr = Point::<P>::origin();
        let direction = Direction::default();
        Self {
            width,
            height,
            cells,
            curr,
            direction,
        }
    }

    /// Reduce a new line delimited set of  PATTERN delimited strings
    /// to Vec<T>
    /// Example:
    /// ````
    /// let input = "0,1,2,3,4\n5,6,7,8,9";
    /// let convert = |s: &str| s.parse::<u32>().map_err(|e| e.into());
    /// let grid = Grid::parse_undelim_str(input,convert).expect("oops");
    /// ````
    pub fn parse_str<F>(input: &str, pattern: &str, convert: F) -> Result<Self>
    where
        F: Fn(&str) -> Result<T>,
    {
        let mut cells: Vec<T> = Vec::new();
        let mut width: usize = 0;
        let mut height: usize = 0;

        for line in input.lines() {
            height += 1;
            width = 0;
            for s in line.split(pattern) {
                width += 1;
                let t = convert(s)?;
                cells.push(t);
            }
        }
        let curr = Point::<P>::origin();
        let direction = Direction::default();
        Ok(Grid {
            width,
            height,
            cells,
            curr,
            direction,
        })
    }

    /// Reduce a new line delimited set of chars
    /// to Vec<T>
    /// Example:
    /// ````
    /// let input = "01234\n56789";
    /// let convert = |c: char| c.to_digit(10).or(anyhow!("Failed to convert"));
    /// let grid = Grid::parse_undelim_str(input,convert).expect("oops");
    /// ````
    pub fn parse_undelim_str<F>(input: &str, convert: F) -> Result<Self>
    where
        F: Fn(char) -> Result<T>,
    {
        let mut cells: Vec<T> = Vec::new();
        let mut width: usize = 0;
        let mut height: usize = 0;

        for line in input.lines() {
            height += 1;
            width = 0;
            for s in line.chars() {
                width += 1;
                let t = convert(s)?;
                cells.push(t);
            }
        }
        let curr = Point::<P>::origin();
        let direction = Direction::default();
        Ok(Grid {
            width,
            height,
            cells,
            curr,
            direction,
        })
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Copies the current position.
    pub fn current(&self) -> Point<P> {
        self.curr
    }

    /// Return all the valid neighbors for the given point
    pub fn neighbors(&self, from: &Point<P>, max: u32) -> Vec<Point<P>> {
        let mut result = vec![];

        let mut tmp_vec: Vec<Point<P>> = Vec::with_capacity(3);
        let mut tmp = *from;
        for _ in 0..max {
            if let Some(next) = self.try_step(&tmp, Direction::North) {
                tmp_vec.push(next);
            }
            tmp = tmp.up();
        }
        while let Some(tmp) = tmp_vec.pop() {
            result.push(tmp);
        }

        tmp = *from;
        for _ in 0..max {
            if let Some(next) = self.try_step(&tmp, Direction::West) {
                tmp_vec.push(next);
            }
            tmp = tmp.left();
        }
        while let Some(tmp) = tmp_vec.pop() {
            result.push(tmp);
        }

        tmp = *from;
        for _ in 0..max {
            if let Some(next) = self.try_step(&tmp, Direction::South) {
                tmp_vec.push(next);
            }
            tmp = tmp.down();
        }
        while let Some(tmp) = tmp_vec.pop() {
            result.push(tmp);
        }

        tmp = *from;
        for _ in 0..max {
            if let Some(next) = self.try_step(&tmp, Direction::East) {
                tmp_vec.push(next);
            }
            tmp = tmp.right();
        }
        while let Some(tmp) = tmp_vec.pop() {
            result.push(tmp);
        }

        result
    }

    fn try_step(&self, from: &Point<P>, direction: Direction) -> Option<Point<P>> {
        let next = from.step(direction);
        match next.bounded_z(&self.bounds()) {
            true => Some(next),
            false => None,
        }
    }

    pub fn step(&mut self) -> Option<Point<P>> {
        let next = self.curr.step(self.direction);
        match next.bounded_z(&self.bounds()) {
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
        let convert = |s: &str| s.parse::<u32>().map_err(|e| e.into());
        let pattern = ",";
        let input = r#"1,2,3,4,5
6,7,8,9,10"#;
        let result = Grid::<i32, u32>::parse_str(input, pattern, convert);
        assert!(result.is_ok());
        let mut grid = result.unwrap();
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.width(), 5);
        println!("grid.get_curr(): {:?}", grid.get_curr().unwrap());
        grid.step();
        println!("grid.get_curr(): {:?}", grid.get_curr().unwrap());
    }
}
