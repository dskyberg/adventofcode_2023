use anyhow::Result;
use std::time::Instant;

const GRID_SIZE: usize = 100;

#[derive(Debug, Clone, Copy, Default)]
enum Direction {
    #[default]
    North,
    West,
    South,
    East,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum RockType {
    #[default]
    None,
    Rounded,
    Cubed,
}

impl From<char> for RockType {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Rounded,
            '#' => Self::Cubed,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<RockType>>);

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in &self.0 {
            for rock in row {
                match rock {
                    RockType::None => s.push('.'),
                    RockType::Rounded => s.push('O'),
                    RockType::Cubed => s.push('#'),
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
impl Grid {
    /// Immutable tilt. Returns a copy of self, tilted north
    fn tilt_north(&mut self) {
        for x in 0..GRID_SIZE {
            let mut y = 1;
            while y < GRID_SIZE {
                if self.0[y][x] == RockType::Rounded {
                    let mut swap = y;
                    while swap > 0 && self.0[swap - 1][x] == RockType::None {
                        swap -= 1;
                    }
                    if swap < y {
                        self.0[y][x] = RockType::None;
                        self.0[swap][x] = RockType::Rounded;
                    }
                }
                y += 1;
            }
        }
    }

    /// Immutable tilt. Returns a copy of self, tilted north
    fn tilt_south(&mut self) {
        for x in 0..GRID_SIZE {
            let mut y = GRID_SIZE - 2;
            loop {
                if self.0[y][x] == RockType::Rounded {
                    let mut swap = y;
                    while swap < GRID_SIZE - 1 && self.0[swap + 1][x] == RockType::None {
                        swap += 1;
                    }
                    if swap > y {
                        self.0[y][x] = RockType::None;
                        self.0[swap][x] = RockType::Rounded;
                    }
                }
                if y == 0 {
                    break;
                }
                y -= 1;
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..GRID_SIZE {
            let mut x = GRID_SIZE - 2;
            loop {
                if self.0[y][x] == RockType::Rounded {
                    let mut swap = x;
                    while swap < GRID_SIZE - 1 && self.0[y][swap + 1] == RockType::None {
                        swap += 1;
                    }
                    if swap > x {
                        self.0[y][x] = RockType::None;
                        self.0[y][swap] = RockType::Rounded;
                    }
                }
                if x == 0 {
                    break;
                }
                x -= 1;
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..GRID_SIZE {
            let mut x = 1;
            while x < GRID_SIZE {
                if self.0[y][x] == RockType::Rounded {
                    let mut swap = x;
                    while swap > 0 && self.0[y][swap - 1] == RockType::None {
                        swap -= 1;
                    }
                    if swap < x {
                        self.0[y][x] = RockType::None;
                        self.0[y][swap] = RockType::Rounded;
                    }
                }
                x += 1;
            }
        }
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.tilt_north(),
            Direction::West => self.tilt_west(),
            Direction::South => self.tilt_south(),
            Direction::East => self.tilt_east(),
        }
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East)
    }

    fn calc_load(&self) -> usize {
        let mut result = 0;
        for (y, row) in self.0.iter().enumerate() {
            for rock in row {
                if *rock == RockType::Rounded {
                    result += GRID_SIZE - y;
                }
            }
        }
        result
    }
}

fn part_one(input: &str) -> Result<()> {
    let timer = Instant::now();
    let mut grid = parse_grid(input)?;
    grid.tilt(Direction::North);
    let result = grid.calc_load();
    println!("Part One: {} -- {:?}", result, timer.elapsed());
    Ok(())
}

fn part_two(input: &str) -> Result<()> {
    // Cycle 1_000_000_000 times... uh... no.
    // Since the state is constant after GRID_SIZE cycles...
    let timer = Instant::now();
    let mut grid = parse_grid(input)?;

    for _x in 0..GRID_SIZE {
        grid.cycle();
    }

    let result = grid.calc_load();
    println!("Part Two: {} -- {:?}", result, timer.elapsed());
    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_14.txt");
    part_one(input)?;
    part_two(input)?;
    Ok(())
}

fn parse_grid(input: &str) -> Result<Grid> {
    let mut grid = vec![vec![RockType::None; GRID_SIZE]; GRID_SIZE];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = RockType::from(c);
        }
    }

    Ok(Grid(grid))
}
