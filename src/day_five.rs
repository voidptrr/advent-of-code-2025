use std::collections::HashSet;

use crate::file;

pub fn solve_part_two<T: AsRef<str>>(input: &[T]) {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut valid = 0;

    for line in input {
        let ref_ = line.as_ref();

        if ref_.trim().is_empty() {
            break;
        }

        let (left, right): (i64, i64) = ref_
            .split_once('-')
            .and_then(|(l, r)| {
                let left_part = l.trim().parse::<i64>().ok()?;
                let right_part = r.trim().parse::<i64>().ok()?;

                Some((left_part, right_part))
            })
            .unwrap();

        ranges.push((left, right));
    }

    ranges.sort_by_key(|(l, _)| *l);
    let mut prev = ranges[0];

    for &(current_start, current_end) in &ranges[1..] {
        if current_start <= prev.1 + 1 {
            prev.1 = prev.1.max(current_end);
        } else {
            valid += (prev.1 - prev.0) + 1;
            prev = (current_start, current_end);
        }
    }

    valid += prev.1 - prev.0 + 1;
    println!("day_five [2] => {}", valid);
}

pub fn solve_part_one<T: AsRef<str>>(input: &[T]) {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut is_ids = false;
    let mut valid = 0;
    let mut seen = HashSet::<i64>::new();

    for line in input {
        let ref_ = line.as_ref();

        if ref_.trim().is_empty() {
            is_ids = true;
            continue;
        }

        if !is_ids {
            let (left, right): (i64, i64) = ref_
                .split_once('-')
                .and_then(|(l, r)| {
                    let left_part = l.trim().parse::<i64>().ok()?;
                    let right_part = r.trim().parse::<i64>().ok()?;

                    Some((left_part, right_part))
                })
                .unwrap();

            ranges.push((left, right));
        } else {
            let n = ref_.parse::<i64>().unwrap();
            for range in ranges.clone() {
                if n >= range.0 && n <= range.1 && !seen.contains(&n) {
                    valid += 1;
                    seen.insert(n);
                }
            }
        }
    }

    println!("day_five [1] => {}", valid);
}

pub fn solve() {
    let lines = file::read_lines("days/5.txt");
    match lines {
        Ok(lines) => {
            let to_vec_lines: Vec<String> = lines.map_while(Result::ok).collect();
            solve_part_one(&to_vec_lines);
            solve_part_two(&to_vec_lines);
        }
        _ => panic!("Could not solve problem"),
    }
}
