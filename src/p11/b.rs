use super::parser;
use std::collections::HashMap;

fn has_even_digits(num: u64) -> bool {
    let mut count = 0;
    let mut num = num;
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count % 2 == 0
}

/// splits a number into the left and right halves of the digits
/// E.g. 2456 -> (24, 56)
fn split_number(num: u64) -> (u64, u64) {
    let num_str = num.to_string();
    let len = num_str.len();
    let (left_str, right_str) = num_str.split_at(len / 2);
    let left = left_str.parse::<u64>().expect("Invalid left half");
    let right = right_str.parse::<u64>().expect("Invalid right half");
    (left, right)
}

/// returns the number of rocks that `num` will split into after `blinks` blinks
fn simulate_blinks(num: u64, blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(num_rocks) = cache.get(&(num, blinks)) {
        return *num_rocks;
    }

    let num_rocks = if num == 0 {
        simulate_blinks(1, blinks - 1, cache)
    } else if has_even_digits(num) {
        let (left, right) = split_number(num);
        simulate_blinks(left, blinks - 1, cache) + simulate_blinks(right, blinks - 1, cache)
    } else {
        simulate_blinks(num * 2024, blinks - 1, cache)
    };
    cache.insert((num, blinks), num_rocks);
    num_rocks
}

pub fn solve() {
    let nums = parser::parse();
    const NUM_BLINKS: u64 = 75;

    let mut total = 0;
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    for num in nums {
        total += simulate_blinks(num, NUM_BLINKS, &mut cache)
    }
    println!("{total}")
}
