use super::parser;

pub fn solve() {
    let parser::Input { mut memory, .. } = parser::parse();

    // We use two pointers. l will keep track of the current next available
    // space of memory. r will keep track of the next file pointer we'd like
    // to move into l.

    let mut checksum: u64 = 0;
    let mut l = 0;
    let mut r = memory.len() - 1;
    'calc_checksum: while l < r {
        while let Some(file_id) = memory[l] {
            checksum += file_id * (l as u64);
            l += 1;
        }
        while memory[r].is_none() {
            r -= 1;
        }
        // l is now on the first free space in memory, and
        // r is on the last used space in memory.
        if l >= r {
            break 'calc_checksum;
        }
        (memory[l], memory[r]) = (memory[r], memory[l]);
    }
    println!("{checksum}");
}
