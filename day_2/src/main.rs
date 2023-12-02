use std::cmp::max;

use regex::Regex;

#[derive(Debug)]
struct GameHand {
    red: usize,
    blue: usize,
    green: usize,
}
impl GameHand {
    pub fn new() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

impl std::fmt::Display for GameHand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result: Vec<String> = Vec::new();
        if self.red > 0 {
            result.push(format!("{} red", self.red))
        }
        if self.blue > 0 {
            result.push(format!("{} blue", self.blue))
        }
        if self.green > 0 {
            result.push(format!("{} green", self.green))
        }
        write!(f, "{}", result.join(", "))
    }
}

impl TryFrom<&str> for GameHand {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut hand = Self::new();
        for colors in value.split(',') {
            let re = Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap();
            let Some(caps) = re.captures(colors) else {
                return Err("Malformed colors".to_string());
            };

            let count = caps["count"].parse::<usize>().map_err(|e| e.to_string())?;
            match &caps["color"] {
                "red" => hand.red = count,
                "blue" => hand.blue = count,
                "green" => hand.green = count,
                _ => return Err(format!("Unknown color:v {}", &caps["color"])),
            }
        }

        Ok(hand)
    }
}

#[derive(Debug)]
struct Game {
    pub count: usize,
    pub hands: Vec<GameHand>,
}
impl Game {
    pub fn new() -> Self {
        Self {
            count: 0,
            hands: Vec::new(),
        }
    }
    pub fn possible(&self, compare: &GameHand) -> bool {
        for hand in &self.hands {
            if hand.red > compare.red || hand.blue > compare.blue || hand.green > compare.green {
                return false;
            }
        }
        true
    }

    pub fn min_cubes(&self) -> GameHand {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for hand in &self.hands {
            red = max(red, hand.red);
            blue = max(blue, hand.blue);
            green = max(green, hand.green);
        }
        GameHand { red, blue, green }
    }
    pub fn power(&self) -> usize {
        let mins = self.min_cubes();
        mins.red * mins.blue * mins.green
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hands = self
            .hands
            .iter()
            .map(|hand| format!("{}", hand))
            .collect::<Vec<String>>();
        write!(f, "Game {}: {}", self.count, hands.join("; "))
    }
}

impl TryFrom<&str> for Game {
    type Error = String;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        // Break the line into the game counter and the hands
        let meta_parts = line.split(':').collect::<Vec<&str>>();
        if meta_parts.len() != 2 {
            return Err("Game is malformed".to_string());
        }
        // Get the game counter:
        let counter_re = Regex::new(r"Game (?<game_counter>\d+)").unwrap();
        let caps = counter_re
            .captures(meta_parts[0])
            .ok_or("Failed to parse game counter".to_string())?;
        let game_counter = caps["game_counter"]
            .parse::<usize>()
            .map_err(|e| e.to_string())?;

        let mut result = Self::new();
        result.count = game_counter;

        // Parse the hands
        let hands = meta_parts[1].split(';').collect::<Vec<&str>>();
        for hand in hands {
            result.hands.push(GameHand::try_from(hand)?);
        }

        Ok(result)
    }
}

fn part_one(games: &Vec<Game>) -> Result<(), String> {
    // Compare to
    let compare_to = GameHand::try_from("12 red, 13 green, 14 blue").expect("Failed bad!!");
    let mut total = 0;
    for game in games {
        if game.possible(&compare_to) {
            total += game.count;
        }
    }
    println!("Part One: {}", total);
    Ok(())
}

fn part_two(games: &Vec<Game>) -> Result<(), String> {
    let mut result = 0usize;
    for game in games {
        let power = game.power();
        result += power;
    }
    println!("Part Two: {}", result);
    Ok(())
}

use std::io::Write;
fn main() -> Result<(), String> {
    let lines = include_str!("../../data/day_2.txt")
        .split('\n')
        .collect::<Vec<&str>>();
    let mut games: Vec<Game> = Vec::new();
    print!("Parsing games...");
    let _ = std::io::stdout().flush();
    for line in lines {
        games.push(Game::try_from(line)?);
    }
    println!(" done!");
    // 2685
    part_one(&games)?;
    // 83707
    part_two(&games)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_hand() {
        let game_hand = GameHand::try_from("8 green, 60 blue, 2 red").expect("oops");
        println!("{}", &game_hand);
    }

    #[test]
    fn test_game() {
        let hands = " Game 12: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red ";
        let game = Game::try_from(hands).expect("failed");
        println!("{}", &game.min_cubes());
    }
}
