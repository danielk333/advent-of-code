use std::fs::read_to_string;
use std::path::Path;
use std::cmp;

const MAX_GAMES: usize = 100;


fn get_data() -> Vec<String> {
    let path = Path::new("input");
    let data: String = read_to_string(path).unwrap();
    let lines: Vec<String> = data
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    return lines;
}


fn parse_game(game_str: &str) -> (i32, i32, i32) {
    let mut rgb: [i32; 3] = [0; 3];
    let mut num: i32;
    // println!("GAME: {}", game_str);

    for part in game_str.rsplit(",") {
        let str_part = part.trim();
        let keyval: Vec<&str> = str_part.split(" ").collect();
        num = keyval[0].parse::<i32>().unwrap();
        match keyval[1] {
            "red" => rgb[0] = num,
            "green" => rgb[1] = num,
            "blue" => rgb[2] = num,
            _ => panic!("wat"),
        }
    }
    return (rgb[0], rgb[1], rgb[2]);
}


fn part1(lines: &Vec<String>) {
    let mut id: i32;
    let mut g_sep_stop: usize;
    let mut separator_pos: usize = 0;
    let mut game_stat_index: usize;
    /* Scuffed way to do it but quite efficient with pre-initialized memory */
    let mut game_seps: [usize; MAX_GAMES] = [0; MAX_GAMES];
    let mut red: [i32; MAX_GAMES] = [0; MAX_GAMES];
    let mut green: [i32; MAX_GAMES] = [0; MAX_GAMES];
    let mut blue: [i32; MAX_GAMES] = [0; MAX_GAMES];

    let elf_question: Vec<i32> = vec![12, 13, 14];
    let mut min_cubes: [i32; 3] = [0; 3];
    let mut summed_powers: i32 = 0;
    let mut game_power: i32;
    
    let mut possible_games: i32 = 0;
    let mut id_sum: i32 = 0;
    let mut possible_game: bool;

    for line in lines {
        game_stat_index = 0;
        for ci in 0..3 {
            min_cubes[ci] = 0;
        }
        for (ind, ch) in line.chars().enumerate() {
            match ch {
                ':' => separator_pos = ind,
                ';' => {
                    game_seps[game_stat_index] = ind;
                    game_stat_index += 1;
                },
                _ => (),
            };
        }
        if game_stat_index == 0 {
            game_seps[0] = line.len();
        }
        g_sep_stop = game_stat_index;

        id = (&line[5..separator_pos]).parse::<i32>().unwrap();
        // First game
        (red[0], green[0], blue[0]) = parse_game(&line[(separator_pos + 1)..game_seps[0]]);
        if g_sep_stop > 1 {
            for ind in 0..(g_sep_stop - 1) {
                (red[ind + 1], green[ind + 1], blue[ind + 1]) = parse_game(&line[(game_seps[ind] + 1)..game_seps[ind + 1]]);
            }
        }
        if g_sep_stop > 0 {
            // Last game
            (red[g_sep_stop], green[g_sep_stop], blue[g_sep_stop]) = parse_game(&line[(game_seps[g_sep_stop - 1] + 1)..line.len()]);
        }
        possible_game = true;
        for ind in 0..(g_sep_stop + 1) {
            min_cubes[0] = cmp::max(min_cubes[0], red[ind]);
            min_cubes[1] = cmp::max(min_cubes[1], green[ind]);
            min_cubes[2] = cmp::max(min_cubes[2], blue[ind]);
            if red[ind] > elf_question[0] || green[ind] > elf_question[1] || blue[ind] > elf_question[2] {
                possible_game = false; 
            }
        }
        if possible_game {
            possible_games += 1;
            id_sum += id;
        }

        game_power = min_cubes.iter().product::<i32>();
        summed_powers += game_power;

        // println!("{line}");
        println!("->{}({} games, {} power) possible={}", id, g_sep_stop + 1, game_power, possible_game);

    }

    println!("possible_games = {possible_games}");
    println!("ID sum = {id_sum}");
    println!("Summed powers = {summed_powers}");

}


fn main() {
    let lines: Vec<String> = get_data();
    part1(&lines);
}
