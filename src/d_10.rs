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
            get_case_count(&file_content, Part::Two)
        }
    };
    println!("Hello, you value is {}", value);
}

fn get_score(file_content: &Vec<String>, part: Part) -> i64 {
    let mut start: Option<(i32, i32)> = None;
    let map: Vec<Vec<char>> = file_content.iter()
        .enumerate()
        .map(|(y, line)| {
            if start.is_none() {
                line.chars().enumerate()
                    .for_each(|(x, char)| if char == 'S' { start = Some((y as i32, x as i32)) })
            }
            line.chars().map(|c| {
                match c {
                    'F' => '┌',
                    'L' => '└',
                    'J' => '┘',
                    '7' => '┐',
                    a => a
                }
            }).collect()
        })
        .collect();
    let mut location = start.unwrap();
    let mut offset = (0, 0);
    let mut count = 0;
    while location != start.unwrap() || count == 0 {
        offset = match map[location.0 as usize][location.1 as usize] {
            'S' => { (0, 1) }
            '┌' => {
                match offset {
                    (0, _) => (1, 0),
                    _ => (0, 1)
                }
            }
            '|' | '-' => { offset }
            '└' => {
                match offset {
                    (0, _) => (-1, 0),
                    _ => (0, 1)
                }
            }
            '┘' => {
                match offset {
                    (0, _) => (-1, 0),
                    _ => (0, -1)
                }
            }
            '┐' => {
                match offset {
                    (0, _) => (1, 0),
                    _ => (0, -1)
                }
            }
            a => panic!("hahahahhaa {} : {};{}", a, location.0, location.1)
        };
        location = (location.0 + offset.0, location.1 + offset.1);
        count += 1;
    }
    count / 2
}

fn get_case_count(file_content: &Vec<String>, part: Part) -> i64 {
    let mut start: Option<(i32, i32)> = None;
    let mut raw_map: Vec<Vec<char>> = file_content.iter()
        .enumerate()
        .map(|(y, line)| {
            if start.is_none() {
                line.chars().enumerate()
                    .for_each(|(x, char)| if char == 'S' { start = Some((y as i32, x as i32)) })
            }
            line.chars().map(|c| {
                match c {
                    'F' => '┌',
                    'L' => '└',
                    'J' => '┘',
                    '7' => '┐',
                    a => a
                }
            }).collect()
        })
        .collect();
    let mut clean_map = Vec::new();
    for y in 0..raw_map.len() {
        let mut v = Vec::new();
        for x in 0..raw_map[y].len() {
            v.push('.');
        }
        clean_map.push(v);
    }
    println!("========= map cleaning ==========");
    let mut location = start.unwrap();
    let mut offset = (0, 0);
    while clean_map[location.0 as usize][location.1 as usize] == '.'{
        clean_map[location.0 as usize][location.1 as usize] =raw_map[location.0 as usize][location.1 as usize];
        offset = match raw_map[location.0 as usize][location.1 as usize] {
            'S' => { (0, 1) }
            '┌' => {
                match offset {
                    (0, _) => (1, 0),
                    _ => (0, 1)
                }
            }
            '|' | '-' => { offset }
            '└' => {
                match offset {
                    (0, _) => (-1, 0),
                    _ => (0, 1)
                }
            }
            '┘' => {
                match offset {
                    (0, _) => (-1, 0),
                    _ => (0, -1)
                }
            }
            '┐' => {
                match offset {
                    (0, _) => (1, 0),
                    _ => (0, -1)
                }
            }
            a => panic!("hahahahhaa {} : {};{}", a, location.0, location.1)
        };
        location = (location.0 + offset.0, location.1 + offset.1);

        println!("{};{} => {}  {}", location.0,location.1, raw_map[location.0 as usize][location.1 as usize],         clean_map[location.0 as usize][location.1 as usize] );
    }
    println!("========= map cleaned ==========");
    let mut count =0;
    for y in 1..clean_map.len() {
        count += deep_find(&mut clean_map[y]);
    }
    println!("=============map =======");
    clean_map.iter().for_each(|l| println!("{:?} ", l));
    count as i64
}

fn deep_find(map: &mut Vec<char>) -> usize {
    let mut inside = false;
    let mut last = None;
    let mut count = 0;
    for i in 0..map.len() {
        match map[i] {
            '|' => {
                inside = !inside;
            }
            '┌' => { last = Some('┌'); }
            '└' => { last = Some('└'); }
            '┘' => {
                match last {
                    Some('┌') => { inside = !inside;}
                    Some('└') => {}
                    _ => {}
                };
                last = None;
            }
            '┐' => {
                match last {
                    Some('┌') => {}
                    Some('└') => { inside = !inside; }
                    _ => {}
                };
                last = None;
            }
            '.' => {
                if inside {

                    count += 1;
                    map[i] = 'I'
                }
            }
            _ => {}
        };
    };
    count
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn get_score_should_return() {
        let input = vec!(
            "..F7.".to_string(),
            ".FJ|.".to_string(),
            "SJ.L7".to_string(),
            "|F--J".to_string(),
            "LJ...".to_string(),
        );
        assert_eq!(get_score(&input, Part::One), 8);
    }

    #[test]
    fn get_case_should_return() {
        let input = vec!(
            "...........".to_string(),
            ".S-------7.".to_string(),
            ".|F-----7|.".to_string(),
            ".||.....||.".to_string(),
            ".||.....||.".to_string(),
            ".|L-7.F-J|.".to_string(),
            ".|..|.|..|.".to_string(),
            ".L--J.L--J.".to_string(),
            "...........".to_string(),
        );
        assert_eq!(get_case_count(&input, Part::One), 4);
    }

    #[test]
    fn get_case_should_return_10() {
        let input = vec!(
            "FF7FSF7F7F7F7F7F---7".to_string(),
            "L|LJ||||||||||||F--J".to_string(),
            "FL-7LJLJ||||||LJL-77".to_string(),
            "F--JF--7||LJLJ7F7FJ-".to_string(),
            "L---JF-JLJ.||-FJLJJ7".to_string(),
            "|F|F-JF---7F7-L7L|7|".to_string(),
            "|FFJF7L7F-JF7|JL---7".to_string(),
            "7-L-JL7||F7|L7F-7F7|".to_string(),
            "L.L7LFJ|||||FJL7||LJ".to_string(),
            "L7JLJL-JLJLJL--JLJ.L".to_string(),
        );
        assert_eq!(get_case_count(&input, Part::One), 10);
    }


    #[test]
    fn indoor_case() {
        let mut input = "..|┌┐..|└--┘|.┌┘...".to_string();
        let mut vec = input.chars().collect();

        let res = deep_find(&mut vec);
        println!("{:?}", vec);
        assert_eq!(res, 3);
    }
}