use crate::file;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug)]
struct Problem {
    operation: Operation,
    numbers: Vec<u64>,
}

impl Problem {
    fn get_count(self: &Self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
        }
    }
}

fn generate_problems<T: AsRef<str>>(input: &[T]) -> Vec<Problem> {
    let operation_line = input[input.len() - 1].as_ref().trim();
    let operations: Vec<Operation> = operation_line
        .split_ascii_whitespace()
        .map(|op| match op {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => panic!("Invalid operation"),
        })
        .collect();

    let mut groups: Vec<Problem> = Vec::with_capacity(operations.len());
    let mut result: Vec<u64> = Vec::new();

    for number_line in &input[0..input.len() - 1] {
        let line_ref = number_line.as_ref().trim();

        let mut numbers: Vec<u64> = line_ref
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        result.append(&mut numbers);
    }

    let mut j = 0;
    while j < operations.len() {
        let mut inner_index = j;
        let mut g = Problem {
            numbers: Vec::new(),
            operation: operations[j],
        };

        while inner_index < result.len() {
            g.numbers.push(result[inner_index]);
            inner_index += operations.len();
        }
        groups.push(g);
        j += 1;
    }

    groups
}

fn solve_part_one(problems: &[Problem]) -> u64 {
    problems.iter().map(|p| p.get_count()).sum()
}

fn solve_part_two<T: AsRef<str>>(input: &[T]) -> u64 {
    let height = input.len();
    let width = input.iter().map(|line| line.as_ref().len()).max().unwrap();

    let mut buffer = vec![b' '; width * height];

    for (chunk, line) in buffer.chunks_exact_mut(width).zip(input.iter()) {
        let bytes_line = line.as_ref().as_bytes();
        let len = bytes_line.len().min(width);
        chunk[..len].copy_from_slice(&bytes_line[..len]);
    }

    let digits = (buffer.len() - width) / width;
    let operators = &buffer[buffer.len() - width..];
    let mut operator_start: usize = 0;
    let mut total_total: u64 = 0;
    loop {
        let mut x = operator_start;
        println!("op_start: {}", operator_start);
        if operator_start >= operators.len() {
            break;
        }
        let operator_end = operators[operator_start + 1..]
            .iter()
            .position(|x| !x.is_ascii_whitespace());
        let next_operator_index = if let Some(pos) = operator_end {
            operator_start + 1 + pos
        } else {
            operators.len()
        };
        let mut total: u64 = 0;
        let op_value = operators[operator_start];

        while x < next_operator_index {
            let mut result: u64 = 0;

            for y in 0..digits {
                let data = buffer[width * y + x];
                if data.is_ascii_whitespace() {
                    continue;
                };
                result *= 10;
                result += (data - b'0') as u64;
            }

            if result == 0 {
                x += 1;
                continue;
            }
            println!("number column is {}", result);

            println!("Operator {}", op_value as char);
            match op_value {
                b'*' => {
                    if total == 0 {
                        total = 1;
                    };
                    total *= result
                }
                b'+' => total += result,
                _ => panic!("wrong op"),
            }
            x += 1;
        }
        total_total += total;
        operator_start = next_operator_index;
    }
    total_total
}

pub fn solve() {
    let lines = file::read_lines("days/6.txt");
    match lines {
        Ok(lines) => {
            let to_vec_lines: Vec<String> = lines.map_while(Result::ok).collect();
            let problems = generate_problems(&to_vec_lines);
            println!("day_six [1] => {}", solve_part_one(&problems));
            println!("day_six [2] => {}", solve_part_two(&to_vec_lines));
        }
        _ => panic!("Could not solve problem"),
    }
}
