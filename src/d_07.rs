use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::env;
use rayon::prelude::*;

enum Part {
    One,
    Two,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let path = env::var("FILE").unwrap();
    let file_content = read_lines(&path);
    let value = match env::var("PART").unwrap_or("".to_string()).as_str() {
        "One" => get_score(&file_content, Part::One),
        _ => {
            get_score(&file_content, Part::Two)
        }
    };
    println!("Hello, you value is {}", value);
}

fn get_score(lines: &Vec<String>, part: Part) -> usize {
    let mut hands = lines.iter()
        .map(|line| {
            match part {
                Part::One => parse_line(line.as_str()),
                Part::Two => parse_line_with_joker(line.as_str())
            }
        })
        .collect::<Vec<Hand>>();
    hands
        .sort_by(|a, b| {
            match a.card_type.cmp(&b.card_type) {
                Ordering::Equal => {
                    a.cards.cmp(&b.cards)
                }
                v => { v }
            }
        });
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| {
            acc + ((i + 1) * item.bid)
        })
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
enum CardType {
    Value,
    Pair,
    DoublePair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    card_type: CardType,
    bid: usize,
    cards: String,
}

fn parse_line(line: &str) -> Hand {
    let mut split = line.split(" ");
    let mut cards = split.next().unwrap().chars().map(|c| {
        match c {
            'A' => 'E',
            'K' => 'D',
            'Q' => 'C',
            'J' => 'B',
            'T' => 'A',
            digit => digit,
        }
    }).collect::<String>();
    let mut card_map: HashMap<char, u8> = HashMap::new();
    for card in cards.chars().take(5) {
        let count = card_map.get(&card).unwrap_or(&0);
        card_map.insert(card, count + 1);
    }

    let mut values: Vec<u8> = card_map.values().map(|i| i.clone()).collect();
    values.sort_by(|a, b| b.cmp(&a));
    values.push(0);

    let card_type = match (values[0], values[1]) {
        (5, _) => CardType::Five,
        (4, _) => CardType::Four,
        (3, 2) => CardType::FullHouse,
        (3, 1) => CardType::Three,
        (2, 2) => CardType::DoublePair,
        (2, 1) => CardType::Pair,
        _ => CardType::Value,
    };
    Hand {
        bid: split.next().unwrap().parse::<usize>().unwrap(),
        card_type,
        cards,
    }
}

fn parse_line_with_joker(line: &str) -> Hand {
    let mut split = line.split(" ");
    let mut cards = split.next().unwrap().chars().map(|c| {
        match c {
            'A' => 'E',
            'K' => 'D',
            'Q' => 'C',
            'J' => '0',
            'T' => 'A',
            digit => digit,
        }
    }).collect::<String>();
    let mut card_map: HashMap<char, u8> = HashMap::new();
    for card in cards.chars().take(5) {
        let count = card_map.get(&card).unwrap_or(&0);
        card_map.insert(card, count + 1);
    }
    println!("card map keys {:?}", card_map.keys());
    let joker_count = card_map.get(&'0').unwrap_or(&0).clone();
    card_map.remove(&'0');

    let mut values: Vec<u8> = card_map.values().map(|i| i.clone()).collect();
    values.sort_by(|a, b| b.cmp(&a));
    values.push(0);
    values.push(0);

    let card_type = match (values[0], values[1], joker_count) {
        (5, _, _) | (4, _, 1) | (3, _, 2) | (2, _, 3) | (1, _, 4) | (0, _, 5) => CardType::Five,
        (4, _, _) | (3, _, 1) | (2, _, 2) | (1, _, 3) | (_, _, 4) => CardType::Four,
        (3, 2, _) | (2, 2, 1) | (2, 1, 2)  => CardType::FullHouse,
        (3, 1, _) | (2, 1, 1) | (1,1,2) => CardType::Three,
        (2, 2, _) => CardType::DoublePair,
        (2, 1, _)|(_, _, 1) => CardType::Pair,
        (_, _, 0) => CardType::Value,
        _ => panic!("on combi {}", cards),
    };
    Hand {
        bid: split.next().unwrap().parse::<usize>().unwrap(),
        card_type,
        cards,
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn get_score_should_return() {
        let input = vec!(
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        );
        assert_eq!(get_score(&input, Part::One), 6440);
    }

    #[test]
    fn parse_should_return() {
        let result = parse_line("T55J5 684");
        assert_eq!(result, Hand { card_type: CardType::Three, bid: 684, cards: "T55J5".into_string() });
    }
}