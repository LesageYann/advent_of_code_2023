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
            file_content.iter()
                .map(|l| count_combinaison(l, Part::One))
                .sum()
        }
        _ => {
            file_content
                .iter()
                .enumerate()
                .map(|(i, l)| {
                    let res = count_combinaison(l, Part::Two);
                    res
                })
                .sum()
        }
    };
    println!("Hello, you value is {}", value);
}

fn count_combinaison(line: &String, part: Part) -> usize {
    let mut split = line.split(' ');
    let mut row: String = split.nth(0).unwrap().to_string();
    let mut pattern: Vec<usize> = split.nth(0).unwrap().split(",").map(|item| item.parse().unwrap()).collect();
    if part == Part::Two {
        row = vec!(row.as_str()).repeat(5).join("?");
        pattern = pattern.repeat(5);
    }
    arrangement(row, &pattern)
}


#[cache(LruCache : LruCache::new(200))]
fn arrangement(row: String, groups: &Vec<usize>) -> usize {
    match row.chars().nth(0) {
        Some('#') => {
            try_apply(&row.as_str(), groups)
        }
        Some('?') => {
            try_apply(&row.as_str(), groups) +
                arrangement(row[1..row.len()].to_string(), groups)

        }
        Some('.') => arrangement(row[1..row.len()].to_string(), groups),
        _ => {
            if groups.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }
    }
}

fn try_apply(row: &str, groups: &Vec<usize>/*, previous: String*/) -> usize {
    if groups.len() == 0 {
        return 0;
    }
    let len = *groups.first().unwrap();
    if can_apply(row, len) {
        if len +1 > row.len() {
            return  arrangement(row[len..row.len()].to_string(), &groups[1..groups.len()].iter().map(|i| i.clone()).collect()/*, previous.clone() + &"#".repeat(len)*/)
        } else {
            arrangement(row[len+1..row.len()].to_string(), &groups[1..groups.len()].iter().map(|i| i.clone()).collect()/*, previous.clone() + &"#".repeat(len) + "."*/)
        }
    } else {
        0
    }
}

fn can_apply(row: &str, len: usize) -> bool {
    row.len() >= len && !row.chars().take(len).any(|item| item == '.') &&
        row.chars().nth(len)
            .and_then(|item|Some(item != '#'))
            .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_t_apply() {
        assert_eq!(can_apply("###", 2), false);
    }

    #[test]
    fn can_apply_() {
        assert_eq!(can_apply("##?", 2), true);
    }

    #[test]
    fn can_apply_dont_panic() {
        assert_eq!(can_apply("##", 2), true);
    }

    #[test]
    fn count_combinaison_should_return_10() {
        assert_eq!(count_combinaison(&"?###???????? 3,2,1".to_string(), Part::One), 10);
    }
    
    #[test]
    fn count_combinaison_should_return_1() {
        assert_eq!(count_combinaison(&"???.### 1,1,3".to_string(), Part::One), 1);
    }

    #[test]
    fn count_combinaison_should_return_1_complex() {
        assert_eq!(count_combinaison(&"?#?#?#?#?#?#?#? 1,3,1,6".to_string(), Part::One), 1);
    }

    #[test]
    fn count_combinaison_should_return_4() {
        assert_eq!(count_combinaison(&"????.######..#####. 1,6,5".to_string(), Part::One), 4);
    }

    #[test]
    fn count_combinaison_should_return_4_complex() {
        assert_eq!(count_combinaison(&".??..??...?##. 1,1,3".to_string(), Part::One), 4);
    }
}