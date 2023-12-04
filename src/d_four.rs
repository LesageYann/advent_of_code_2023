use std::convert::TryInto;
use std::fs::read_to_string;
use std::env;

enum Part {
    One,
    Two
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn parse_number(suite: &str) -> Vec<usize> {
    println!("suite :{}", suite);
    suite.split(' ')
        .filter(|item| { item != &" " && item.len() > 0 })
        .map(|item| {
        println!("parse that : {}", item);
        item.parse().unwrap()
    }).collect()
}

fn get_count(line: &String) -> usize {
    let numbers = line.split(':').nth(1).expect("bad line format").split(" | ").collect::<Vec<&str>>();
    let winning = parse_number(numbers[0]);
    let owned = parse_number(numbers[1]);
    owned.iter().fold(0_usize, |acc, number| {
        if winning.contains(number) {
            return acc +1
        } else {
            acc
        }
    })

}

fn count(lines: Vec<String>, part: Part) -> usize {
    let mut instances = vec![1; lines.len()];
    lines.iter()
        .enumerate()
        .for_each(|(idx, line)|{
            let count = get_count(line);
            match &part {
                Part::One => {
                    if count > 0 {
                        instances[idx] = 2_usize.pow((count - 1).try_into().unwrap());
                    } else {
                        instances[idx] = count;
                    }
                },
                Part::Two => {
                    for i in 1..=count {
                        println!("{}", i);
                        instances[i + idx] += 1 * instances[idx];
                    }
                }
            }
        });

    instances.iter().sum()
}

fn main() {
    let path = env::var("FILE").unwrap();
    let part = match env::var("PART").unwrap_or("".to_string()).as_str() {
        "One" => Part::One,
        _ => Part::Two
    };
    let file_content = read_lines(&path);
    let value = count(file_content, part);
    println!("Hello, you value is {}", value);
}

#[cfg(test)]
mod tests {
    use std::str::Chars;
    use super::*;


    #[test]
    fn count_should_return_13() {
        let input = vec!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        );
        assert_eq!(count(input, Part::One), 13);
    }

    #[test]
    fn count_should_return_30() {
        let input = vec!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        );
        assert_eq!(count(input, Part::Two), 30);
    }
}