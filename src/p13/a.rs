use super::parser::*;
use std::cmp;

pub fn solve() {
    let configs = parse();

    let mut total = 0;
    for config in configs {
        let mut best = i64::MAX;
        for a in 0..=100 {
            for b in 0..=100 {
                let Config {
                    a_button,
                    b_button,
                    prize,
                } = config;
                let pos = a_button * a + b_button * b;
                if pos == prize {
                    best = cmp::min(best, 3 * a + b);
                }
            }
        }
        if best != i64::MAX {
            total += best;
        }
    }
    println!("{total}")
}
