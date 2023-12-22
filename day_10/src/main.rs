use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq)]
enum Connections {
    NorthAndSouth,
    EastAndWest,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    #[default]
    Ground,
    Start,
}

impl From<char> for Connections {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NorthAndSouth,
            '-' => Self::EastAndWest,
            'L' => Self::NorthAndEast,
            'J' => Self::NorthAndWest,
            '7' => Self::SouthAndWest,
            'F' => Self::SouthAndEast,
            'S' => Self::Start,
            _ => Self::Ground,
        }
    }
}

impl std::fmt::Display for Connections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Connections::NorthAndSouth => '│',
                Connections::EastAndWest => '─',
                Connections::NorthAndEast => '└',
                Connections::NorthAndWest => '┘',
                Connections::SouthAndWest => '┐',
                Connections::SouthAndEast => '┌',
                Connections::Ground => ' ',
                Connections::Start => 'S',
            }
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Default, Clone)]
struct Tile {
    point: Point,
    conns: Connections,
}

impl Tile {
    fn possible_connects(&self, max_x: usize, max_y: usize) -> Vec<Point> {
        let mut results: Vec<Point> = Vec::new();
        match self.conns {
            Connections::NorthAndSouth => {
                if self.point.y > 0 {
                    results.push(Point {
                        x: self.point.x,
                        y: self.point.y - 1,
                    })
                }
                if self.point.y < max_y - 1 {
                    results.push(Point {
                        x: self.point.x,
                        y: self.point.y + 1,
                    })
                }
            }
            Connections::EastAndWest => {
                if self.point.x > 0 {
                    results.push(Point {
                        x: self.point.x - 1,
                        y: self.point.y,
                    });
                }
                if self.point.x < max_x - 1 {
                    results.push(Point {
                        x: self.point.x + 1,
                        y: self.point.y,
                    });
                }
            }
            Connections::NorthAndEast => {
                if self.point.y > 0 {
                    results.push(Point {
                        x: self.point.x,
                        y: self.point.y - 1,
                    })
                }
                if self.point.x < max_x - 1 {
                    results.push(Point {
                        x: self.point.x + 1,
                        y: self.point.y,
                    });
                }
            }
            Connections::NorthAndWest => {
                if self.point.y > 0 {
                    results.push(Point {
                        x: self.point.x,
                        y: self.point.y - 1,
                    })
                }
                if self.point.x > 0 {
                    results.push(Point {
                        x: self.point.x - 1,
                        y: self.point.y,
                    });
                }
            }
            Connections::SouthAndWest => {
                if self.point.y < max_y - 1 {
                    results.push(Point {
                        x: self.point.x,
                        y: self.point.y + 1,
                    })
                }
                if self.point.x > 0 {
                    results.push(Point {
                        x: self.point.x - 1,
                        y: self.point.y,
                    });
                }
            }
            Connections::SouthAndEast => {
                if self.point.y < max_y - 1 {
                    results.push(Point {
                        x: self.point.x,
                        y: self.point.y + 1,
                    })
                }
                if self.point.x < max_x - 1 {
                    results.push(Point {
                        x: self.point.x + 1,
                        y: self.point.y,
                    });
                }
            }
            _ => {}
        }
        results
    }
}

#[derive(Debug, Default)]
struct Pipes {
    pub max_x: usize,
    pub max_y: usize,
    pub map: Vec<Vec<Tile>>,
    /// The Connections for the starting coordinate is normalized in [parse_input].
    pub start: Tile,
}

impl Pipes {
    fn get_coord(&self, point: &Point) -> &Tile {
        &self.map[point.y][point.x]
    }

    /// Get the starting connections, and just use the first one as the first "from"
    fn starting_from(&self) -> &Tile {
        let start_possibles = self
            .get_coord(&self.start.point)
            .possible_connects(self.max_x, self.max_y);

        self.get_coord(&start_possibles[0])
    }

    /// Find the two connecting tiles for the current tile,
    /// One should match 'from'.  Return the other.
    fn next(&self, curr: &Tile, from: &Tile) -> &Tile {
        let possibles = curr.possible_connects(self.max_x, self.max_y);
        if possibles.len() != 2 {
            panic!("This should always be 2 possibilities!");
        }
        if possibles[0] == from.point {
            return self.get_coord(&possibles[1]);
        }
        if possibles[1] == from.point {
            return self.get_coord(&possibles[0]);
        } else {
            panic!("A connection without possibilities!");
        }
    }
    ///
    fn measure_path(&self) -> Vec<Point> {
        let mut from = self.starting_from();

        let mut curr = &self.start;
        let mut moves: Vec<Point> = vec![curr.point.clone()];
        loop {
            let next = self.next(curr, from);
            from = curr;
            curr = next;
            moves.push(curr.point.clone());

            if curr.point == moves[0] {
                break;
            }
        }
        moves
    }

