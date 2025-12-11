use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use crate::file;

struct Diagram {
    seen_state: HashSet<u16>,
    target: u16,
    button_wiring: Vec<u16>,
}

impl Diagram {
    fn apply_masks(&mut self) -> usize {
        let mut queue = VecDeque::new();

        queue.push_back((0u16, 0));
        self.seen_state.insert(0u16);

        while let Some((state, presses)) = queue.pop_front() {
            if state == self.target {
                return presses;
            }

            for mask in &self.button_wiring {
                let next_state = state ^ mask;
                if self.seen_state.insert(next_state) {
                    queue.push_back((next_state, presses + 1));
                }
            }
        }

        usize::MAX
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDiagramError;

impl FromStr for Diagram {
    type Err = ParseDiagramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_target_position = s.chars().position(|c| c == '[').unwrap();
        let end_target_position = s.chars().position(|c| c == ']').unwrap();
        let target_ref = &s[start_target_position + 1..end_target_position];
        let mut target: u16 = 0;

        for (i, c) in target_ref.chars().enumerate() {
            if c == '#' {
                target |= 1 << i;
            }
        }

        let start_button_position = s.chars().position(|c| c == '(').unwrap();
        let end_button_position = s.chars().position(|c| c == '{').unwrap();

        let slice_buttons = &s[start_button_position..end_button_position - 1];
        let buttons: Vec<_> = slice_buttons.split(" ").collect();
        let bit_masks: Vec<u16> = buttons
            .iter()
            .map(|b| {
                let s = b.trim_matches(|c| c == '(' || c == ')');
                s.split(',')
                    .map(|x| 1 << x.trim().parse::<u16>().unwrap())
                    .fold(0, |acc, x| acc | x)
            })
            .collect();

        Ok(Diagram {
            seen_state: HashSet::new(),
            button_wiring: bit_masks,
            target,
        })
    }
}

fn solve_part_one<T: AsRef<str>>(input: &[T]) -> usize {
    input
        .iter()
        .filter_map(|l| {
            let line_ref = l.as_ref();
            Diagram::from_str(line_ref)
                .ok()
                .map(|mut diagram| diagram.apply_masks())
        })
        .sum()
}

pub fn solve() {
    let lines = file::read_lines("days/10.txt");
    match lines {
        Ok(input) => {
            let to_vec_lines: Vec<String> = input.map_while(Result::ok).collect();
            println!("day_ten [1] => {}", solve_part_one(&to_vec_lines));
        }
        _ => panic!("Error reading file"),
    }
}
