use super::parser;

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

pub fn solve() {
    let parser::Input { rules, manuals } = parser::parse();

    let mut total = 0;
    for manual in manuals {
        if is_correctly_ordered(&manual, &rules) {
            let middle = manual[manual.len() / 2];
            total += middle;
        }
    }
    println!("{total}");
}
