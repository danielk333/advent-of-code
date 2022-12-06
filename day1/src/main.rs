use std::fs::read_to_string;
use std::io::{Error};
use std::path::Path;

/*
Oh my god this code is so horrible...
i feel like a toddler who is just learning how to use my hands

*/
fn main() -> Result<(), Error> {
    let path = Path::new("input.txt");

    /*the ? operator is fantastic*/
    let data = read_to_string(path)?;
    let mut vec = Vec::new();
    let mut current_val: i32 = 0;

    for line in data.lines() {
        if line.is_empty() {
            vec.push(current_val);
            current_val = 0;
            println!("PUSHING VALUE!");
        }
        else {
            /* now im getting the whole ::<> thing, its just templates again! */
            current_val += line.parse::<i32>().unwrap();
        }
        println!("{}", current_val);
    }
    if current_val != 0 {
        vec.push(current_val);
        println!("PUSHING VALUE!");
    }

    /* it took me SO long to figure out unwrap is ? for Options*/
    let max_value = vec.iter().max().unwrap(); 

    println!("MAX VALUE: {}", max_value);

    vec.sort();

    let top_number = 3;
    let n = vec.len();
    let top_vals = vec[(n - top_number - 1)..(n - 1)].iter().sum::<i32>();

    println!("SUM OF TOP {}: {}", top_number, top_vals);

    Ok(()) /* nice */
}
