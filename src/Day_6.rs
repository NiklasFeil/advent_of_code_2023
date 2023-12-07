pub fn solution(file_content: String) -> u64 {
    solution_part_two(file_content)
}

fn solution_part_one(file_content: String) -> u32 {
    let mut lines = file_content.lines();
    let times = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| {
            let mut valid_charge_times_count: u32 = 0;
            for charge_time in 1..time {
                if (time - charge_time) * charge_time > distance {
                    valid_charge_times_count += 1;
                }
            }
            valid_charge_times_count
        })
        .reduce(|acc, b| acc * b)
        .unwrap()
}

fn solution_part_two(file_content: String) -> u64 {
    let mut lines = file_content.lines();
    let time = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .ok()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .ok()
        .unwrap();
    let mut valid_charge_times_count = 0;
    for charge_time in 1..time {
        if (time - charge_time) * charge_time > distance {
            valid_charge_times_count += 1;
        }
    }
    valid_charge_times_count
}
