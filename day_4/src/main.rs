#[derive(Debug)]
struct Card {
    winners: Vec<usize>,
    haves: Vec<usize>,
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let card_parts = value.split(':').collect::<Vec<&str>>();
        if card_parts.len() != 2 {
            return Err("Malformed card".to_string());
        }
        let game_parts = card_parts[1].split('|').collect::<Vec<&str>>();
        if game_parts.len() != 2 {
            return Err("Malformed game parts".to_string());
        }

        let winners = game_parts[0].split(' ').collect::<Vec<&str>>();

        let winners = winners
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<usize>, String>>()?;

        //    let winners = winners.map(|s| s.parse::<usize>().map_err(|e| e.to_string())?)
        let haves = game_parts[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<usize>, String>>()?;

        Ok(Card { winners, haves })
    }
}

fn part_one(cards: &Vec<Card>) -> Result<(), String> {
    let mut total = 0;

    for card in cards {
        let mut points = 0;
        for have in &card.haves {
            if card.winners.contains(have) {
                points = match points {
                    0 => 1,
                    _ => points * 2,
                };
            }
        }
        total += points;
    }
    println!("Part One: {}", total);
    Ok(())
}

fn parse_data(data: &str) -> Result<Vec<Card>, String> {
    let cards = data
        .split('\n')
        .map(|s| Card::try_from(s).map_err(|e| e.to_string()))
        .collect::<Result<Vec<Card>, String>>()?;

    Ok(cards)
}

fn main() -> Result<(), String> {
    let data = include_str!("../../data/day_4.txt");
    let cards = parse_data(data.trim())?;
    // Anser is 32001.
    part_one(&cards)?;
    Ok(())
}
