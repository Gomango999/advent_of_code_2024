use super::parser::{parse, Key, Lock};
use itertools::Itertools;

const HEIGHT: u64 = 5;
const WIDTH: usize = 5;

fn do_key_and_lock_match(key: &Key, lock: &Lock) -> bool {
    for i in 0..WIDTH {
        if key.heights[i] + lock.heights[i] > HEIGHT {
            return false;
        }
    }
    true
}

pub fn solve() {
    let (keys, locks) = parse();

    let all_combos = keys.iter().cartesian_product(locks.iter());
    let num_matches = all_combos
        .filter(|&(key, lock)| do_key_and_lock_match(key, lock))
        .count();
    println!("{}", num_matches)
}
