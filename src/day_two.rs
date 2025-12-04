use crate::file;

fn get_range_ids<T: AsRef<str>>(range: &[T]) -> (i64, i64) {
    let left_r = range[0].as_ref().parse::<i64>().unwrap();
    let right_r = range[1].as_ref().parse::<i64>().unwrap();

    (left_r, right_r)
}

fn solve_part_two<T: AsRef<str>>(lines: &[T]) {
    let mut sum = 0;
    for line in lines {
        line.as_ref().split(',').for_each(|range| {
            let ids: Vec<_> = range.split('-').collect();
            let (left_r, right_r) = get_range_ids(&ids);
            for num in left_r..=right_r {
                let id_string = num.to_string();
                let len = id_string.len();

                for seq_len in 1..=(len / 2) {
                    if len % seq_len != 0 {
                        continue;
                    }

                    let seq = &id_string[0..seq_len];
                    let times = len / seq_len;

                    if seq.repeat(times) == id_string {
                        sum += num;
                        break;
                    }
                }
            }
        });
    }

    println!("day_two [2] => {}", sum);
}

fn solve_part_one<T: AsRef<str>>(lines: &[T]) {
    let mut sum = 0;
    for line in lines {
        line.as_ref().split(',').for_each(|range| {
            let ids: Vec<_> = range.split('-').collect();
            let (left_r, right_r) = get_range_ids(&ids);
            for num in left_r..=right_r {
                let num_to_string = num.to_string();
                let (left, right) = num_to_string.split_at(num_to_string.len() / 2);
                if left == right {
                    sum += num;
                };
            }
        });
    }

    println!("day_two [1] => {}", sum);
}

pub fn solve() {
    let lines = file::read_lines("days/2.txt");
    match lines {
        Ok(lines) => {
            let to_vec_lines: Vec<String> = lines.map_while(Result::ok).collect();
            solve_part_one(&to_vec_lines);
            solve_part_two(&to_vec_lines);
        }
        _ => panic!("Could not solve problem"),
    }
}
