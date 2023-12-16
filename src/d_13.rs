use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::env;
use rayon::prelude::*;
use cache_macro::cache;
use lru_cache::LruCache;


#[derive(PartialEq)]
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
    let value: usize = match env::var("PART").unwrap_or("".to_string()).as_str() {
        "One" => {
            parse_all(&file_content)
                .iter()
                .map(|l| found_axe(l, Part::One))
                .sum()
        }
        _ => {
            parse_all(&file_content)
                .iter()
                .map(|l| found_axe(l, Part::Two))
                .sum()
        }
    };
    println!("Hello, you value is {}", value);
}

fn parse_all(lines: &Vec<String>) -> Vec<Vec<Vec<usize>>> {
    lines
        .iter()
        .fold(vec!(Vec::new()), |mut acc, line| {
            if line == "" {
                acc.push(Vec::new());
            } else {
                let v = acc.last_mut().unwrap();
                v.push(line.chars().map(map_char).collect());
            }
            acc
        })
}

fn map_char(c: char) -> usize {
    match c {
        '.' => 0,
        _ => 1
    }
}

fn found_axe(pattern: &Vec<Vec<usize>>, part: Part) -> usize {
    let mut total = 0;
    for i in 1..pattern.len() {
        if checkHorizontal(&pattern, i, &part) {
            total += 100 * i;
            break;
        }
    }

    let transposed = transpose(&pattern);
    for i in 1..transposed.len() {
        if checkHorizontal(&transposed, i, &part) {
            total += i;
            break;
        }
    }
    println!("total {}", total);
    total
}

fn checkHorizontal(pattern: &Vec<Vec<usize>>, row: usize, part: &Part) -> bool {
    match part {
        Part::One => {
            let mut i: i32 = row as i32 - 1;
            let mut j = row;
            while i >= 0 && j < pattern.len() {
                if pattern[i as usize] != pattern[j] {
                    return false;
                }
                i -= 1;
                j += 1;
            }
            true
        }
        Part::Two => {
            let mut smudge = None;
            let mut i: i32 = row as i32 - 1;
            let mut j = row;
            while i >= 0 && j < pattern.len() {
                let pi = &pattern[i as usize];
                let pj = &pattern[j];

                for k in 0..pi.len() {
                    if pi[k] != pj[k] {
                        if smudge.is_some(){
                            return false
                        } else {
                            smudge = Some(0);
                        }
                    }
                }
                i -= 1;
                j += 1;
            }
            if smudge.is_some() {
                true
            } else {
                false
            }
        }
        }
    }

    fn transpose(pattern: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        for _ in 0..pattern[0].len() {
            result.push(Vec::new())
        }
        pattern.iter()
            .for_each(|row| {
                row.iter()
                    .enumerate()
                    .for_each(|(i, c)| result[i].push(c.clone()))
            }
            );
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn found_axe_column() {
            let input = vec!(
                "#.##..##.".chars().map(map_char).collect(),
                "..#.##.#.".chars().map(map_char).collect(),
                "##......#".chars().map(map_char).collect(),
                "##......#".chars().map(map_char).collect(),
                "..#.##.#.".chars().map(map_char).collect(),
                "..##..##.".chars().map(map_char).collect(),
                "#.#.##.#.".chars().map(map_char).collect(),
            );

            assert_eq!(found_axe(&input), 5);
        }

        #[test]
        fn found_axe_row() {
            let input = vec!(
                "#...##..#".chars().map(map_char).collect(),
                "#....#..#".chars().map(map_char).collect(),
                "..##..###".chars().map(map_char).collect(),
                "#####.##.".chars().map(map_char).collect(),
                "#####.##.".chars().map(map_char).collect(),
                "..##..###".chars().map(map_char).collect(),
                "#....#..#".chars().map(map_char).collect(),
            );

            assert_eq!(found_axe(&input), 400);
        }

        #[test]
        fn found_axe_11() {
            let input = vec!(
                "#.......##.".chars().map(map_char).collect(),
                ".#.###....#".chars().map(map_char).collect(),
                "###...#....".chars().map(map_char).collect(),
                "..#........".chars().map(map_char).collect(),
                "###.##...#.".chars().map(map_char).collect(),
                ".#..##..###".chars().map(map_char).collect(),
                "####.#.##.#".chars().map(map_char).collect(),
                "...#.#.###.".chars().map(map_char).collect(),
                "..##.#.###.".chars().map(map_char).collect(),
                ".#...#.##.#".chars().map(map_char).collect(),
                "#.##..#####".chars().map(map_char).collect(),
                "#.##..#####".chars().map(map_char).collect(),
                ".#...#.##.#".chars().map(map_char).collect(),
                "..##.#.###.".chars().map(map_char).collect(),
                "...#.#.###.".chars().map(map_char).collect(),
            );
            assert_eq!(found_axe(&input), 1100);
        }
    }