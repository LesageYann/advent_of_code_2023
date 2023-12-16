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
            let mut content = file_content.iter().map(|line| line.chars().map(|c| c.clone()).collect()).collect();
            go_north(&mut content);
            count_weight(&content)
        }
        _ => {
            let mut content: Vec<Vec<char>> = file_content.iter().map(|line| line.chars().map(|c| c.clone()).collect()).collect();
            let mut loop_content = Vec::new();
            let mut loop_end = 0;
            while !loop_content.iter().any(|t|  t== &content) {
                loop_content.push(content.clone());
                loop_end += 1;
                content = apply_cylces(&content);
            }
            let loop_start = loop_content.iter().position(|t|  t== &content).unwrap();
            let loop_len = loop_end - loop_start;
            println!("loop_len {}", loop_len);
            let rest = (1000000000 -loop_end) - ((1000000000 -loop_end) / loop_len * loop_len);

            for i in 0..rest {
                content = apply_cylces(&content)
            }
            count_weight(&content)
        }
    };
    println!("Hello, you value is {}", value);
}

fn apply_cylces(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    //allow cache a result
    let mut content: Vec<Vec<char>> = grid.iter().map(|l| l.iter().map(|c| c.clone()).collect()).collect();
    go_north(&mut content);
    go_west(&mut content);
    go_south(&mut content);
    go_east(&mut content);
    content
}


fn go_north(grid: &mut Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
                let mut j = y as i32 - 1;
                while j >= 0 && grid[j as usize][x] == '.' {
                    j -= 1;
                }
                grid[y][x] = '.';
                grid[(j + 1) as usize][x] = 'O';
            }
        }
    }
}

fn go_south(grid: &mut Vec<Vec<char>>) {
    //allow cache a result
    for y in (0..grid.len()).rev()  {
        for x in 0..grid[y].len(){
            if grid[y][x] == 'O' {
                let mut j = y as i32 + 1;
                while j < grid[y].len() as i32 && grid[j as usize][x] == '.' {
                    j += 1;
                }
                grid[y][x] = '.';
                grid[(j - 1) as usize][x] = 'O';
            }
        }
    }
}

fn go_east(grid: &mut Vec<Vec<char>>) {
    for x in (0..grid[0].len()).rev() {
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                let mut j = x as i32 + 1;
                while j < grid[y].len() as i32 && grid[y][j as usize] == '.' {
                    j += 1;
                }
                grid[y][x] = '.';
                grid[y][(j - 1) as usize] = 'O';
            }
        }
    }
}

fn go_west(grid: &mut Vec<Vec<char>>) {
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                let mut j = x as i32 - 1;
                while j >= 0 && grid[y][j as usize] == '.' {
                    j -= 1;
                }
                grid[y][x] = '.';
                grid[y][(j + 1) as usize] = 'O';
            }
        }
    }
}


