#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(&usize, &usize)> for Point {
    fn from(value: (&usize, &usize)) -> Self {
        Point {
            x: *value.0,
            y: *value.1,
        }
    }
}

//-----------------

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point32 {
    pub x: u32,
    pub y: u32,
}

impl Point32 {
    pub fn manhattan_distance(&self, other: &Point32) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
impl From<(u32, u32)> for Point32 {
    fn from(value: (u32, u32)) -> Self {
        Point32 {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<(&u32, &u32)> for Point32 {
    fn from(value: (&u32, &u32)) -> Self {
        Point32 {
            x: *value.0,
            y: *value.1,
        }
    }
}

impl From<(usize, usize)> for Point32 {
    fn from(value: (usize, usize)) -> Self {
        Point32 {
            x: value.0 as u32,
            y: value.1 as u32,
        }
    }
}
