//# Attempt 1: Hill Climbing
//# - Result: Incorrect
//# The idea is try to get a better and better machine by finding one of the
//# correct swaps one at a time. The hope is that each time we make a correct
//# swap, the machine will get better starting from the least significant bit.
//#
//# Unfortunately, this didn't work. The machine is unable to improve even
//# after all possible swaps have been tested. This can either be caused by
//# bugged implementation (wouldn't put it past myself), the fact that multiple
//# swaps are needed simultaneously to improve the machine, or weak test_data.

use super::machine::Machine;
use super::parser::*;
use rand::Rng;

/// Generate a list of random pairs of numbers to test the machine with.
fn generate_test_pairs() -> Vec<(u64, u64)> {
    const NUM_TESTS: usize = 200;
    let mut rng = rand::thread_rng();
    let mut test_pairs = vec![];
    for _ in 0..NUM_TESTS {
        let a = rng.gen_range(0..(1 << 45));
        let b = rng.gen_range(0..(1 << 45));
        test_pairs.push((a, b));
    }
    test_pairs
}

/// Takes two numbers and checks how many consecutive bits starting from the
/// least significant bit are the same.
fn similarity(a: u64, b: u64) -> u64 {
    let mut count = 0;
    let mut mask = 1;
    while mask < (1 << 45) {
        if (a & mask) == (b & mask) {
            count += 1;
        } else {
            break;
        }
        mask <<= 1;
    }
    count
}

/// Takes a list of pairs of numbers and uses those as the input of the machine.
/// It then runs the machine and returns the average similarity score.
fn test_machine(machine: &mut Machine, test_pairs: &Vec<(u64, u64)>) -> f64 {
    let mut total_score = 0;
    for &(a, b) in test_pairs {
        machine.reset(a, b);
        let output = machine.run();
        let score = similarity(output, a + b);
        total_score += score;
    }
    total_score as f64 / test_pairs.len() as f64
}

pub fn solve() {
    let mut machine = parse();

    let test_pairs = generate_test_pairs();

    let initial_score = test_machine(&mut machine, &test_pairs);

    let mut best_score = (initial_score, (0, 0));
    println!("initial:{}", initial_score);

    const NUM_SWAPS: usize = 4;
    for _ in 0..NUM_SWAPS {
        // We try every possible swap of outputs of two gates. We find the one
        // that improves the current score of the machine the most and make the
        // swap.

        for i in 0..machine.gates.len() {
            for j in i + 1..machine.gates.len() {
                machine.swap_outputs(i, j);

                let score = test_machine(&mut machine, &test_pairs);
                let curr_score = (score, (i, j));

                if curr_score > best_score {
                    best_score = curr_score;
                }

                // Undo the swap so the is back to its original state.
                machine.swap_outputs(i, j);
            }
        }

        println!(
            "best:{} swap:{},{}",
            best_score.0, best_score.1 .0, best_score.1 .1
        );

        machine.swap_outputs(best_score.1 .0, best_score.1 .1);
    }
}
