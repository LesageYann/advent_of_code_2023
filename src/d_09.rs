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
            println!("{}", env::var("PART").unwrap_or("".to_string()).as_str());
            get_score(&file_content, Part::Two)
        }
    };
    println!("Hello, you value is {}", value);
}

fn get_score(file_content: &Vec<String>, part: Part) -> i64 {
    file_content.par_iter()
        .map(|line|{
            let mut grid: Vec<Vec<i64>> = Vec::new();
            grid.push(line.split(" ").map(|item| item.parse::<i64>().unwrap()).collect());
            while grid.last().unwrap().iter().any(|i| *i != 0) {
                let previous_row = grid.last().unwrap();
                let mut row = Vec::new();
                for i in 1..previous_row.len() {
                    row.push(previous_row[i] - previous_row[i-1]);
                }
                grid.push(row);
            }
            match part {
                Part::One => grid.iter().rev().fold(0, |acc, item| { acc+item.last().unwrap() }),
                Part::Two => grid.iter().rev().fold(0, |acc, item| { item.first().unwrap() - acc})
            }
        })
        .sum()
}

fn parse_line(line: &str) -> (String, String, String) {
    let one = line.chars().take(3).collect::<String>();
    let two = line.chars().skip(7).take(3).collect::<String>();
    let three = line.chars().skip(12).take(3).collect::<String>();
    (one, two, three)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn get_score_should_return() {
        let input = vec!(
"0 3 6 9 12 15".to_string(),
"1 3 6 10 15 21".to_string(),
"10 13 16 21 30 45".to_string(),
        );
        assert_eq!(get_score(&input, Part::One), 114);
    }


}