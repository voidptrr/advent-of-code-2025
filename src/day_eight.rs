use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::file;

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn get_euclidean_distance(&self, other: &JunctionBox) -> i64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        dx * dx + dy * dy + dz * dz
    }
}

struct Circuit {
    box_sets: Vec<HashSet<usize>>,
    box_lookup: HashMap<usize, usize>,
}

impl Circuit {
    fn add_pair_to_set(&mut self, pair: usize, set_index: usize) {
        let s = &mut self.box_sets[set_index];
        s.insert(pair);
        self.box_lookup.insert(pair, set_index);
    }

    fn new_set(&mut self) -> usize {
        let s = HashSet::<usize>::new();
        let set_id = self.box_sets.len();

        self.box_sets.push(s);
        set_id
    }

    fn merge_sets(&mut self, src: usize, dst: usize) -> bool {
        if src == dst {
            return false;
        }
        let (src, dst) = if self.box_sets[src].len() > self.box_sets[dst].len() {
            (dst, src)
        } else {
            (src, dst)
        };
        let source_values: Vec<_> = self.box_sets[src].drain().collect();

        for el in source_values {
            self.box_sets[dst].insert(el);
            self.box_lookup.insert(el, dst);
        }

        true
    }

    fn connect_boxes(&mut self, pair_1: usize, pair_2: usize) -> bool {
        let pair_1_exists = self.box_lookup.get(&pair_1);
        let pair_2_exists = self.box_lookup.get(&pair_2);

        match (pair_1_exists, pair_2_exists) {
            (None, None) => {
                let new_set = self.new_set();
                self.add_pair_to_set(pair_1, new_set);
                self.add_pair_to_set(pair_2, new_set);
                true
            }
            (None, Some(&set_id)) => {
                self.add_pair_to_set(pair_1, set_id);
                true
            }
            (Some(&set_id), None) => {
                self.add_pair_to_set(pair_2, set_id);
                true
            }
            (Some(&set_1), Some(&set_2)) if set_1 != set_2 => self.merge_sets(set_1, set_2),
            _ => false, // both in same set already
        }
    }
}

fn solve_both<T: AsRef<str>>(input: &[T]) -> (usize, usize) {
    let mut bh: BinaryHeap<(Reverse<i64>, (usize, usize))> = BinaryHeap::new();

    let boxes: Vec<JunctionBox> = input
        .iter()
        .map(|line| {
            let line_ref = line.as_ref();
            let numbers: Vec<i64> = line_ref
                .split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            assert_eq!(numbers.len(), 3);

            JunctionBox {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
        .collect();

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            bh.push((Reverse(boxes[i].get_euclidean_distance(&boxes[j])), (i, j)));
        }
    }

    let mut circuit = Circuit {
        box_sets: Vec::new(),
        box_lookup: HashMap::new(),
    };

    let mut num_sets = boxes.len();
    let mut result_part_two = 0;
    let mut result_part_one = 0;
    let mut index = 0;
    while let Some((_, (a, b))) = bh.pop() {
        if index == 1000 {
            let n = 3;
            circuit.box_sets.sort_by_key(|b| std::cmp::Reverse(b.len()));

            let top_sets = &circuit.box_sets[..n];
            let product_of_lengths: usize = top_sets.iter().map(|s| s.len()).product();
            result_part_one += product_of_lengths;
        }
        index += 1;
        if circuit.connect_boxes(a, b) {
            num_sets -= 1; // two sets merged
            if num_sets == 1 {
                result_part_two += boxes[a].x as usize * boxes[b].x as usize;
                break;
            }
        }
    }

    (result_part_one, result_part_two)
}

pub fn solve() {
    let lines = file::read_lines("days/8.txt");
    match lines {
        Ok(input) => {
            let to_vec_lines: Vec<String> = input.map_while(Result::ok).collect();
            let (part_one, part_two) = solve_both(&to_vec_lines);
            println!("day_eight [1] => {}", part_one);
            println!("day_eight [2] => {}", part_two);
        }
        _ => panic!("Error reading file"),
    }
}
