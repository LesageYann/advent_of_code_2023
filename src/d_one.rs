
use std::fs::read_to_string;
use regex::Regex;
use std::env;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn string_to_number(str: &str) -> u32 {
    match str {
        "one" => {1},
        "two" => {2},
        "three" => {3},
        "four" => {4},
        "five" => {5},
        "six" => {6},
        "seven" => {7},
        "eight" => {8},
        "nine" => {9},
        digit => {
            digit.parse::<u32>().unwrap()
        }
    }
}

fn search_number(line: &str) -> (u32,u32) {
    let regex = Regex::new(r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|\d").unwrap();
    let regex_rev = Regex::new(r"(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)|\d").unwrap();
    let reverse: String = line.chars().rev().collect();
    let str1 = regex.find(line).unwrap().as_str();
    let str2: String = regex_rev.find(&reverse).unwrap().as_str().chars().rev().collect();
    (string_to_number(str1), string_to_number(&str2))
}


fn find_value(file_content: Vec<String>) -> u32{
    file_content
        .into_iter()
        .map(|line| {
            let (start,end)=search_number(&line);
            start *10 + end
        })
        .sum()
}

fn main() {
    let path = env::var("FILE").unwrap();
    let file_content = read_lines(&path);
    let value =  find_value(file_content);
    println!("Hello, you value is {}", value);
}
