use std::fs;
use std::path;
use std::collections::HashMap;

const TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
const VALIDATION_DATA1: &str = "pLPvts";
const VALIDATION_DATA2: &str = "rZ";
const VALIDATION_RESULT1: u32 = 157;
const VALIDATION_RESULT2: u32 = 70;

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

fn parse_data_part1(data: &str) -> String {
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

fn parse_data_part2(data: &str) -> String {
    data
        .lines()
        .collect::<Vec<&str>>()
        .as_slice()
        .chunks(3)
        .map(|lines| {
            let mut candidates: Vec<char> = lines[0]
                .chars()
                .collect::<Vec<char>>();

            for line in &lines[1..] {
                let new_candidates: Vec<char> = candidates
                    .iter()
                    .map(|c| line.find(*c))
                    .filter(|ind| ind.is_some())
                    .map(|ind| line.chars().nth(ind.unwrap()).unwrap())
                    .collect::<Vec<char>>();
                candidates = new_candidates;
            }
            /* Again should only be one item */
            candidates[0]
        })
        .collect::<String>()
}

fn test_function(priority_map: &HashMap<u8, u8>) {
    let test_items: String = parse_data_part1(TEST_DATA);
    println!("!TEST DATA ANALYSIS {} == {}", VALIDATION_DATA1, test_items);
    assert_eq!(test_items, VALIDATION_DATA1.to_string());

    let test_badges: String = parse_data_part2(TEST_DATA);
    println!("!TEST DATA ANALYSIS {} == {}", VALIDATION_DATA2, test_badges);
    assert_eq!(test_badges, VALIDATION_DATA2.to_string());


    let test_item_priorities: Vec<u8> = get_priorities(&test_items, &priority_map);
    let test_item_priorities_sum: u32 = test_item_priorities.iter().map(|&b| b as u32).sum();
    println!("!TEST Summed item priorities: {} !", test_item_priorities_sum);
    assert_eq!(test_item_priorities_sum, VALIDATION_RESULT1);
    
    let test_badge_priorities: Vec<u8> = get_priorities(&test_badges, &priority_map);
    let test_badge_priorities_sum: u32 = test_badge_priorities.iter().map(|&b| b as u32).sum();
    println!("!TEST Summed badge priorities: {} !", test_badge_priorities_sum);
    assert_eq!(test_badge_priorities_sum, VALIDATION_RESULT2);
}

fn main() {
    let priority_map = generate_priority_map();

    test_function(&priority_map);

    let data = read_data();

    let items: String = parse_data_part1(&data);
    let badges: String = parse_data_part2(&data);

    let item_priorities: Vec<u8> = get_priorities(&items, &priority_map);
    let item_priorities_sum: u32 = item_priorities.iter().map(|&b| b as u32).sum();

    let badge_priorities: Vec<u8> = get_priorities(&badges, &priority_map);
    let badge_priorities_sum: u32 = badge_priorities.iter().map(|&b| b as u32).sum();

    println!("Items {}", items);
    println!("Summed item priorities: {}", item_priorities_sum);
    println!("Summed badge priorities: {}", badge_priorities_sum);
}