use super::parser;

/// Calculates if (x,y) is an antinode based on the coordinates of two antennae at (a1x, a1y) and (a2x, a2y)
fn check_is_antinode((a1x, a1y): (i32, i32), (a2x, a2y): (i32, i32), (x, y): (i32, i32)) -> bool {
    let ax = a2x - a1x;
    let ay = a2y - a1y;
    let x = x - a1x;
    let y = y - a1y;
    ax * y == ay * x
}

pub fn solve() {
    let parser::Input {
        height,
        width,
        antennae: all_antennae,
    } = parser::parse();

    // 50 x 50 grid locations. Each one considers up to 26+26+10 = 62 character.
    // Suppose there are about 5 antennae per character, then this means on
    // average 12 comparisons per character. This gives 50 * 50 * 62 * 12 ~=
    // 1,860,000, which is feasible.

    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            let mut is_antinode = false;
            'check_antinode: for ch in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
                let Some(antennae) = all_antennae.get(&ch) else {
                    continue;
                };
                for i in 0..antennae.len() {
                    for j in i + 1..antennae.len() {
                        if check_is_antinode(antennae[i], antennae[j], (x, y)) {
                            is_antinode = true;
                            break 'check_antinode;
                        }
                    }
                }
            }
            if is_antinode {
                total += 1
            }
        }
    }
    println!("{total}")
}
