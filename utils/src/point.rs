use anyhow::anyhow;
use num::Integer;
use std::cmp::{max, min};

#[derive(Debug, Clone, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Integer + PartialOrd + Ord + Eq + Sized + Send + Sync + Copy + num::FromPrimitive>
    Point<T>
{
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Returns the origin point `Point{x: 0, y: 0}`
    #[inline]
    pub fn origin() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &Self) -> T {
        let x_diff = max(self.x, other.x) - min(self.x, other.x);
        let y_diff = max(self.y, other.y) - min(self.y, other.y);
        x_diff + y_diff
    }

    /// Returns true if self is a point wihtout the bounds of the provided min, max points
    #[inline]
    pub fn bounded(&self, min: &Self, max: &Self) -> bool {
        self.x >= min.x && self.x <= max.x && self.y >= min.y && self.y <= max.y
    }

    /// Returns true if self is within the bounds of ORIGIN and max
    #[inline]
    pub fn bounded_z(&self, max: &Self) -> bool {
        self.bounded(&Self::origin(), max)
    }

    /// Returns the point left of self
    #[inline]
    pub fn left(&self) -> Self {
        Self {
            x: self.x - T::from_i32(1).unwrap(),
            y: self.y,
        }
    }

    #[inline]
    pub fn right(&self) -> Self {
        Self {
            x: self.x + T::from_i32(1).unwrap(),
            y: self.y,
        }
    }

    #[inline]
    pub fn up(&self) -> Self {
        Self {
            y: self.y - T::from_i32(1).unwrap(),
            x: self.x,
        }
    }
    #[inline]
    pub fn down(&self) -> Self {
        Self {
            y: self.y + T::from_i32(1).unwrap(),
            x: self.x,
        }
    }
}

impl<T: Copy> From<(&T, &T)> for Point<T> {
    fn from(value: (&T, &T)) -> Self {
        Self {
            x: *value.0,
            y: *value.1,
        }
    }
}

impl<T: Eq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: num::Num + Send + Sync> TryFrom<&str> for Point<T> {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value
            .split_once(',')
            .ok_or(anyhow!("Failed to parse Point"))?;
        Ok(Self {
            x: num::Num::from_str_radix(x, 10).map_err(|_| anyhow!("Failed to parse x"))?,
            y: num::Num::from_str_radix(y, 10).map_err(|_| anyhow!("Failed to parse x"))?,
        })
    }
}

impl<T: num::FromPrimitive> From<(usize, usize)> for Point<T> {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: T::from_usize(value.0).unwrap(),
            y: T::from_usize(value.1).unwrap(),
        }
    }
}

impl<T: num::FromPrimitive> From<(isize, isize)> for Point<T> {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: T::from_isize(value.0).unwrap(),
            y: T::from_isize(value.1).unwrap(),
        }
    }
}

impl<T: num::FromPrimitive> From<(u32, u32)> for Point<T> {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: T::from_u32(value.0).unwrap(),
            y: T::from_u32(value.1).unwrap(),
        }
    }
}

impl<T: num::FromPrimitive> From<(i32, i32)> for Point<T> {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: T::from_i32(value.0).unwrap(),
            y: T::from_i32(value.1).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "1,2";
        let point = Point::<i32>::try_from(input);
        assert!(point.is_ok());
        assert_eq!(point.unwrap(), Point::<i32>::new(1, 2));
    }

    #[test]
    fn test_bounded() {
        let test_in = Point::<i32>::from((1, 2));
        let test_out1 = Point::from((0, 1));
        let test_out2 = Point::from((1, 3));
        let min = Point::from((1, 1));
        let max = Point::from((2, 2));
        assert!(test_in.bounded(&min, &max));
        assert!(!test_out1.bounded(&min, &max));
        assert!(!test_out2.bounded(&min, &max));
    }

    #[test]
    fn test_bounded_t_i32() {
        let test_in = Point::<i32>::from((1, 2));
        let test_out1 = Point::<i32>::from((0, 1));
        let test_out2 = Point::<i32>::from((1, 3));
        let min = Point::<i32>::from((1, 1));
        let max = Point::<i32>::from((2, 2));
        assert!(test_in.bounded(&min, &max));
        assert!(!test_out1.bounded(&min, &max));
        assert!(!test_out2.bounded(&min, &max));
    }

    #[test]
    fn test_bounded_t_usize() {
        let test_in = Point::<usize>::from((1, 2));
        let test_out1 = Point::<usize>::from((0, 1));
        let test_out2 = Point::<usize>::from((1, 3));
        let min = Point::<usize>::from((1, 1));
        let max = Point::<usize>::from((2, 2));
        assert!(test_in.bounded(&min, &max));
        assert!(!test_out1.bounded(&min, &max));
        assert!(!test_out2.bounded(&min, &max));
    }

    #[test]
    fn test_manhattan_t_usize() {
        let p1 = Point::<usize>::from((1, 1));
        let p2 = Point::<usize>::from((2, 3));

        assert_eq!(p1.manhattan_distance(&p2), 3usize);
    }

    #[test]
    fn test_froms() {
        let x = 1;
        let y = 1;
        let p = Point::<i32>::from((&x, &y));
        assert_eq!(p, Point::<i32> { x: 1, y: 1 });

        let p = Point::<i32>::try_from("1,2");
        assert!(p.is_ok());
        assert_eq!(p.unwrap(), Point::<i32> { x: 1, y: 2 });

        let p = Point::<usize>::from((x, y));
    }
}
