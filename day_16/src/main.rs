use anyhow::Result;
use lazy_static::lazy_static;
use std::time::Instant;
const GRID_SIZE: u32 = 110;

lazy_static! {
    static ref GRID_BOUNDS: Point = Point::new((GRID_SIZE - 1) as i32, (GRID_SIZE - 1) as i32);
}

type Point = utils::Point<i32>;

#[derive(Clone, Debug, Default, PartialEq)]
enum TileType {
    #[default]
    Empty,
    LeftRight,
    RightLeft,
    HorizSplit,
    VertSplit,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '\\' => Self::LeftRight,
            '/' => Self::RightLeft,
            '|' => Self::VertSplit,
            '-' => Self::HorizSplit,
            _ => Self::Empty,
        }
    }
}

impl TileType {
    #[allow(dead_code)]
    fn to_char(&self) -> char {
        match self {
            TileType::Empty => '.',
            TileType::LeftRight => '\\',
            TileType::RightLeft => '/',
            TileType::HorizSplit => '-',
            TileType::VertSplit => '|',
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Tile {
    pub energized: bool,
    pub type_: TileType,
}

impl Tile {
    fn new(c: char) -> Self {
        Self {
            energized: false,
            type_: TileType::from(c),
        }
    }
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn get_mut(&mut self, point: &Point) -> &mut Tile {
        &mut self.0[point.y as usize][point.x as usize]
    }

    fn energized(&self) -> i32 {
        self.0.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc, t| if t.energized { acc + 1 } else { acc })
        })
    }

    #[allow(dead_code)]
    fn show_energized(&self) {
        let mut s = String::new();
        for row in &self.0 {
            let mut left = String::new();
            let mut right = String::new();
            for tile in row {
                left.push_str(&format!("{} ", tile.type_.to_char()));
                if tile.energized {
                    right.push_str("# ");
                } else {
                    right.push_str(". ");
                }
            }
            s.push_str(&format!("{}  {}\n", left, right));
        }
        println!("{}", s);
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
enum Direction {
    North,
    South,
    #[default]
    East,
    West,
}

#[derive(Clone, Debug, Default)]
struct Beam {
    direction: Direction,
    curr: Point,
}

impl Beam {
    fn new(point: &Point, direction: &Direction) -> Self {
        Self {
            direction: direction.clone(),
            curr: *point,
        }
    }
    #[inline]
    fn east_west(&self) -> bool {
        self.direction == Direction::East || self.direction == Direction::West
    }

    #[inline]
    fn north_south(&self) -> bool {
        self.direction == Direction::North || self.direction == Direction::South
    }

    fn shine(&mut self, grid: &mut Grid) -> Result<()> {
        let mut turn = false;
        // Keep moving until either an edge or a barrier is hit
        loop {
            let tile = grid.get_mut(&self.curr);

            if turn
                && tile.energized
                && !matches!(tile.type_, TileType::LeftRight | TileType::RightLeft)
            {
                // The last tile was a turn, and this tile is already engergized.
                // we're in a loop.
                return Ok(());
            }
            tile.energized = true;

            // If the beam hits a splitter, launch 2 new beams and exit the loop
            if self.east_west() && tile.type_ == TileType::VertSplit {
                if self.curr.y > 0 {
                    Beam::new(&self.curr.up(), &Direction::North).shine(grid)?;
                }
                if self.curr.y < (GRID_SIZE - 1) as i32 {
                    Beam::new(&self.curr.down(), &Direction::South).shine(grid)?;
                }
                break;
            }

            if self.north_south() && tile.type_ == TileType::HorizSplit {
                if self.curr.x > 0 {
                    Beam::new(&self.curr.left(), &Direction::West).shine(grid)?;
                }
                if self.curr.x < (GRID_SIZE - 1) as i32 {
                    Beam::new(&self.curr.right(), &Direction::East).shine(grid)?;
                }
                break;
            }

            // If the beam hits a mirror, change direction
            if tile.type_ == TileType::LeftRight {
                turn = true;
                // \
                match self.direction {
                    Direction::North => self.direction = Direction::West,
                    Direction::South => self.direction = Direction::East,
                    Direction::East => self.direction = Direction::South,
                    Direction::West => self.direction = Direction::North,
                }
            } else if tile.type_ == TileType::RightLeft {
                turn = true;
                // /
                match self.direction {
                    Direction::North => self.direction = Direction::East,
                    Direction::South => self.direction = Direction::West,
                    Direction::East => self.direction = Direction::North,
                    Direction::West => self.direction = Direction::South,
                }
            } else {
                turn = false;
            }

            let next = self.next_point();
            if next.is_none() {
                return Ok(());
            }
            let next = next.unwrap();
            self.curr = next;
        }
        Ok(())
    }

    fn next_point(&self) -> Option<Point> {
        let p = match self.direction {
            Direction::North => self.curr.up(),
            Direction::South => self.curr.down(),
            Direction::East => self.curr.right(),
            Direction::West => self.curr.left(),
        };

        // If the new point is outside the grid...
        if !p.bounded_z(&GRID_BOUNDS) {
            return None;
        }
        Some(p)
    }
}

fn part_one(input: &str) -> Result<()> {
    let timer = Instant::now();

    let mut grid = parse_input(input);
    let mut beam = Beam::default();
    beam.shine(&mut grid)?;
    let result = grid.energized();

    //grid.show_energized();

    println!("Part One: {} -- {:?}", &result, timer.elapsed());
    Ok(())
}

fn part_two(_input: &str) -> Result<()> {
    let timer = Instant::now();
    let result = 0;

    println!("Part Two: {} -- {:?}", &result, timer.elapsed());
    Ok(())
}

fn parse_input(input: &str) -> Grid {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::new).collect())
        .collect();
    Grid(grid)
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_16.txt");
    part_one(input)?;
    part_two(input)?;
    Ok(())
}
