#[derive(Debug)]
struct Card {
    winners: Vec<usize>,
    haves: Vec<usize>,
}

impl Card {
    pub fn matches(&self) -> usize {
        let mut matches = 0;
        for have in &self.haves {
            if self.winners.contains(have) {
                matches += 1;
            }
        }
        matches
    }

    pub fn points(&self) -> usize {
        let m = self.matches();
        match m {
            0 => 0,
            1 => 1,
            _ => 2_usize.pow((m - 1) as u32),
        }
    }
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
        total += card.points();
    }
    println!("Part One: {}", total);
    Ok(())
}

fn part_two(cards: &Vec<Card>) -> Result<(), String> {
    let mut card_counts: Vec<usize> = vec![1; cards.len()];
    let mut total = 0usize;

    for (idx, card) in cards.iter().enumerate() {
        total += card_counts[idx];

        let matches = card.matches();
        if matches == 0 || idx == cards.len() - 1 {
            continue;
        }

        let cnt = std::cmp::min(idx + matches, cards.len() - 1);
        for _ in 0..card_counts[idx] {
            for i in card_counts.iter_mut().take(cnt + 1).skip(idx + 1) {
                *i += 1;
            }
        }
    }

    println!("Part Two: {}", &total);
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
    // Answer: 32001.
    part_one(&cards)?;
    // Answer: 5037841
    part_two(&cards)?;
    Ok(())
}
