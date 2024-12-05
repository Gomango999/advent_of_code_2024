use super::parser;
use std::cmp::Ordering;

// For each manual, for each pair of pages, we check against the entire rule book
// ~200 manuals, max ~23^2/2 pairs per manual, ~1000 rules = ~50mil operations
// This is computable in less than a second, so safe to run this function on
// every manual once.
fn is_correctly_ordered(manual: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for i in 0..manual.len() {
        for j in i + 1..manual.len() {
            for rule in rules.iter() {
                let has_broken_rule = manual[i] == rule.1 && manual[j] == rule.0;
                if has_broken_rule {
                    return false;
                }
            }
        }
    }
    return true;
}

// mat[i][j] is true if there is a rule that says page i must come before page j
fn make_rule_matrix(rules: &Vec<(i32, i32)>) -> [[bool; 101]; 101] {
    let mut adj = [[false; 101]; 101];
    for &(lhs, rhs) in rules {
        adj[lhs as usize][rhs as usize] = true;
    }
    adj
}

pub fn solve() {
    let parser::Input { rules, manuals } = parser::parse();

    let rule_matrix = make_rule_matrix(&rules);

    let mut total = 0;
    for mut manual in manuals {
        if is_correctly_ordered(&manual, &rules) {
            continue;
        };

        // I ran some code to check that whether every pair of pages in every
        // manual corresponding rule and the answer is yes. Thus we can use
        // this to sort the pages.
        manual.sort_by(|&a, &b| {
            if a == b {
                Ordering::Equal
            } else if rule_matrix[a as usize][b as usize] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let middle = manual[manual.len() / 2];
        total += middle;
    }
    println!("{total}");
}
