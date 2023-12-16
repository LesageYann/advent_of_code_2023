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
            sum_hash(file_content.first().unwrap())
        }
        _ => {
            hash_box(file_content.first().unwrap())
        }
    };
    println!("Hello, you value is {}", value);
}

fn hash_box(line: &String) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = Vec::new();
    for i in 0..256 {
        boxes.push(Vec::new());
    }
    line.split(',')
        .for_each(|item| {
            println!("item {}", item);
            let separator = item.chars().position(|c| c=='-' || c=='=').unwrap();
            let label = &item[0..separator];
            let hash_label = hash(label);
            let mut b = &mut boxes[hash_label];
            let current_pos = b.iter().position(|(c, _)| label == c.as_str());

            match (item.chars().nth(separator).unwrap(), current_pos) {
                ('-', Some(idx)) => {b.remove(idx);}
                ('-', None) => {}
                ('=', Some(idx)) => {
                    let rest =&item[(separator +1)..(separator +2)];
                    b[idx].1 = rest.parse().unwrap();
                }
                ('=', None) => {
                    let rest =&item[(separator +1)..(separator +2)];
                    b.push((label.to_owned(), rest.parse().unwrap()));
                }
                (a,_) => {panic!(" on char {} in item {}",a,item )}
            }
        });
boxes
    .iter()
    .enumerate()
    .map(|(y, b)| {
        b
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (_, f))| {
                acc + (y + 1) * (i + 1) * f
            })
    })
    .sum()

}

fn sum_hash(line: &String) -> usize {
    line.split(",")
        .map(hash)
        .sum()
}

fn hash(s: &str) -> usize{
    s.chars()
        .fold(0, |acc, ch| {
            println!("a {}", acc);
            println!("b {}", ch as u32);
            println!("r {}", (acc + (ch  as u32))*17% 256);
            (acc + (ch  as u32))*17% 256
        }) as usize
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn algo_hash() {
        let mut input = "HASH";

        assert_eq!(hash(input), 52)
    }

    #[test]
    fn sum_hash_test() {
        let mut input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();

        assert_eq!(sum_hash(&input), 1320)
    }


}