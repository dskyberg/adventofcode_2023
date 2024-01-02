//! Generic 2D coordinate.
//! If you want cartesian points, use Point<i32>.  If you want simple row,col grid coordinates, use Point<u32>.
//! If you need very large cartesian or grid coordinates, use Point<isize> or Point<usize>.  And of course, if you have
//! very small grid needs, you can use  Point<u8>.
//!
//! Typical aritimetic is supported
//!
//! ```rust
//! let lhs = Point::<i32>::new(1,1);
//! let rhs = Point::<i32>::new(2,2);
//! let result = lhs + rhs;
//! assert_eq!(&result, &Point::<i32>::new(3,3));
//! ```
//!
//! Test whether a point is bounded
//!
//! ```rust
//! let upper = Point::<i32>::origin();
//! let lower = Point::<i32>::new(10,10);
//! let inside = Point::<i32>::new(4,5);
//! assert!(inside.bounded(&upper, &lower));
//! ```
//!
//! The above can be simplified with [bounded_z]
//!
//! ```rust
//! let lower = Point::<i32>::new(10,10);
//! let inside = Point::<i32>::new(4,5);
//! assert!(inside.bounded_z(&lower));
//!
//! Find the distance between 2 points:
//! ```rust
//!  let p1 = Point::<i32>::from((1, 1));
//!  let p2 = Point::<i32>::from((2, 3));
//!  assert_eq!(p1.manhattan_distance(&p2), 3);
//!
//! Parse points from strings:
//!
//! ```rust
//!   let input = "1,2";
//!   let point = Point::<i32>::try_from(input);
//!   assert!(point.is_ok());
//!   assert_eq!(point.unwrap(), Point::<i32>::new(1, 2));

use crate::Direction;
use anyhow::{anyhow, Result};
use num::Integer;
use std::cmp::{max, min};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<
        T: Integer
            + num::PrimInt
            + PartialOrd
            + Ord
            + Eq
            + Sized
            + Send
            + Sync
            + Copy
            + num::FromPrimitive,
    > Default for Point<T>
{
    /// The default for Point is the origin
    fn default() -> Self {
        Self::origin()
    }
}

impl<T: Integer + PartialOrd + Ord + Eq + Sized + Send + Sync + Copy + num::FromPrimitive>
    Point<T>
{
    /// Create a new point with the provied X and Y values.
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

    /// returns the [Manhattan Distance](https://simple.wikipedia.org/wiki/Manhattan_distance) between two points.
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

    /// Returns the point left of self - self.x - 1
    #[inline]
    pub fn left(&self) -> Self {
        Self {
            x: self.x - T::one(),
            y: self.y,
        }
    }

    /// Returns the point right of self - self.x + 1
    #[inline]
    pub fn right(&self) -> Self {
        Self {
            x: self.x + T::one(),
            y: self.y,
        }
    }

    /// Returns the point above self - self.y - 1
    #[inline]
    pub fn up(&self) -> Self {
        Self {
            y: self.y - T::one(),
            x: self.x,
        }
    }

    /// Returns the point below self - self.y + 1
    #[inline]
    pub fn down(&self) -> Self {
        Self {
            y: self.y + T::one(),
            x: self.x,
        }
    }

    /// Returns the point, stepping one point in the direction provided.
    #[inline]
    pub fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::East => self.right(),
            Direction::West => self.left(),
            Direction::North => self.up(),
            Direction::South => self.down(),
        }
    }
}

impl<T: Copy> Point<T> {
    pub fn indexible(&self) -> Result<Point<usize>>
    where
        usize: std::convert::TryFrom<T>,
    {
        let x: usize = self
            .x
            .try_into()
            .map_err(|_| anyhow!("Failed to convert x to usize"))?;
        let y: usize = self
            .y
            .try_into()
            .map_err(|_| anyhow!("Failed to convert y to usize"))?;

        Ok(Point::<usize>::from((x, y)))
    }
}

