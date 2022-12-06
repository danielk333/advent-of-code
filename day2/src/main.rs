use std::fs;
use std::path;
use std::collections::HashMap;

/* This is gonna be used alot */
fn read_data() -> String {
    let path = path::Path::new("input.txt");
    return fs::read_to_string(path).unwrap();
}

/*
match outcome matrix
          Rock Paper Scissors (opponent)
Rock       3     0      6
Paper      6     3      0
Scissors   0     6      3
(you)

betting values
         Rock Paper Scissors
Score      1     2      3
*/
const MATCH_RESULT: [[u32; 3]; 3] = [
    [3,0,6],
    [6,3,0],
    [0,6,3],
];
const MOVE_SCORE: [u32; 3] = [1,2,3];

fn resolve_game(
    p1_move: usize, 
    p2_move: usize, 
) -> (u32, u32) {
    (
        MATCH_RESULT[p1_move][p2_move] + MOVE_SCORE[p1_move],
        MATCH_RESULT[p2_move][p1_move] + MOVE_SCORE[p2_move],
    )
}

fn parse_data(data: &str) -> (Vec<u32>, Vec<u32>) {
    let mut move_translate: HashMap<char, u32> = HashMap::new();
    let mut opponent_move: Vec<u32> = Vec::new();
    let mut my_move: Vec<u32> = Vec::new();

    /* Map to translate into consistent game action IDs */
    move_translate.insert('X', 0);
    move_translate.insert('Y', 1);
    move_translate.insert('Z', 2);

    move_translate.insert('A', 0);
    move_translate.insert('B', 1);
    move_translate.insert('C', 2);

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();

        let c: char = parts[0].chars().next().unwrap();
        opponent_move.push(*move_translate.get(&c).unwrap());

        let c: char = parts[1].chars().next().unwrap();
        my_move.push(*move_translate.get(&c).unwrap());
    }

    return (opponent_move, my_move)
}

fn main() {
    let data = read_data();
    let (opponent_move, my_move) = parse_data(&data);

    let mut my_score: u32 = 0;

    /* Im player 1 */
    let it = my_move.iter().zip(opponent_move.iter());
    for (c1, c2) in it {
        let (p1_sc, p2_sc) = resolve_game(*c1 as usize, *c2 as usize);
        println!("you-{} vs them-{} -> you-{} vs them-{}", c1, c2, p1_sc, p2_sc);
        
        my_score += p1_sc;
    }

    println!("my score: {}", my_score);
}