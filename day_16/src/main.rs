/// Thanks to https://github.com/clearlyMine for the hints on tracking visited cells
use anyhow::Result;
use lazy_static::lazy_static;
use std::{collections::HashSet, time::Instant};
use utils::Direction;
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

fn shine_beam(
    grid: &[Vec<TileType>],
    initial_position: Point,
    initial_direction: Direction,
) -> Result<u32> {
    let mut queue: Vec<(Point, Direction)> = vec![(initial_position, initial_direction)];

    let mut visited: HashSet<(Point, Direction)> = HashSet::new();
    visited.insert((initial_position, initial_direction));

    while let Some((position, direction)) = queue.pop() {
        let new_directions: Vec<Direction> = match grid[position.y as usize][position.x as usize] {
            TileType::LeftRight => match direction {
                // \
                Direction::East => vec![Direction::South],
                Direction::West => vec![Direction::North],
                Direction::North => vec![Direction::West],
                Direction::South => vec![Direction::East],
            },
            TileType::RightLeft => match direction {
                // /
                Direction::East => vec![Direction::North],
                Direction::West => vec![Direction::South],
                Direction::North => vec![Direction::East],
                Direction::South => vec![Direction::West],
            },
            TileType::HorizSplit => match direction {
                Direction::North | Direction::South => vec![Direction::East, Direction::West],
                _ => vec![direction],
            },
            TileType::VertSplit => match direction {
                Direction::East | Direction::West => vec![Direction::North, Direction::South],
                _ => vec![direction],
            },
            TileType::Empty => vec![direction],
        };

        for direction in new_directions {
            // Move to the next point, based on the direction
            let new_position = position.step(direction);
            // If the point is in bounds and hasn't been  visited...
            if new_position.bounded_z(&GRID_BOUNDS) && !visited.contains(&(new_position, direction))
            {
                visited.insert((new_position, direction));
                queue.push((new_position, direction));
            }
        }
    }

    let visited_points = visited.iter().map(|(p, _)| *p).collect::<HashSet<Point>>();
    //show_energized(&grid, &visited_points);

    Ok(visited_points.len() as u32)
}

fn part_one(grid: &[Vec<TileType>]) -> Result<()> {
    let timer = Instant::now();

    let position = Point::origin();
    let direction = Direction::East;

    let result = shine_beam(grid, position, direction)?;

    println!("Part One: {} -- {:?}", &result, timer.elapsed());
    Ok(())
}

fn part_two(grid: &[Vec<TileType>]) -> Result<()> {
    let timer = Instant::now();
    let mut result = 0;

    for i in 0..GRID_SIZE as i32 {
        // Any point on first row
        result = std::cmp::max(
            result,
            shine_beam(grid, Point::new(i, 0), Direction::South)?,
        );

        // Any point on first col
        result = std::cmp::max(result, shine_beam(grid, Point::new(0, i), Direction::East)?);
    }

    for i in (0..GRID_SIZE as i32).rev() {
        // Any point on last row
        result = std::cmp::max(
            result,
            shine_beam(grid, Point::new(i, 0), Direction::North)?,
        );

        // Any point on last col
        result = std::cmp::max(result, shine_beam(grid, Point::new(0, i), Direction::West)?);
    }
    println!("Part Two: {} -- {:?}", &result, timer.elapsed());
    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_16.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().map(TileType::from).collect::<Vec<TileType>>())
        .collect::<Vec<Vec<TileType>>>();

    part_one(&grid)?;
    part_two(&grid)?;
    Ok(())
}