impl<
        T: Integer
            + PartialOrd
            + Ord
            + Eq
            + Sized
            + Send
            + Sync
            + Copy
            + num::FromPrimitive
            + num::Signed,
    > Point<T>
{
    #[inline]
    pub fn signum(self, other: Self) -> Self {
        Self::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }
}

impl<
        T: Integer + PartialOrd + Ord + Eq + Sized + Send + Sync + Copy + num::FromPrimitive + Add,
    > Add for Point<T>
{
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<
        T: Integer
            + PartialOrd
            + Ord
            + Eq
            + Sized
            + Send
            + Sync
            + Copy
            + num::FromPrimitive
            + AddAssign,
    > AddAssign for Point<T>
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<
        T: Integer + PartialOrd + Ord + Eq + Sized + Send + Sync + Copy + num::FromPrimitive + Mul,
    > Mul<T> for Point<T>
{
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: T) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl<
        T: Integer
            + PartialOrd
            + Ord
            + Eq
            + Sized
            + Send
            + Sync
            + Copy
            + num::FromPrimitive
            + num::CheckedSub
            + std::default::Default,
    > Sub for Point<T>
{
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        let x = self.x.checked_sub(&rhs.x).unwrap_or_default();
        let y = self.y.checked_sub(&rhs.y).unwrap_or_default();
        Self::new(x, y)
    }
}

impl<
        T: Integer
            + PartialOrd
            + Ord
            + Eq
            + Sized
            + Send
            + Sync
            + Copy
            + num::FromPrimitive
            + num::CheckedSub
            + std::default::Default,
    > SubAssign for Point<T>
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x.checked_sub(&rhs.x).unwrap_or_default();
        self.y = self.y.checked_sub(&rhs.y).unwrap_or_default();
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

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", &self.x, &self.y)
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
        let p1 = Point::<i32>::from((1, 1));
        let p2 = Point::<i32>::from((2, 3));

        assert_eq!(p1.manhattan_distance(&p2), 3);
    }

    #[test]
    fn test_froms() {
        let x = 1;
        let y = 1;
        let p = Point::<i32>::from((x, y));
        assert_eq!(p, Point::<i32> { x: 1, y: 1 });

        let p = Point::<i32>::try_from("1,2");
        assert!(p.is_ok());
        assert_eq!(p.unwrap(), Point::<i32> { x: 1, y: 2 });
    }

    #[test]
    fn test_add() {
        let lhs = Point::<i32>::new(1, 1);
        let rhs = Point::<i32>::new(2, 2);
        let result = lhs + rhs;
        assert_eq!(result, Point::<i32>::new(3, 3));

        let mut lhs = Point::<i32>::new(1, 1);
        lhs += rhs;
        assert_eq!(lhs, Point::<i32>::new(3, 3));
        println!("Can still borrow lhs: {lhs}, and rhs: {rhs}");
    }

    #[test]
    fn test_sub() {
        let lhs = Point::<i32>::new(1, 1);
        let rhs = Point::<i32>::new(2, 2);
        let result = lhs - rhs;
        assert_eq!(result, Point::<i32>::new(-1, -1));

        let mut lhs = Point::<i32>::new(1, 1);
        lhs -= rhs;
        assert_eq!(lhs, Point::<i32>::new(-1, -1));
        println!("Can still borrow lhs: {lhs}, and rhs: {rhs}");
    }

    #[test]
    fn test_checked_sub() {
        let lhs = Point::<u32>::new(1, 1);
        let rhs = Point::<u32>::new(2, 2);
        let result = lhs - rhs;
        assert_eq!(result, Point::<u32>::new(0, 0));

        let mut lhs = Point::<u32>::new(1, 1);
        lhs -= rhs;
        assert_eq!(lhs, Point::<u32>::new(0, 0));
        println!("Can still borrow lhs: {lhs}, and rhs: {rhs}");
    }
}
