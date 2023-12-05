use std::collections::HashMap;
use std::cmp;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::env;
use std::thread;
use std::sync::mpsc;
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

fn parse_seed(lines: &Vec<String>, start_idx: usize) -> (usize, Vec<usize>) {
    let mut index = start_idx;
    let mut seeds = Vec::new();
    while index < lines.len() && !lines[index].is_empty() {
        lines[index].split(' ').skip(1).for_each(|seed| {
            seeds.push(seed.parse().expect("invalid seed "))
        });
        index += 1
    }
    (index, seeds)
}

#[derive(Copy, Clone)]
struct Mapping {
    pub origin: usize,
    pub dest: usize,
    pub end: usize,
}

fn parse_map(lines: &Vec<String>, start_index: usize) -> (usize, String, Vec<Mapping>) {
    println!("start_index {}", lines[start_index]);
    let key = lines[start_index].split(" ").nth(0).unwrap();
    let mut index = start_index + 1;
    let mut range = Vec::new();
    while index < lines.len() && !lines[index].is_empty() {
        println!("{}", lines[index]);
        let range_data = lines[index]
            .split(" ")
            .map(|item| item.parse().unwrap())
            .collect::<Vec<usize>>();

        range.push(Mapping { origin: range_data[1], dest: range_data[0], end: range_data[1] + range_data[2] });
        index += 1;
    }
    (index, key.to_string(), range)
}

fn apply_range(range: &Vec<Mapping>, origin: usize) -> usize {
    match range.iter().find(|mapping| {
        mapping.origin <= origin && origin < mapping.end
    }) {
        Some(mapping) => { mapping.dest + (origin - mapping.origin) }
        None => origin
    }
}

fn find_lowest_location_for(initial: Option<usize>, seed: usize,maps: &HashMap<String,Vec<Mapping>>) -> Option<usize> {

        let soil = apply_range(maps.get("seed-to-soil").unwrap(), seed);
        let fertilizer = apply_range(maps.get("soil-to-fertilizer").unwrap(), soil);
        let water = apply_range(maps.get("fertilizer-to-water").unwrap(), fertilizer);
        let light = apply_range(maps.get("water-to-light").unwrap(), water);
        let temperature = apply_range(maps.get("light-to-temperature").unwrap(), light);
        let humidity = apply_range(maps.get("temperature-to-humidity").unwrap(), temperature);
        let location = apply_range(maps.get("humidity-to-location").unwrap(), humidity);
        match initial {
            None => Some(location),
            Some(previous) => Some(cmp::min(location, previous))
        }

}

fn found_lowest_location(lines: Vec<String>, part: Part) -> usize {
    let (mut index, seeds) = parse_seed(&lines, 0);
    let mut maps = HashMap::new();
    while index < lines.len() {
        let (new_idx, key, range) = parse_map(&lines, index + 1);
        maps.insert(key, range);
        index = new_idx
    }
    match part {
        Part::One => {
            seeds.into_iter().fold(None, |acc, seed| {
                find_lowest_location_for(None, seed, &maps)
            }).unwrap()
        },
        Part::Two => {
            let mut handles = Vec::new();
            let (tx, rx) = mpsc::channel();
            for i in 0..(seeds.len() / 2) {
                let j = i * 2;
                let seeds_count = seeds[j + 1];
                let t_map = maps.clone();
                let sj = seeds[j];
                let tx1 = mpsc::Sender::clone(&tx);
                handles.push(thread::spawn(move || {
                    let mut lowest= None;
                    for s in 0..seeds_count {
                        lowest = find_lowest_location_for(lowest, sj + s, &t_map);
                    }
                    tx1.send(lowest.unwrap()).unwrap();
                    println!("lot j {} fini", j);
                }));
            };

            let expected_msg_count = handles.len();
            handles.into_iter().for_each(|h| h.join().unwrap());

            let mut lowest= None;
            let mut msg_count = 0;
            for i in rx {
                println!("received {}", i);
                lowest = match lowest {
                    None => Some(i),
                    Some(previous) => Some(cmp::min(i, previous))
                };
                msg_count += 1;
                if msg_count == expected_msg_count {
                    println!("hahahaha");
                    break
                }else {
                    println!("msg_count {}, expected {}", msg_count, expected_msg_count);
                }

            }
            println!("exit ?");
            lowest.unwrap()
        }
    }
}

fn main() {
    let path = env::var("FILE").unwrap();
    let part = match env::var("PART").unwrap_or("".to_string()).as_str() {
        "One" => Part::One,
        _ => Part::Two
    };
    let file_content = read_lines(&path);
    let value = found_lowest_location(file_content, part);
    println!("Hello, you value is {}", value);
}

#[cfg(test)]
mod tests {
    use std::str::Chars;
    use super::*;


    #[test]
    fn parse_seed_should_return_a_vec() {
        let input = vec!(
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
        );
        assert_eq!(parse_seed(&input, 0), (1, vec![79, 14, 55, 13]));
    }

    #[test]
    fn parse_map_should_return_a_vec() {
        let input = vec!(
            "seed-to-soil map:".to_string(),
            "50 10 2".to_string(),
            "52 0 10".to_string(),
            "".to_string()
        );
        let expected: Vec<usize> = vec!(52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 50, 51);
        assert_eq!(parse_map(&input, 0), (3_usize, "seed-to-soil".to_string(), expected));
    }
}