    fn find_area(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let path = self.measure_path();
        let inside = |edges: usize| -> bool { edges > 0 && edges % 2 == 1 };

        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.conns != Connections::Ground {
                    continue;
                }
                let left_edges = self.map[y][0..x]
                    .iter()
                    .filter(|t| {
                        path.contains(&t.point)
                            && matches!(
                                t.conns,
                                Connections::NorthAndSouth
                                    | Connections::NorthAndEast
                                    | Connections::NorthAndWest
                            )
                    })
                    .count();

                let right_edges = self.map[y][x..self.max_x]
                    .iter()
                    .filter(|t| {
                        path.contains(&tile.point)
                            && matches!(
                                t.conns,
                                Connections::NorthAndSouth
                                    | Connections::NorthAndEast
                                    | Connections::NorthAndWest
                            )
                    })
                    .count();

                if inside(left_edges) && right_edges > 0 {
                    points.push(Point {
                        x: tile.point.x,
                        y: tile.point.y,
                    });
                }
            }
        }
        points
    }
}

/// Find the farthest
fn part_one(pipes: &Pipes) -> Result<()> {
    let timer = std::time::Instant::now();
    let result = pipes.measure_path();

    println!("Part One: {} -- {:?}", result.len() / 2, timer.elapsed());
    Ok(())
}

fn part_two(pipes: &Pipes) -> Result<()> {
    let timer = std::time::Instant::now();
    let result = pipes.find_area();

    println!("Part Two: {} -- {:?}", result.len(), timer.elapsed());
    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_10.txt");
    let pipes = parse_input(input);
    part_one(&pipes)?;
    part_two(&pipes)?;

    Ok(())
}

fn parse_input(input: &str) -> Pipes {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut start_point = Point::default();

    for (y, row) in input.lines().enumerate() {
        let mut dirs: Vec<Tile> = Vec::new();
        for (x, c) in row.chars().enumerate() {
            let conns = Connections::from(c);
            if matches!(conns, Connections::Start) {
                start_point = Point { x, y };
            }
            dirs.push(Tile {
                point: Point { x, y },
                conns,
            });
        }
        map.push(dirs);
    }

    let max_y = map.len();
    let max_x = map[0].len();

    // Figure out what Start direction actually is.  I don't know if this is really necessary,
    // but let's find out.
    let west_coord = &map[start_point.y][start_point.x - 1];
    let possible_west = west_coord.possible_connects(max_x, max_y);

    let east_coord = &map[start_point.y][start_point.x + 1];
    let possible_east = east_coord.possible_connects(max_x, max_y);

    let north_coord = &map[start_point.y - 1][start_point.x];
    let possible_north = north_coord.possible_connects(max_x, max_y);

    let south_coord = &map[start_point.y + 1][start_point.x];
    let possible_south = south_coord.possible_connects(max_x, max_y);

    let mut start_coord = Tile {
        point: start_point.clone(),
        conns: Connections::Ground,
    };

    if possible_west.contains(&start_point) && possible_west.contains(&start_point) {
        start_coord.conns = Connections::EastAndWest;
    } else if possible_west.contains(&start_point) && possible_north.contains(&start_point) {
        start_coord.conns = Connections::SouthAndWest;
    } else if possible_west.contains(&start_point) && possible_south.contains(&start_point) {
        start_coord.conns = Connections::NorthAndWest
    } else if possible_east.contains(&start_point) && possible_north.contains(&start_point) {
        start_coord.conns = Connections::SouthAndEast;
    } else if possible_east.contains(&start_point) && possible_south.contains(&start_point) {
        start_coord.conns = Connections::NorthAndEast;
    } else if possible_north.contains(&start_point) && possible_south.contains(&start_point) {
        start_coord.conns = Connections::NorthAndSouth;
    } else {
        panic!("Was not able to rectify Start");
    }
    map[start_point.y][start_point.x].conns = start_coord.conns.clone();

    Pipes {
        max_x,
        max_y,
        map,
        start: start_coord,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let input = include_str!("../../data/day_10.txt");
        let pipes = parse_input(input);
        let result = pipes.measure_path();
        let area = pipes.find_area();
        for row in pipes.map {
            for tile in row {
                if result.contains(&tile.point) {
                    print!("{}", &tile.conns);
                } else if area.contains(&tile.point) {
                    print!("I");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!("Area: {}", area.len());
    }
}
