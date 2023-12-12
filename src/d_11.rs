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
        "One" => {
            let galaxies = get_galaxies(&file_content, 2);
            get_score(&galaxies, Part::One)
        }
        _ => {
            let galaxies = get_galaxies(&file_content,1000000);
            get_score(&galaxies, Part::One)
        }
    };
    println!("Hello, you value is {}", value);
}

fn get_galaxies(file_content: &Vec<String>, pound: usize) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    let mut grid: Vec<Vec<usize>> = file_content
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        '.' => { 1 }
                        a => { galaxies.push((y, x));0 }
                    }
                })
                .collect()
        }).collect();
    let mut add_y = Vec::new();
    for y in 0..grid.len() {
        if !grid[y].iter().any(|item| *item != 1) {
            add_y.push(y);
        }
    }
    let mut add_x = Vec::new();
    for x in 0..grid[0].len() {
        if !galaxies.iter().any(|(_, gx)| *gx == x) {
            add_x.push(x);
        }
    }

    let p = pound -1;
    galaxies.into_iter().map(|(y, x)| {
        let y_offset = add_y.iter().filter(|yo| **yo < y).count();
        let x_offset = add_x.iter().filter(|xo| **xo < x).count();
        (y + (y_offset *p), x + (x_offset * p))
    }).collect()
}

fn get_score(galaxies: &Vec<(usize, usize)>, part: Part) -> usize {
    let mut count = 0;
    for current in 0..galaxies.len() {
        let (y, x) = galaxies[current];
        for next in (current + 1)..galaxies.len() {
            let (y_b, x_b) = galaxies[next];
            let y_d = (y as i64 - y_b as i64).abs() as usize;
            let x_d = (x as i64 - x_b as i64).abs() as usize;
            println!("for {};{} et  {};{}, {}",y,x,y_b,x_b,y_d + x_d);
            count += y_d + x_d;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn get_score_should_return() {
        let input = vec!(
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        );
        assert_eq!(get_score(&get_galaxies(&input), Part::One), 374);
    }
}