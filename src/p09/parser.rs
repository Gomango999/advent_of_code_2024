use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Input {
    pub memory_sizes: MemorySizes,
    pub memory: Vec<Option<u64>>,
}

/// Returns two vectors:
/// 1. Alternating numbers reprseenting the free and used space on disk
/// 2. A vector of options representing each block of memory on disk in order.
pub fn parse() -> Input {
    let file = File::open("src/p09/in.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    let raw_memory_sizes: Vec<usize> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|x| x as usize))
        .collect();
    let memory_sizes = MemorySizes::new(raw_memory_sizes.clone());
    let memory: Vec<Option<u64>> = raw_memory_sizes
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            if i % 2 == 0 {
                vec![Some((i / 2) as u64); n]
            } else {
                vec![None; n]
            }
        })
        .flatten()
        .collect();
    // Assuming an average size of 5 per memory block, we get that memory_sizes
    // will have length about 20,000 * 5 = 100,000.
    Input {
        memory_sizes,
        memory,
    }
}
