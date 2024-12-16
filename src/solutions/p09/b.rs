use super::parser::*;

/// Returns the start position of the freeblock if there is one.
fn find_free_block(memory: &Vec<Option<u64>>, min_size: usize) -> Option<usize> {
    for i in 0..memory.len() {
        if memory[i].is_some() {
            continue;
        }
        let mut size = 0;
        while i + size < memory.len() && memory[i + size].is_none() {
            size += 1
        }
        if size >= min_size {
            return Some(i);
        }
    }
    None
}

fn get_checksum(memory: &Vec<Option<u64>>) -> u64 {
    let mut checksum = 0;
    for (i, slot) in memory.iter().enumerate() {
        if let Some(id) = slot {
            checksum += (i as u64) * id
        }
    }
    checksum
}

pub fn solve() {
    let Input {
        file_blocks,
        mut memory,
    } = parse();

    for file_block in file_blocks.iter().rev() {
        if let Some(free_block_pos) = find_free_block(&memory, file_block.size) {
            if free_block_pos > file_block.pos {
                continue;
            }
            for i in 0..file_block.size {
                memory[free_block_pos + i] = Some(file_block.id);
                memory[file_block.pos + i] = None;
            }
        }
    }
    let checksum = get_checksum(&memory);
    println!("{checksum}")
}
