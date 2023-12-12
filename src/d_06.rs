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
    let (tomes, dists)=  match env::var("PART").unwrap_or("".to_string()).as_str() {
        "One" => get_grid(&file_content),
        _ => {
            get_one_time_grid(&file_content)
        }
    };

    let solutions_by_run = get_solution_by_run(tomes, dists);
    let value = solutions_by_run.into_iter().reduce(|acc, i| acc * i).unwrap();
    println!("Hello, you value is {}", value);
}

fn get_solution_by_run(times:Vec<i64>, dists: Vec<i64>) -> Vec<i64>{
    (0..times.len()).map(|idx|{
        let mut min_time = 0;
        while min_time <= times[idx] {
            if min_time * (times[idx]-min_time) > dists[idx] {
                break
            } else {
                min_time += 1;
            }
        }
        let mut max_time = times[idx];
        while max_time >= 0 {
            if max_time * (times[idx]-max_time) > dists[idx] {
                break
            } else {
                max_time -= 1;
            }
        }
        if max_time >= min_time {
            max_time - min_time + 1
        } else {
            0
        }
    }).collect::<Vec<i64>>()
}

fn parse_line(line: &String)-> Vec<i64> {
    line.split(":").nth(1).unwrap().split(" ")
        .filter(|item| item != &" " && !item.is_empty())
        .map(|item| {
            item.parse::<i64>().unwrap()
        }).collect::<Vec<i64>>()
}

fn get_grid(lines: &Vec<String>) -> (Vec<i64>,Vec<i64>){
    (
        parse_line(&lines[0]),
        parse_line(&lines[1])
    )
}

fn parse_line_one_result(line: &String)-> Vec<i64> {
    let result = line.split(":").nth(1).unwrap().split(" ")
        .filter(|item| item != &" " && !item.is_empty())
        .collect::<Vec<&str>>()
        .join("");
    println!("result {}", result);
    vec!(result.parse::<i64>().unwrap())
}
fn get_one_time_grid(lines: &Vec<String>) -> (Vec<i64>,Vec<i64>){
    (
        parse_line_one_result(&lines[0]),
        parse_line_one_result(&lines[1])
    )
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn parse_get_grid() {
        let input = vec!(
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        );
        assert_eq!(get_grid(&input), (vec![7,15,30],vec![9,40, 200]));
    }

    #[test]
    fn get_solution_by_run_should_return_arr() {
        let result = get_solution_by_run(vec![7,15,30],vec![9,40, 200]);
        assert_eq!(result, vec!(4,8,9));
    }


}