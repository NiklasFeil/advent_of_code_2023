use std::cmp;
use std::collections::HashMap;

pub fn solution(file_content: String) -> u32 {
    solution_part_two(file_content)
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

fn calculate_max_number_by_color(cubes: &Vec<(&str, &str)>, color: &str) -> u32 {
    cubes.iter().fold(0, |acc, b| {
        if b.1 != color {
            acc
        } else {
            cmp::max(acc, b.0.parse::<u32>().unwrap())
        }
    })
}

fn solution_part_two(file_content: String) -> u32 {
    let lines = file_content.split("\n");
    let mut power_per_game: Vec<u32> = vec![];
    for line in lines {
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
        let max_number_red = calculate_max_number_by_color(&revealed_cubes_this_game, "red");
        let max_number_green = calculate_max_number_by_color(&revealed_cubes_this_game, "green");
        let max_number_blue = calculate_max_number_by_color(&revealed_cubes_this_game, "blue");
        let power = max_number_red * max_number_green * max_number_blue;
        power_per_game.push(power);
    }

    power_per_game.into_iter().reduce(|a, b| a + b).unwrap()
}
