use std::fs::read_to_string;
use std::env;

enum CountType {
    AllSum,
    Gear
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn is_symbol(char: &char) -> bool {
    !".1234567890".chars().any(|c| c == *char)
}

fn find_number(index: usize, line: &str) -> (usize, (usize, usize)) {
    let mut start = index;
    while start > 0 && line.chars().nth(start - 1).unwrap().is_digit(10) {
        start = start - 1;
    }
    let mut end = index;
    while end < line.len() && line.chars().nth(end ).unwrap().is_digit(10) {
        end = end + 1;
    }
    // println!("{} at ({};{})",line[start..end].parse::<usize>().expect("bad number"), start, end);
    (line[start..end].parse::<usize>().expect("bad number"), (start, end))
}

fn search_and_add_number_at(lines: &Vec<String>, x: usize, y: usize, counted: &mut Vec<(usize, (usize, usize))>) -> usize {
    if lines[x].chars().nth(y).unwrap_or('.').is_digit(10) &&
        !counted.iter().any(|(x1, (y1, y2))| {
            x == *x1 && *y1 <= y && y <= *y2
        })
    {
        let (value, pos) = find_number(y, lines[x].as_str());
        counted.push((x, pos));
        println!("count {}", value);
        value
    } else {
        0
    }
}

fn count(lines: Vec<String>, count_type: CountType) -> usize {
    let mut counted = Vec::new();
    let mut sum = 0;
    for (l, line) in lines.iter().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if is_symbol(&char) {
                let mut acc = Vec::new();
                acc.push(search_and_add_number_at(&lines, l + 1, c - 1, &mut counted));
                acc.push(search_and_add_number_at(&lines, l + 1, c, &mut counted));
                acc.push(search_and_add_number_at(&lines, l + 1, c + 1, &mut counted));
                acc.push(search_and_add_number_at(&lines, l, c - 1, &mut counted));
                acc.push(search_and_add_number_at(&lines, l, c + 1, &mut counted));
                acc.push(search_and_add_number_at(&lines, l - 1, c - 1, &mut counted));
                acc.push(search_and_add_number_at(&lines, l - 1, c, &mut counted));
                acc.push(search_and_add_number_at(&lines, l - 1, c + 1, &mut counted));
                match count_type {
                    CountType::AllSum => { sum += acc.iter().sum::<usize>()}
                    CountType::Gear => {
                        let to_sum: Vec<usize> = acc.into_iter().filter(|i| *i != 0).collect();
                        if to_sum.len() > 1 {
                            sum += to_sum.iter().fold(1, |acc,v| acc * *v);
                        }
                    }
                }
            }
        }
    }
    sum
}

fn main() {
    let path = env::var("FILE").unwrap();
    let part = match env::var("PART").unwrap_or("".to_string()).as_str(){
        "One" => CountType::AllSum,
         _ => CountType::Gear
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
    fn is_symbol_should_be_true() {
        let res= "*/+$@%#=".chars().fold(true,|acc, c| acc && is_symbol(&c));

        assert_eq!(res, true)
    }

    #[test]
    fn count_should_return_4361() {
        let input = vec!(
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string());
        assert_eq!(count(input, CountType::AllSum), 4361);
    }

    #[test]
    fn count_should_return_6666() {
        let input = read_lines("inputs/puzzle_3_1");
        assert_eq!(count(input, CountType::AllSum), 525181);
    }

    fn count_should_return_() {
        let input = read_lines("inputs/puzzle_3_1");
        assert_eq!(count(input, CountType::Gear), 84289137);

    }
}