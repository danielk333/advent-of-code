use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::Path;

/*
Oh my god this code is so horrible...
i feel like a toddler who is just learning how to use my hands

*/
fn main() -> Result<(), Error> {
    let path = Path::new("input.txt");
    let input = File::open(&path)?;

    let buffered = BufReader::new(input);
    let mut vec = Vec::new();
    let mut current_val: i32 = 0;

    for line in buffered.lines() {
        let line_checked = line?;
        if line_checked.len() == 0 {
            vec.push(current_val);
            current_val = 0;
            println!("PUSHING VALUE!");
        }
        else {
            current_val += line_checked.parse::<i32>().unwrap();
        }
        println!("{}", current_val);
    }

    let max_value = vec.iter().max().unwrap();

    println!("MAX VALUE: {}", max_value);

    vec.sort();

    let top_number = 3;
    let mut top_vals: i32 = 0;
    for n in 0..top_number {
        println!("adding {} to the sum...", vec[vec.len() - n - 1]);
        top_vals += vec[vec.len() - n - 1];
    }

    println!("SUM OF TOP {}: {}", top_number, top_vals);

    Ok(())
}
