mod Day_1;
mod Day_2;
mod Day_5;
mod Day_6;
mod Day_7;

use std::fs;

struct AoCSolver {
    day: u8,
}

impl AoCSolver {
    fn read_file_content(&self) -> String {
        let file_path: String = format!("data/{}.txt", self.day);
        let contents = fs::read_to_string(file_path).expect("Unable to read file");
        contents
    }

    fn solve(&self) -> u64 {
        let file_content = self.read_file_content();
        Day_7::solution(file_content)
    }
}

fn main() {
    let solver: AoCSolver = AoCSolver { day: 7 };
    let solution = solver.solve();
    println!("{:?}", solution);
}
