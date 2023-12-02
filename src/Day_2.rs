use std::collections::HashMap;

pub fn solution(file_content: String) -> u32 {
    solution_part_one(file_content)
}

fn solution_part_one(file_content: String) -> u32 {
    let allowed_amount = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let lines = file_content.split("\n");
    let mut game_legitimacy: Vec<bool> = vec![true; 100];
    for (game_number, line) in lines.enumerate() {
        if line.is_empty() {
            break;
        }
        let mut line_iter = line.split(": ");
        line_iter.next();
        let game_results = line_iter.next().unwrap();
        let revealed_cubes_this_game: Vec<(&str, &str)> = game_results
            .split(|c: char| c.is_ascii_punctuation())
            .map(|s: &str| {
                let mut iter = s.split_whitespace();
                (iter.next().unwrap(), iter.next().unwrap())
            })
            .collect::<Vec<(&str, &str)>>();
        for colored_cubes in revealed_cubes_this_game {
            if allowed_amount[colored_cubes.1] < colored_cubes.0.parse::<usize>().unwrap() {
                game_legitimacy[game_number] = false;
            }
        }
    }
    game_legitimacy
        .iter()
        .enumerate()
        .fold(0, |acc, b| if *b.1 { acc + b.0 + 1 } else { acc }) as u32
}
