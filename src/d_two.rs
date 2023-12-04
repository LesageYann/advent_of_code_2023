use std::fs::read_to_string;
use regex::Regex;
use std::env;
use std::cmp;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, Eq, PartialEq)]
struct Draw {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

fn parse_line(line: &str) -> (u32, Vec<Draw>) {
    let mut draws = Vec::new();
    let split: Vec<&str> = line.split(":").collect();
    let index = split[0].split(" ").collect::<Vec<&str>>()[1];
    println!("index ? {}", index);
    split[1].split(";").for_each(|entry| {
        let mut draw = Draw { r: 0, g: 0, b: 0 };
        entry.split(",").for_each(|part| {
            let vec: Vec<&str> = part.split(" ").collect();
            println!("vex ? {:?}", vec);
            let value = vec[1].parse::<u32>().unwrap();
            match vec[2] {
                "green" => { draw.g = value }
                "red" => { draw.r = value }
                "blue" => { draw.b = value }
                _ => {}
            }
        });
        draws.push(draw);
    });

    (index.parse::<u32>().unwrap(), draws)
}

fn is_possible_draw(draw: &Draw, max: &Draw) -> bool {
    draw.r <= max.r &&
        draw.g <= max.g &&
        draw.b <= max.b
}

fn compute_max_draw(a: &Draw, b: &Draw) -> Draw {
    Draw {
        r: cmp::max(a.r, b.r),
        g: cmp::max(a.g, b.g),
        b: cmp::max(a.b, b.b),
    }
}

fn find_value(file_content: Vec<String>, max: Draw, part: &str) -> u32 {
    match part {
        "one" => {
            file_content
                .into_iter()
                .map(|line| {
                    parse_line(&line)
                })
                .filter(|(_, draws)| {
                    draws.iter().fold(true, |acc, draw| {
                        acc && is_possible_draw(draw, &max)
                    })
                })
                .map(|(index, _)| {
                    index
                })
                .sum()
        }
        _ => {
            file_content
                .into_iter()
                .map(|line| {
                    compute_line_score(&max, &line)
                })
                .sum()
        }
    }
}

fn compute_line_score(max: &Draw, line: &String) -> u32 {
    let (_, draws) = parse_line(&line);
    let max = draws.iter().fold(Draw { r: 0, g: 0, b: 0 }, |acc, draw| {
        compute_max_draw(draw, &max)
    });
    max.r * max.g * max.b
}

fn main() {
    let path = env::var("FILE").unwrap();
    let part = env::var("PART").unwrap();
    let file_content = read_lines(&path);
    let value = find_value(file_content, Draw { r: 12, g: 13, b: 14 }, &part);
    println!("Hello, you value is {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let (index, draws) = parse_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(index, 3);
        assert_eq!(draws, vec!(
            Draw { r: 20, g: 8, b: 6 },
            Draw { r: 4, g: 13, b: 5 },
            Draw { r: 1, g: 5, b: 0 },
        ));
    }

    #[test]
    fn test_is_possible_draw_should_be_false() {
        let result = is_possible_draw(&Draw { r: 20, g: 8, b: 6 }, &Draw { r: 12, g: 13, b: 14 });
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_possible_draw_should_be_true() {
        let result = is_possible_draw(&Draw { r: 2, g: 8, b: 6 }, &Draw { r: 12, g: 13, b: 14 });
        assert_eq!(result, true);
    }


    #[test]
    fn test_part_two_result() {
        let input = vec!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
        );
        let result = find_value(input, Draw {r:0, g:0, b:0}, "two");
        assert_eq!(result,2286 );
    }


    #[test]
    fn test_first_line_score() {

    }
}