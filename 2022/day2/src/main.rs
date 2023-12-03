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
          Rock Paper Scissors (you)
Rock       3     6      0
Paper      0     3      6
Scissors   6     0      3
(opponent)

betting values
         Rock Paper Scissors
Score      1     2      3
*/
const MATCH_RESULT: [[u32; 3]; 3] = [
    [3,6,0],
    [0,3,6],
    [6,0,3],
];
const MOVE_SCORE: [u32; 3] = [1,2,3];

fn resolve_game(
    p1_move: usize, 
    p2_move: usize, 
) -> (u32, u32) {
    (
        MATCH_RESULT[p2_move][p1_move] + MOVE_SCORE[p1_move],
        MATCH_RESULT[p1_move][p2_move] + MOVE_SCORE[p2_move],
    )
}

fn find_move(p2_move: u32, p1_score: u32) -> u32 {
    let scores = MATCH_RESULT[p2_move as usize];
    let index = scores.iter().position(|&x| x == p1_score).unwrap();
    return index as u32;
}

fn parse_data(data: &str) -> (Vec<u32>, Vec<u32>) {
    let mut move_id: HashMap<char, u32> = HashMap::new();
    let mut outcome_score: HashMap<char, u32> = HashMap::new();
    let mut p1_moves: Vec<u32> = Vec::new();
    let mut p2_moves: Vec<u32> = Vec::new();

    /* Decode input data move ID */
    move_id.insert('A', 0);
    move_id.insert('B', 1);
    move_id.insert('C', 2);

    /* Decode input data outcome score */
    outcome_score.insert('X', 0);
    outcome_score.insert('Y', 3);
    outcome_score.insert('Z', 6);

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();

        let c: char = parts[0].chars().next().unwrap();
        let p2_move: u32 = *move_id.get(&c).unwrap();
        p2_moves.push(p2_move);

        let c: char = parts[1].chars().next().unwrap();
        let p1_score: u32 = *outcome_score.get(&c).unwrap();
        let p1_move: u32 = find_move(p2_move, p1_score);
        p1_moves.push(p1_move);

        /*println!("{}", line);*/
    }

    return (p2_moves, p1_moves)
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