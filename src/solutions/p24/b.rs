use super::{machine::Machine, parser};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rand::Rng;

/// Generate a list of random pairs of numbers to test the adder with.
fn generate_test_pairs() -> Vec<(u64, u64)> {
    const NUM_TESTS: usize = 20;
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
    while mask < (1 << Machine::NUM_BITS) {
        if (a & mask) == (b & mask) {
            count += 1;
        } else {
            break;
        }
        mask <<= 1;
    }
    count
}

/// Takes a list of pairs of numbers and uses those as the input of the adder.
/// It then runs the adder and returns the average similarity score.
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

/// Finds all swaps. Machine is modified as well to account for the swaps.
pub fn find_all_swaps(machine: &mut Machine) -> Vec<(usize, usize)> {
    // Binary adders typically work from least significant bits to most significant
    // bits. If our binary adder is only minorly flawed because of swapped output
    // gates, it is likely that it will get the first x bits correct, and then
    // be wrong for almost everything else afterwards.
    //
    // Thus, we say that the 'score' of a machine is how many bits it gets correct
    // starting from the least significant bit. We will try every swap, and then
    // keep the swap that generates the best score. We repeat this 4 times to
    // get the final improved machine.
    //
    // This relies on the assumption that there is always one swap that we can
    // make that will impprove the score of the machine, which is not necessarily
    // true (imagine that the two gates responsible for the calculation of a
    // single bit are both corrupted, we'd need them both to be swapped for an
    // improvement in score!). However, I'm hoping that the odds of this are low.

    let test_pairs = generate_test_pairs();

    let initial_score = test_machine(machine, &test_pairs);

    let mut best_score = (initial_score, (0, 0));
    println!("initial:{}", initial_score);

    const NUM_SWAPS: usize = 4;

    let mut all_swaps = vec![];
    for _ in 0..NUM_SWAPS {
        let num_pairs = ((machine.gates.len() * machine.gates.len() - 1) / 2) as u64;
        let pb = ProgressBar::new(num_pairs);

        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} ({eta})",
            )
            .unwrap()
            .progress_chars("##-"),
        );

        // We try every possible swap of outputs of two gates. We find the one
        // that improves the current score of the adder the most and make the
        // swap.

        for i in 0..machine.gates.len() {
            for j in i + 1..machine.gates.len() {
                machine.swap_outputs(i, j);

                let score = test_machine(machine, &test_pairs);
                let curr_score = (score, (i, j));

                if curr_score > best_score {
                    best_score = curr_score;
                }

                // Undo the swap so the is back to its original state.
                machine.swap_outputs(i, j);

                pb.inc(1);
            }
        }

        pb.finish_with_message("found!");

        println!(
            "best:{} swap:{},{}",
            best_score.0, best_score.1 .0, best_score.1 .1
        );

        all_swaps.push(best_score.1);
    }

    all_swaps
}

pub fn solve() {
    let mut machine = parser::parse();

    let swaps = find_all_swaps(&mut machine);
    // swaps = vec![(19, 187), (165, 206), (44, 50), (133, 177)]

    let swaps = swaps
        .into_iter()
        .flat_map(|(x, y)| vec![x, y])
        .collect::<Vec<_>>();
    let swaps = swaps
        .into_iter()
        .map(|i| machine.gates[i].output.name.clone())
        .collect::<Vec<_>>();
    let answer = swaps.into_iter().sorted().join(",");
    println!("{}", answer)
}
