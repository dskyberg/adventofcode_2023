use anyhow::anyhow;

pub const ORIGIN: Point = Point { x: 0, y: 0 };

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        self.x.abs_diff(other.x) as i32 + self.y.abs_diff(other.y) as i32
    }

    #[inline]
    pub fn left(&self) -> Point {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    #[inline]
    pub fn right(&self) -> Point {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    #[inline]
    pub fn up(&self) -> Point {
        Self {
            y: self.y - 1,
            x: self.x,
        }
    }
    #[inline]
    pub fn down(&self) -> Point {
        Self {
            y: self.y + 1,
            x: self.x,
        }
    }

    #[inline]
    pub fn signum(self, other: Point) -> Point {
        Point::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }

    /// Point is within the bounds of the given min, max points
    #[inline]
    pub fn bounded(&self, min: &Point, max: &Point) -> bool {
        self.x >= min.x && self.x <= max.x && self.y >= min.y && self.y <= max.y
    }

    #[inline]
    pub fn bounded_z(&self, max: &Point) -> bool {
        self.bounded(&ORIGIN, max)
    }
}
impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(&i32, &i32)> for Point {
    fn from(value: (&i32, &i32)) -> Self {
        Point {
            x: *value.0,
            y: *value.1,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}

impl From<(&usize, &usize)> for Point {
    fn from(value: (&usize, &usize)) -> Self {
        Point {
            x: *value.0 as i32,
            y: *value.1 as i32,
        }
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Point {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}

impl From<(&u32, &u32)> for Point {
    fn from(value: (&u32, &u32)) -> Self {
        Point {
            x: *value.0 as i32,
            y: *value.1 as i32,
        }
    }
}

impl TryFrom<&str> for Point {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value
            .split_once(',')
            .ok_or(anyhow!("Failed to parse Point"))?;
        Ok(Point {
            x: x.parse::<i32>()?,
            y: y.parse::<i32>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "1,2";
        let point = Point::try_from(input);
        assert!(point.is_ok());
        assert_eq!(point.unwrap(), Point::new(1, 2));
    }

    #[test]
    fn test_bounded() {
        let test_in = Point::from((1, 2));
        let test_out1 = Point::from((0, 1));
        let test_out2 = Point::from((1, 3));
        let min = Point::from((1, 1));
        let max = Point::from((2, 2));
        assert!(test_in.bounded(&min, &max));
        assert!(!test_out1.bounded(&min, &max));
        assert!(!test_out2.bounded(&min, &max));
    }
}
