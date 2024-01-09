use std::collections::HashMap;

use anyhow::{anyhow, Result};
use utils::PriorityQueue;

type Point = utils::Point<i32>;
type Grid = utils::Grid<i32, u32>;

#[allow(dead_code)]
fn breadth_first_search(grid: &mut Grid, goal: Point) -> Result<Vec<Point>> {
    let mut frontier: Vec<Point> = Vec::with_capacity(grid.width() * grid.height());
    let mut came_from = HashMap::<Point, Option<Point>>::new();
    let mut cost = HashMap::<Point, u32>::new();

    frontier.push(grid.current());
    came_from.insert(grid.current(), None);
    cost.insert(grid.current(), 0);

    while let Some(current) = frontier.pop() {
        if current == goal {
            break;
        }
        for point in grid.neighbors(&current, 3) {
            came_from.entry(point).or_insert_with(|| {
                frontier.push(point);
                Some(current)
            });
        }
    }
    let path = extract_path(&came_from, goal);
    display_grid(grid, &path);
    Ok(path)
}

#[allow(dead_code)]
fn dijkstra_search(grid: &mut Grid, goal: Point) -> Result<Vec<Point>> {
    let mut frontier: PriorityQueue<u32, Point> =
        PriorityQueue::with_capacity(true, grid.width() * grid.height());
    let mut came_from = HashMap::<Point, Option<Point>>::new();
    let mut cost_so_far = HashMap::<Point, u32>::new();

    frontier.push(0, grid.current());
    came_from.insert(grid.current(), None);
    cost_so_far.insert(grid.current(), 0);

    while let Some((_heat, current)) = frontier.pop() {
        if current == goal {
            break;
        }
        /*
        for next in graph.neighbors(current):
             new_cost = cost_so_far[current] + graph.cost(current, next)
             if next not in cost_so_far or new_cost < cost_so_far[next]:
                cost_so_far[next] = new_cost
                priority = new_cost
                frontier.put(next, priority)
                came_from[next] = current
        */
        let current_cost = cost_so_far.get(&current).unwrap_or(&0).to_owned();
        for next in grid.neighbors(&current, 1) {
            let next_heat = grid.get(&next)?;
            let new_cost = current_cost + next_heat;
            let next_cost = cost_so_far.get(&next).unwrap_or(&0).to_owned();
            if !cost_so_far.contains_key(&next) || new_cost < next_cost {
                cost_so_far.insert(next, new_cost);
                frontier.push(new_cost, next);
                came_from.insert(next, Some(current));
            }
        }
    }

    let path = extract_path(&came_from, goal);
    display_grid(grid, &path);
    Ok(path)
}

fn extract_path(came_from: &HashMap<Point, Option<Point>>, goal: Point) -> Vec<Point> {
    // Now walk backward
    let mut path: Vec<Point> = Vec::new();
    let mut next = &goal;
    loop {
        let Some(point) = came_from.get(next) else {
            break;
        };
        let Some(point) = point else {
            break;
        };
        path.push(*point);
        next = point;
    }
    path
}

fn display_grid(grid: &Grid, path: &[Point]) {
    let mut result = String::from("");

    for y in 0..grid.height() {
        let mut left = String::new();
        let mut right = String::new();
        for x in 0..grid.width() {
            let point = Point::from((x as i32, y as i32));
            let heat = grid.get_at(x, y).expect("WTFFFF");
            left.push_str(&format!("{}", heat));
            if path.contains(&point) {
                right.push('.');
            } else {
                right.push_str(&format!("{}", heat));
            }
        }
        result.push_str(&format!("{}  {}\n", left, right));
    }
    println!("{}", result);
}

fn parse_input(puzzle_input: &str) -> Grid {
    let convert = |c: char| c.to_digit(10).ok_or(anyhow!("Failed to convert"));
    Grid::parse_undelim_str(puzzle_input, convert).unwrap()
}

fn part_one(puzzle_input: &str) -> Result<()> {
    let timer = std::time::Instant::now();
    let mut grid = parse_input(puzzle_input);
    println!(
        "Grid: {} - {} {},{}",
        grid.current(),
        grid.bounds(),
        grid.width(),
        grid.height()
    );

    let goal = Point::from((grid.width() - 1, grid.height() - 1));
    let path = breadth_first_search(&mut grid, goal)?;

    let _ = dijkstra_search(&mut grid, goal)?;
    println!(
        "Part One: {} -- {}",
        path.len(),
        timer.elapsed().as_millis()
    );
    Ok(())
}
fn main() -> Result<()> {
    let puzzle_input = std::fs::read_to_string("./puzzle_input.txt")?;
    part_one(&puzzle_input)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        println!("This is a test");
    }
}