fn count_weight(grid: &Vec<Vec<char>>) -> usize {
    let size = grid.len();
    grid
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            let l = line.
                iter()
                .fold(0, |sum, ch| {
                    match ch {
                        'O' => {
                            sum + (size - idx)
                        }
                        _ => sum
                    }
                });
            l
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn north() {
        let mut input = vec!(
            ".....#....".chars().collect(),
            "....#...O#".chars().collect(),
            "...OO##...".chars().collect(),
            ".OO#......".chars().collect(),
            ".....OOO#.".chars().collect(),
            ".O#...O#.#".chars().collect(),
            "....O#....".chars().collect(),
            "......OOOO".chars().collect(),
            "#...O###..".chars().collect(),
            "#..OO#....".chars().collect(),
        );

        let mut expected:Vec<Vec<char>> = vec!(
            ".OOO.#.OO.".chars().collect(),
            ".O..#....#".chars().collect(),
            "....O##...".chars().collect(),
            "...#OOO...".chars().collect(),
            "...OO.O.#.".chars().collect(),
            "..#.O.O#O#".chars().collect(),
            ".....#.O.O".chars().collect(),
            "..........".chars().collect(),
            "#....###..".chars().collect(),
            "#....#....".chars().collect(),
        );
        go_north(&mut input);
        assert_eq!(input, expected)
    }


    #[test]
    fn west() {
        let mut input = vec!(
            ".OOO.#.OO.".chars().collect(),
            ".O..#....#".chars().collect(),
            "....O##...".chars().collect(),
            "...#OOO...".chars().collect(),
            "...OO.O.#.".chars().collect(),
            "..#.O.O#O#".chars().collect(),
            ".....#.O.O".chars().collect(),
            "..........".chars().collect(),
            "#....###..".chars().collect(),
            "#....#....".chars().collect(),
        );

        let mut expected:Vec<Vec<char>> = vec!(
            "OOO..#OO..".chars().collect(),
            "O...#....#".chars().collect(),
            "O....##...".chars().collect(),
            "...#OOO...".chars().collect(),
            "OOO.....#.".chars().collect(),
            "..#OO..#O#".chars().collect(),
            ".....#OO..".chars().collect(),
            "..........".chars().collect(),
            "#....###..".chars().collect(),
            "#....#....".chars().collect(),
        );
        go_west(&mut input);
        assert_eq!(input, expected)
    }


    #[test]
    fn south() {
        let mut input = vec!(
            "OOO..#OO..".chars().collect(),
            "O...#....#".chars().collect(),
            "O....##...".chars().collect(),
            "...#OOO...".chars().collect(),
            "OOO.....#.".chars().collect(),
            "..#OO..#O#".chars().collect(),
            ".....#OO..".chars().collect(),
            "..........".chars().collect(),
            "#....###..".chars().collect(),
            "#....#....".chars().collect(),
        );

        let mut expected:Vec<Vec<char>> = vec!(
            ".....#....".chars().collect(),
            "....#.O..#".chars().collect(),
            ".....##...".chars().collect(),
            "..O#......".chars().collect(),
            "O.O....O#.".chars().collect(),
            "O.#..O.#.#".chars().collect(),
            "O....#O...".chars().collect(),
            "O.....OO..".chars().collect(),
            "#O..O###..".chars().collect(),
            "#O.OO#..O.".chars().collect(),
        );
        go_south(&mut input);
        assert_eq!(input, expected)
    }

    #[test]
    fn est() {
        let mut input = vec!(
            ".....#....".chars().collect(),
            "....#.O..#".chars().collect(),
            ".....##...".chars().collect(),
            "..O#......".chars().collect(),
            "O.O..O.O#.".chars().collect(),
            "O.#..O.#.#".chars().collect(),
            "O....#O...".chars().collect(),
            "O.....OO..".chars().collect(),
            "#O..O###..".chars().collect(),
            "#O.OO#..O.".chars().collect(),
        );

        let mut expected:Vec<Vec<char>> = vec!(
            ".....#....".chars().collect(),
            "....#...O#".chars().collect(),
            ".....##...".chars().collect(),
            "..O#......".chars().collect(),
            "....OOOO#.".chars().collect(),
            ".O#...O#.#".chars().collect(),
            "....O#...O".chars().collect(),
            ".......OOO".chars().collect(),
            "#..OO###..".chars().collect(),
            "#.OOO#...O".chars().collect(),
        );
        go_east(&mut input);
        assert_eq!(input, expected)
    }

    #[test]
    fn cycles_test() {
        let input: Vec<Vec<char>> = vec!(
            "OOOO.#.O..".chars().collect(),
            "OO..#....#".chars().collect(),
            "OO..O##..O".chars().collect(),
            "O..#.OO...".chars().collect(),
            "........#.".chars().collect(),
            "..#....#.#".chars().collect(),
            "..O..#.O.O".chars().collect(),
            "..O.......".chars().collect(),
            "#....###..".chars().collect(),
            "#....#....".chars().collect(),
        );
        let expected1: Vec<Vec<char>> = vec!(
            ".....#....".chars().collect(),
            "....#...O#".chars().collect(),
            "...OO##...".chars().collect(),
            ".OO#......".chars().collect(),
            ".....OOO#.".chars().collect(),
            ".O#...O#.#".chars().collect(),
            "....O#....".chars().collect(),
            "......OOOO".chars().collect(),
            "#...O###..".chars().collect(),
            "#..OO#....".chars().collect(),
        );
        let expected2: Vec<Vec<char>> = vec!(
            ".....#....".chars().collect(),
            "....#...O#".chars().collect(),
            ".....##...".chars().collect(),
            "..O#......".chars().collect(),
            ".....OOO#.".chars().collect(),
            ".O#...O#.#".chars().collect(),
            "....O#...O".chars().collect(),
            ".......OOO".chars().collect(),
            "#..OO###..".chars().collect(),
            "#.OOO#...O".chars().collect(),
        );
        let expected3: Vec<Vec<char>> = vec!(
            ".....#....".chars().collect(),
            "....#...O#".chars().collect(),
            ".....##...".chars().collect(),
            "..O#......".chars().collect(),
            ".....OOO#.".chars().collect(),
            ".O#...O#.#".chars().collect(),
            "....O#...O".chars().collect(),
            ".......OOO".chars().collect(),
            "#...O###.O".chars().collect(),
            "#.OOO#...O".chars().collect(),
        );

        let mut state1 = apply_cylces(&input);
        let mut state2 = apply_cylces(&state1);
        let mut state3 = apply_cylces(&state2);
        assert_eq!(state1, expected1);
        assert_eq!(state2, expected2);
        assert_eq!(state3, expected3);
    }
}