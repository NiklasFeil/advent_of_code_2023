use std::collections::HashMap;

pub fn solution(file_content: String) -> u32 {
    solution_part_one(file_content)
}

fn solution_part_one(file_content: String) -> u32 {
    let lines = file_content.lines();
    let mut numbers: Vec<u32> = vec![];
    for line in lines {
        if line.is_empty() {
            break;
        }

        let mut first_number: u32 = 0;
        let mut second_number: u32 = 0;
        for c in line.chars() {
            match c.to_digit(10) {
                Some(n) => {
                    first_number = n;
                    break;
                }
                None => {}
            };
        }
        for c in line.chars().rev() {
            match c.to_digit(10) {
                Some(n) => {
                    second_number = n;
                    break;
                }
                None => {}
            };
        }
        numbers.push(first_number * 10 + second_number);
    }
    numbers.into_iter().reduce(|a, b| a + b).unwrap()
}

fn solution_part_two(file_content: String) -> u32 {
    let lines = file_content.lines();
    let mut numbers: Vec<u32> = vec![];
    let digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    for line in lines {
        let mut digit_occurences: Vec<(usize, &str)> = vec![];
        if line.is_empty() {
            break;
        }

        for digit in digits.keys() {
            digit_occurences.extend(line.match_indices(digit));
        }
        digit_occurences.sort_by(|a, b| a.0.cmp(&b.0));
        let first_number = digits[digit_occurences[0].1];
        let second_number = digits[digit_occurences[digit_occurences.len() - 1].1];
        numbers.push(first_number * 10 + second_number);
    }
    numbers.into_iter().reduce(|a, b| a + b).unwrap()
}
