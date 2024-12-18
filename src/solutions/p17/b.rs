#![allow(dead_code)]

use super::machine::Machine;
use super::parser;
use indicatif::ProgressBar;
use std::time::Instant;

/// `brute_force` tries every value of register_a until we find something that
/// produces the final output.
///
/// It takes 11 seconds to calculate 10,000,000 possibilities
/// This approach was unable to find the correct solution within 1,000,000,000
/// attempts (which took about ~20m to run)
fn brute_force(mut machine: Machine) {
    const MAX_REGISTER_A: u64 = 10_000_000;

    let start = Instant::now();
    let bar = ProgressBar::new(MAX_REGISTER_A);

    let mut result = None;
    for register_a in 0..MAX_REGISTER_A {
        machine.reset(register_a as u64, 0, 0);
        machine.run(false);
        if machine.get_output() == machine.get_program() {
            result = Some(register_a);
            break;
        }
        bar.inc(1);
    }
    bar.finish_with_message("Done!");

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    if let Some(register_a) = result {
        println!("{register_a}");
    } else {
        println!("Not found!");
    }
}

fn u64_to_bool(value: u64) -> bool {
    match value {
        0 => false,
        1 => true,
        _ => panic!(
            "Invalid value: {}. Only 0 or 1 can be converted to bool.",
            value
        ),
    }
}

fn bool_to_u64(value: bool) -> u64 {
    match value {
        false => 0,
        true => 1,
    }
}

/// Returns a vector starting with the least significant bit.
fn num_to_bits(mut num: u64) -> Vec<bool> {
    let mut bits = vec![];
    while num > 0 {
        bits.push(num % 2 == 1);
        num /= 2;
    }
    bits
}

/// Turns a vector with most significant bit first into a number
fn bits_to_num(bits: Vec<bool>) -> u64 {
    bits.iter().fold(0, |acc, &b| (acc << 1) + (b as u64))
}

fn check_is_quine(machine: &mut Machine, register_a: u64) {
    machine.reset(register_a, 0, 0);
    machine.run(false);
    assert_eq!(machine.get_output(), machine.get_program());
}

/// Computes the smallest value for register A, assuming that it's most significant bits
/// are already given by `bits`, and we have output everything in the program
/// after `program_index`. The output is stored in the `bits` vector.
fn rec_compute_register_a(bits: &mut Vec<bool>, program: &Vec<u8>, program_index: usize) -> bool {
    // This is what the code does line by line
    // 0. B = A % 8
    // 1. B = B ^ 1
    // 2. C = A >> B
    // 3. B = B ^ 5
    // 4. A = A >> 3
    // 5. B = B ^ C
    // 6. output (B % 8)
    // 7. Jump to 0
    //
    // The code processes chunks of register A 3 bits at a time. We store each
    // chunk into register B, and then some processing is done on that in order
    // to generate a single output before looping again. This function recursively
    // tries every value of B from smallest to largest until we get an output
    // that matches the program, and then moves on to the next chunk of 3 until
    // we have found all the bits that make up register A.

    // Recursively try values of B that produces the correct output, from smallest
    // to largest.
    for b in 0..8 {
        // We add on the bits of B to our existing bits, and see this will
        // produce the output value that we desire. If we do, we will continue
        // recursing. If we don't get the right output or recursion fails,
        // we remove the bits of B we added on and try again.

        let b_bit0 = b & 1 > 0;
        let b_bit1 = b & 2 > 0;
        let b_bit2 = b & 4 > 0;
        bits.extend([b_bit2, b_bit1, b_bit0]);

        let last_i = bits.len() - 1;
        let offset = (b ^ 1) as usize;
        let c_bit0 = bool_to_u64(bits[last_i - offset - 0]);
        let c_bit1 = bool_to_u64(bits[last_i - offset - 1]);
        let c_bit2 = bool_to_u64(bits[last_i - offset - 2]);
        let c = c_bit0 + c_bit1 * 2 + c_bit2 * 4;

        let target_output = program[program_index] as u64;
        let curr_output = (b ^ 1 ^ 5) ^ c;
        if curr_output == target_output {
            if program_index == 0 {
                return true;
            }
            if rec_compute_register_a(bits, program, program_index - 1) {
                return true;
            }
        }
        // This value of B does not work, remove it off the bits vector for the
        // next iteration of the loop.
        bits.pop();
        bits.pop();
        bits.pop();
    }

    return false;
}

fn compute_register_a(program: &Vec<u8>) -> Option<u64> {
    let mut bits: Vec<bool> = vec![false; 16];
    if rec_compute_register_a(&mut bits, program, program.len() - 1) {
        let register_a = bits_to_num(bits);
        return Some(register_a);
    } else {
        return None;
    }
}

pub fn reverse_engineer(mut machine: Machine) {
    if let Some(register_a) = compute_register_a(machine.get_program()) {
        check_is_quine(&mut machine, register_a);
        println!("{register_a}");
    } else {
        println!("Not found!");
    }
}

pub fn solve() {
    let machine = parser::parse();

    // First attempt - Didn't work :(
    // brute_force(machine);

    // Second attempt - Works!
    reverse_engineer(machine);
}
