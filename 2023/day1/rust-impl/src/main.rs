use std::fs::read_to_string;
use std::path::Path;

extern crate regex;
extern crate ndarray;

use ndarray::{Array};
use regex::{Regex, RegexSet};

const VERBOSE: bool = false;

fn check_string_number(string_pos: &str) -> Option<String> {
    None
}


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


fn part1(lines: &Vec<String>, verbose: bool) {
    let mut numbers = Array::<i32, _>::zeros(lines.len());
    let number_regex: Regex = Regex::new("[0-9]").unwrap();

    for (ind, line) in lines.iter().enumerate() {
        let nums: Vec<_> = number_regex
            .find_iter(line)
            .map(|m| m.as_str())
            .collect();
        let mut num_str = nums[0].to_string();
        num_str.push_str(nums[nums.len() - 1]);
        numbers[ind] = num_str.parse::<i32>().unwrap();
        if verbose {
            println!("{:?} -> {:?}", line, numbers[ind]);
        }
    }
    if verbose {
        println!("{:?}", numbers);
    }
    let total = numbers.sum();
    println!("Part 1 - total: {}", total);
}


fn part2(lines: &Vec<String>, verbose: bool) {
    let mut numbers = Array::<i32, _>::zeros(lines.len());

    let str_num_set = RegexSet::new(&[
        r"0-9",
        r"one",
        r"two",
        r"three",
        r"four",
        r"five",
        r"six",
        r"seven",
        r"eight",
        r"nine",
    ]).unwrap();
    
    let str_num_regexes: Vec<_> = str_num_set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();

    let number_regex: Regex = Regex::new("[0-9]").unwrap();
    
    for (ind, line) in lines.iter().enumerate() {
        let nums: Vec<_> = number_regex
            .find_iter(line)
            .map(|m| m.as_str())
            .collect();
        let mut num_str = nums[0].to_string();
        num_str.push_str(nums[nums.len() - 1]);
        numbers[ind] = num_str.parse::<i32>().unwrap();
        if verbose {
            println!("{:?} -> {:?}", line, numbers[ind]);
        }
    }
    if verbose {
        println!("{:?}", numbers);
    }
    let total = numbers.sum();
    println!("Part 2 - total: {}", total);
}


fn main() {
    let lines: Vec<String> = get_data();
    part1(&lines, VERBOSE);
    // part2(&lines, VERBOSE);
}
