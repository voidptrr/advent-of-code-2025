use crate::file;

pub fn solve_part_two(lines: &Vec<String>) {
    let mut sum = 50;
    let mut combination = 0;
    let mut start_at_zero = false;
    for line in lines {
        let ref_line = line.as_str();
        let (rotation, value) = ref_line.split_at(1);

        let p_value = value.parse::<i32>().unwrap();
        match rotation {
            "L" => sum -= p_value,
            "R" => sum += p_value,
            _ => panic!("Invalid rotation symbol"),
        };

        let steps = sum.abs() / 100; // Total turns
        combination += steps;

        if sum < 0 && !start_at_zero {
            combination += 1;
        }

        if sum == 0 {
            combination += 1;
        }

        sum = sum.rem_euclid(100);
        start_at_zero = sum == 0;
    }
    println!("day_one [2] => {}", combination);
}

pub fn solve_part_one(lines: &Vec<String>) {
    let mut sum = 50;
    let mut combination = 0;
    for line in lines {
        let ref_line = line.as_str();
        let (rotation, value) = ref_line.split_at(1);

        let p_value = value.parse::<i32>().unwrap();

        match rotation {
            "L" => sum -= p_value,
            "R" => sum += p_value,
            _ => panic!("Invalid rotation symbol"),
        }

        sum = sum.rem_euclid(100);

        if sum == 0 {
            combination += 1;
        }
    }
    println!("day_one [1] => {}", combination);
}

pub fn solve() {
    let lines = file::read_lines("days/1.txt");
    match lines {
        Ok(lines) => {
            let to_vec_lines: Vec<String> = lines.map_while(Result::ok).collect();
            solve_part_one(&to_vec_lines);
            solve_part_two(&to_vec_lines);
        }
        _ => panic!("Could not solve problem"),
    }
}
