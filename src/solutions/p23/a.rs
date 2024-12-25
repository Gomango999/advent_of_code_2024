use super::parser;
use itertools::Itertools;
use std::collections::HashMap;

struct Graph {
    adj: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: String, v: String) {
        self.adj
            .entry(u.clone())
            .or_insert_with(Vec::new)
            .push(v.clone());
        self.adj.entry(v).or_insert_with(Vec::new).push(u);
    }
}

pub fn solve() {
    let connections = parser::parse();
    let mut graph = Graph::new();

    for (u, v) in connections {
        graph.add_edge(u, v);
    }

    let mut count = 0;
    for (u, vs) in &graph.adj {
        if !u.starts_with('t') {
            continue;
        }

        for pair in vs.iter().combinations(2) {
            let v1 = pair[0];
            let v2 = pair[1];

            // Only consider other "t"s from the perspective of the node which
            // is lexicographically smallest. This prevents double counting.
            if v1.starts_with('t') && u >= v1 {
                continue;
            }
            if v2.starts_with('t') && u >= v2 {
                continue;
            }

            if graph.adj[v1].contains(v2) {
                count += 1;
            }
        }
    }
    println!("{count}");
}
