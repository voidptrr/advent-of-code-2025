use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::file;

#[derive(Debug)]
struct Node {
    id: String,
    neighbors: HashSet<String>,
}

#[derive(Debug)]
struct ParseNodeError;
impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(":").unwrap();

        let id = parts.0.trim();
        let neighors: Vec<String> = parts.1.trim().split(" ").map(|id| id.into()).collect();

        let mut set = HashSet::<String>::new();

        for n in neighors {
            set.insert(n);
        }

        Ok(Node {
            id: id.into(),
            neighbors: set,
        })
    }
}

fn solve_part_two<T: AsRef<str>>(input: &[T]) -> usize {
    fn dfs<'a>(
        node: &'a str,
        node_map: &'a HashMap<&str, HashSet<String>>,
        seen_dac: bool,
        seen_fft: bool,
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
    ) -> usize {
        if node == "out" {
            if seen_dac && seen_fft {
                return 1;
            }

            return 0;
        }

        if let Some(d) = cache.get(&(node, seen_dac, seen_fft)) {
            return *d;
        }

        let is_dac = node == "dac";
        let is_fft = node == "fft";

        let mut count = 0;
        if let Some(neighbors) = node_map.get(node) {
            for next in neighbors {
                count += dfs(
                    next.as_ref(),
                    node_map,
                    seen_dac || is_dac,
                    seen_fft || is_fft,
                    cache,
                );
            }
        }
        cache.insert((node, seen_dac, seen_fft), count);
        count
    }
    let nodes: Vec<Node> = input
        .iter()
        .map(|l| {
            let r = l.as_ref();
            Node::from_str(r).unwrap()
        })
        .collect();

    let node_map: HashMap<_, _> = nodes
        .iter()
        .map(|n| (n.id.as_str(), n.neighbors.clone()))
        .collect();
    let mut memo = HashMap::new();
    dfs("svr", &node_map, false, false, &mut memo)
}

fn solve_part_one<T: AsRef<str>>(input: &[T]) -> usize {
    fn dfs<'a>(
        node: &'a Node,
        nodes: &'a [Node],
        path_seen: &mut HashSet<String>,
        count: &mut usize,
    ) {
        for id in &node.neighbors {
            if *id == "out" {
                *count += 1;
                continue;
            }

            if path_seen.contains(id) {
                continue;
            }

            let next = nodes.iter().find(|n| n.id == *id).unwrap();

            path_seen.insert(id.clone());
            dfs(next, nodes, path_seen, count);
            path_seen.remove(id);
        }
    }

    let nodes: Vec<Node> = input
        .iter()
        .map(|l| {
            let r = l.as_ref();
            Node::from_str(r).unwrap()
        })
        .collect();

    let start = nodes.iter().find(|n| n.id == "you").unwrap();

    let mut count = 0;
    let mut path_seen = HashSet::new();
    path_seen.insert("you".to_string());

    dfs(start, &nodes, &mut path_seen, &mut count);

    count
}

pub fn solve() {
    let lines = file::read_lines("days/11.txt");
    match lines {
        Ok(input) => {
            let to_vec_lines: Vec<String> = input.map_while(Result::ok).collect();
            println!("day_eleven [1] => {}", solve_part_one(&to_vec_lines));
            println!("day_eleven [2] => {}", solve_part_two(&to_vec_lines));
        }
        _ => panic!("Error reading file"),
    }
}
