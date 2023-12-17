use std::fs::read_to_string;
use std::path::Path;
use std::time::Instant;

extern crate regex;

use regex::{Regex};


fn get_data() -> Vec<String> {
    let path = Path::new("../input");
    let data: String = read_to_string(path).unwrap();
    let lines: Vec<String> = data
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    return lines
}


fn part1(lines: &Vec<String>) {
    let mut numbers = vec![0; lines.len()];
    let number_regex: Regex = Regex::new("[0-9]").unwrap();

    for (ind, line) in lines.iter().enumerate() {
        let nums: Vec<_> = number_regex
            .find_iter(line)
            .map(|m| m.as_str())
            .collect();
        let mut num_str = nums[0].to_string();
        num_str.push_str(nums[nums.len() - 1]);
        numbers[ind] = num_str.parse::<i32>().unwrap();
    }
    let total = numbers.iter().sum::<i32>();
    println!("Part 1 - total: {}", total);
}


fn part2(lines: &Vec<String>) {
    let mut numbers = vec![0; lines.len()];

    for (ind, line) in lines.iter().enumerate() {
        let line_len = line.len();
        if line_len == 0 {
            continue
        }
        let byte_line = line.as_bytes();
        let mut nums: Vec<char> = Vec::new();
        let mut ch: u32;
        let mut res: char;
        for pos in 0..line_len {
            ch = byte_line[pos] as u32;
            if ch >= 48 && ch <= 57 {
                nums.push(char::from_u32(ch).unwrap());
                continue
            }
            if pos + 2 >= line_len { continue }
            res = match &line[pos..pos+3] {
                "one" => '1',
                "two" => '2',
                "six" => '6',
                _ => '-',
            };
            if res != '-' { nums.push(res); continue }
            else if pos + 3 >= line_len { continue }
            res = match &line[pos..pos+4] {
                "four" => '4',
                "five" => '5',
                "nine" => '9',
                _ => '-',
            };
            if res != '-' { nums.push(res); continue }
            else if pos + 4 >= line_len { continue }
            res = match &line[pos..pos+5] {
                "three" => '3',
                "seven" => '7',
                "eight" => '8',
                _ => '-',
            };
            if res != '-' { nums.push(res) }
        }
        let mut num_str: String = nums[0].to_string();
        num_str.push(nums[nums.len() - 1]);

        numbers[ind] = num_str.parse::<i32>().unwrap();
    }
    let total = numbers.iter().sum::<i32>();
    println!("Part 2 - total: {}", total);
}


fn main() {
    let lines: Vec<String> = get_data();

    // let mut now = Instant::now();
    part1(&lines);

    // let mut elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    
    // now = Instant::now();
    // part2(&lines);

    // elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
