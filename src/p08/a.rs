use super::parser;
use std::collections::HashSet;

fn calc_antinodes((a1_x, a1_y): (i32, i32), (a2_x, a2_y): (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let (diff_x, diff_y) = (a1_x - a2_x, a1_y - a2_y);
    let antinode1 = (a1_x + diff_x, a1_y + diff_y);
    let antinode2 = (a2_x - diff_x, a2_y - diff_y);
    (antinode1, antinode2)
}

fn is_inside(height: i32, width: i32, (x, y): (i32, i32)) -> bool {
    (0..height).contains(&y) && (0..width).contains(&x)
}

pub fn solve() {
    let parser::Input {
        height,
        width,
        antennae: all_antennae,
    } = parser::parse();

    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    for ch in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
        // antennae stores the locations of all antennae corresponding to ch.
        let Some(antennae) = all_antennae.get(&ch) else {
            continue;
        };
        for i in 0..antennae.len() {
            for j in i + 1..antennae.len() {
                let (antinode1, antinode2) = calc_antinodes(antennae[i], antennae[j]);
                if is_inside(height, width, antinode1) {
                    antinode_locations.insert(antinode1);
                }
                if is_inside(height, width, antinode2) {
                    antinode_locations.insert(antinode2);
                }
            }
        }
    }

    println!("{}", antinode_locations.len());
}
