pub fn solution(file_content: String) -> u32 {
    solution_part_one(file_content)
}

fn solution_part_one(file_content: String) -> u32 {
    let lines = file_content.split("\n"); // Iterator over Strings
    let lines = lines
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>(); // Vector of Vectors of &str
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j].is_digit(10) {
                let first_number_index = j;
            }
        }
    }
    8
}
