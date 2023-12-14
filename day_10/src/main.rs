use anyhow::Result;

#[derive(Debug, Default, Clone)]
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
                Connections::NorthAndSouth => '|',
                Connections::EastAndWest => '-',
                Connections::NorthAndEast => 'L',
                Connections::NorthAndWest => 'J',
                Connections::SouthAndWest => '7',
                Connections::SouthAndEast => 'F',
                Connections::Ground => '.',
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
struct Coord {
    point: Point,
    conns: Connections,
}

impl Coord {
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
    pub map: Vec<Vec<Coord>>,
    /// The Connections for the starting coordinate is normalized in [parse_input].
    pub start: Coord,
}

impl Pipes {
    fn get_coord(&self, point: &Point) -> &Coord {
        &self.map[point.y][point.x]
    }

    /// Get the starting connections, and just use the first one as the first "from"
    fn starting_from(&self) -> &Coord {
        let start_possibles = self
            .get_coord(&self.start.point)
            .possible_connects(self.max_x, self.max_y);

        self.get_coord(&start_possibles[0])
    }

    /// Find the two connecting coords for the current spot,
    /// One should match 'from'.  Return the other.
    fn next(&self, curr: &Coord, from: &Coord) -> &Coord {
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
}

fn part_one(input: &str) -> Result<()> {
    let start = std::time::Instant::now();
    let pipes = parse_input(input);
    let result = pipes.measure_path();

    println!("Part One: {} -- {:?}", result.len() / 2, start.elapsed());
    Ok(())
}

fn part_two(_input: &str) -> Result<()> {
    let start = std::time::Instant::now();

    println!("Part Two: -- {:?}", start.elapsed());
    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_10.txt");

    // Answer:
    part_one(input)?;
    // Answer:
    part_two(input)?;

    Ok(())
}

fn parse_input(input: &str) -> Pipes {
    let mut map: Vec<Vec<Coord>> = Vec::new();
    let mut start_point = Point::default();

    for (y, row) in input.lines().enumerate() {
        let mut dirs: Vec<Coord> = Vec::new();
        for (x, c) in row.chars().enumerate() {
            let conns = Connections::from(c);
            if matches!(conns, Connections::Start) {
                start_point = Point { x, y };
            }
            dirs.push(Coord {
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

    let mut start_coord = Coord {
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
