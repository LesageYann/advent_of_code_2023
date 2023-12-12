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
            println!("{}",env::var("PART").unwrap_or("".to_string()).as_str());
            get_score(&file_content, Part::Two)
        }
    };
    println!("Hello, you value is {}", value);
}

fn get_score(file_content: &Vec<String>, part: Part) -> usize {
    let pattern: Vec<char> = file_content.iter().nth(0).unwrap().chars().collect();

    let map = file_content.iter()
        .skip(2)
        .fold(HashMap::new(), |mut acc, line| {
            let (origin, left, right) = parse_line(&line);
            acc.insert(origin, (left, right));
            acc
        });
    match part {
        Part::One =>{
            let mut location = "AAA";
            get_loop_size(&pattern, &map, &mut location)
        }
        Part::Two => {
            let mut loc = map
                .keys()
                .filter(|s| s.ends_with('A'))
                .clone()
                .collect::<Vec<&String>>();
            let res = loc.par_iter()
                .map(|start| get_loop_size(&pattern, &map, &mut start.as_str()) as i64)
                .reduce_with(lcm)
                .unwrap();
            res as usize
        }
    }

}

fn get_loop_size(pattern: &Vec<char>, map: &HashMap<String, (String, String)>, start: &str) -> usize {
    let mut count = 0;
    let mut location = start;
    while !location.ends_with("Z") {
        println!("location {} {}", count, location);
        location = match pattern[count % pattern.len()] {
            'R' => {
                let (_, r) = map.get(location).unwrap();
                r
            }
            'L' => {
                let (l, _) = map.get(location).unwrap();
                l
            }
            _ => panic!("bad pattern char {}", pattern[count % pattern.len()])
        };
        count += 1;
    }
    count
}

fn parse_line(line: &str) -> (String, String, String) {
    let one = line.chars().take(3).collect::<String>();
    let two = line.chars().skip(7).take(3).collect::<String>();
    let three = line.chars().skip(12).take(3).collect::<String>();
    (one, two, three)
}

pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    println!("x: {} y: {}",x,y);
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}

/// Returns the least common multiple of two numbers.
pub fn lcm(x: i64, y: i64) -> i64 {
    x * y / gcd(x, y)
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