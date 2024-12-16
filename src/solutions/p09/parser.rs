use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
pub struct FileBlock {
    pub id: u64,
    pub pos: usize,
    pub size: usize,
}

pub struct Input {
    pub file_blocks: Vec<FileBlock>,
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

    let mut id = 0;
    let mut pos = 0;
    let file_blocks = raw_memory_sizes
        .iter()
        .enumerate()
        .filter_map(|(i, &n)| {
            let block = if i % 2 == 0 {
                let result = Some(FileBlock { id, pos, size: n });
                id += 1;
                result
            } else {
                None
            };
            pos += n;
            block
        })
        .collect();

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

    Input {
        file_blocks,
        memory,
    }
}
