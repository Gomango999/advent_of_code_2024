use super::parser;
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

/// Written by ChatGPT
fn find_largest_clique(graph: &Graph) -> Vec<String> {
    fn is_clique(graph: &Graph, nodes: &[String]) -> bool {
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                if !graph.adj[&nodes[i]].contains(&nodes[j]) {
                    return false;
                }
            }
        }
        true
    }

    let mut max_clique = Vec::new();
    let nodes: Vec<String> = graph.adj.keys().cloned().collect();

    for i in 0..nodes.len() {
        let mut current_clique = vec![nodes[i].clone()];
        for j in 0..nodes.len() {
            let mut candidate_clique = current_clique.clone();
            candidate_clique.push(nodes[j].clone());
            if i != j && is_clique(graph, &candidate_clique) {
                current_clique = candidate_clique
            }
        }
        if current_clique.len() > max_clique.len() {
            max_clique = current_clique;
        }
    }

    max_clique
}

pub fn solve() {
    let connections = parser::parse();
    let mut graph = Graph::new();

    for (u, v) in connections {
        graph.add_edge(u, v);
    }

    let mut largest_clique = find_largest_clique(&graph);
    largest_clique.sort();
    let password = largest_clique.join(",");
    println!("{password}")
}
