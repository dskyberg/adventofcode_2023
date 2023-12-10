/// Thanks to https://www.reddit.com/user/KyleGBC/ for hings on ordering!
use std::collections::HashMap;

fn map_card(card: char, wilds: bool) -> u32 {
    match card {
        'T' => 10,
        'J' => {
            if wilds {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => String::from(card).parse::<u32>().unwrap(),
    }
}

#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
enum HandKind {
    #[default]
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandKind {
    fn eval(cards: &[u32], use_wilds: bool) -> Result<Self, String> {
        let mut counts = HashMap::<u32, u32>::new();

        let mut wilds = 0;
        for card_val in cards {
            if use_wilds && *card_val == 1 {
                wilds += 1;
            } else {
                counts.entry(*card_val).and_modify(|n| *n += 1).or_insert(1);
            }
        }
        let counts = counts.into_values().collect::<Vec<_>>();
        let max = counts.iter().max().unwrap_or(&0) + wilds;

        match counts.len() {
            1 | 0 => Ok(Self::Five),
            2 if max == 4 => Ok(Self::Four),
            2 => Ok(Self::FullHouse),
            3 if max == 3 => Ok(Self::Three),
            3 => Ok(Self::TwoPair),
            4 => Ok(Self::OnePair),
            5 => Ok(Self::High),
            _ => Err("Unrecognized card".to_string()),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    kind: HandKind,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {:?}", self.cards_to_string(), self.bid, self.kind)
    }
}

impl Hand {
    fn cards_to_string(&self) -> String {
        let mut result = String::new();
        for card in &self.cards {
            match *card {
                1 => result.push('J'),
                2 => result.push('2'),
                3 => result.push('3'),
                4 => result.push('4'),
                5 => result.push('5'),
                6 => result.push('6'),
                7 => result.push('7'),
                8 => result.push('8'),
                9 => result.push('9'),
                10 => result.push('T'),
                11 => result.push('J'),
                12 => result.push('Q'),
                13 => result.push('K'),
                14 => result.push('A'),
                _ => result.push('?'),
            }
        }
        result
    }

    fn from_str(value: &str, use_wilds: bool) -> Result<Self, String> {
        let parts = value.split(' ').collect::<Vec<&str>>();

        if parts[0].len() != 5 {
            return Err("wrong number of cards".to_string());
        }
        let cards = parts[0]
            .chars()
            .map(|c| map_card(c, use_wilds))
            .collect::<Vec<u32>>();
        let bid = parts[1].parse::<u32>().unwrap();
        let kind = HandKind::eval(&cards, use_wilds)?;
        Ok(Hand { cards, bid, kind })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind != other.kind {
            self.kind.cmp(&other.kind)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn eval(lines: &str, part: &str) -> Result<(), String> {
    let start = std::time::Instant::now();
    let use_wilds = matches!(part, "Two");

    let mut hands = parse_hands(lines, use_wilds)?;
    hands.sort();

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + (rank as u32 + 1) * hand.bid);

    let elapsed = start.elapsed();
    println!("Part {}: {} -- {:?}", part, &winnings, elapsed);
    Ok(())
}

fn parse_hands(input: &str, use_wilds: bool) -> Result<Vec<Hand>, String> {
    let hands = input
        .lines()
        .map(|line| Hand::from_str(line, use_wilds).map_err(|e| e.to_string()))
        .collect::<Result<Vec<Hand>, String>>()?;
    Ok(hands)
}

fn main() -> Result<(), String> {
    let lines = include_str!("../../data/day_7.txt");
    // Git: 253954294
    eval(lines, "One")?;
    eval(lines, "Two")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_cards() {
        let line = "AA444 465";
        let hand = Hand::from_str(line, false).expect("oops");
        dbg!(hand);
    }

    #[test]
    fn test_ordering() {
        assert!(HandKind::Five > HandKind::Four);
        assert!(HandKind::FullHouse > HandKind::Three);
    }

    #[test]
    fn card_ord() {
        let cards1: Vec<u32> = vec![2, 2, 3, 4, 5];
        let cards2: Vec<u32> = vec![2, 2, 3, 4, 6];
        dbg!(cards1.cmp(&cards2));
    }
}
