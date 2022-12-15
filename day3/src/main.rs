use std::fs;
use std::path;
use std::collections::HashMap;

const TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
const VALIDATION_DATA: &str = "pLPvts";
const VALIDATION_RESULT: u32 = 157;

fn generate_priority_map() -> HashMap<u8, u8> {
    (97..=122)
    .chain(65..=90)
    .zip(1..=52)
    .collect::<HashMap<u8, u8>>()
}

fn get_priorities(s: &str, priority_map: &HashMap<u8, u8>) -> Vec<u8> {
    s.as_bytes()
        .iter()
        .map(|c| priority_map[c])
        .collect::<Vec<u8>>()
}

fn read_data() -> String {
    let path = path::Path::new("input.txt");
    return fs::read_to_string(path).unwrap();
}

fn parse_data(data: &str) -> String {
    data
        .lines()
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len()/2);
            let inds: Vec<usize> = comp2
                .chars()
                .map(|c| comp1.find(c))
                .filter(|ind| ind.is_some())
                .map(|ind| ind.unwrap())
                .collect::<Vec<usize>>();
            /* This should be just one common item so we can take the first */
            comp1.chars().nth(inds[0]).unwrap()
        })
        .collect::<String>()
}

fn main() {
    let priority_map = generate_priority_map();
    let data = read_data();

    let test_items: String = parse_data(TEST_DATA);
    println!("!TEST DATA ANALYSIS {} == {}", VALIDATION_DATA, test_items);
    assert_eq!(test_items, VALIDATION_DATA.to_string());

    let test_priorities: Vec<u8> = get_priorities(&test_items, &priority_map);
    let test_priorities_sum: u32 = test_priorities.iter().map(|&b| b as u32).sum();
    println!("!TEST Summed priorities: {} !", test_priorities_sum);
    assert_eq!(test_priorities_sum, VALIDATION_RESULT);
    
    
    let items: String = parse_data(&data);
    let priorities: Vec<u8> = get_priorities(&items, &priority_map);
    let priorities_sum: u32 = priorities.iter().map(|&b| b as u32).sum();

    println!("Items {}", items);
    println!("Summed priorities: {}", priorities_sum);
}