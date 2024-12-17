use super::parser;
use super::parser::Input;
use super::Direction;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord)]
struct Node {
    score: u64,
    y: usize,
    x: usize,
    dir: Direction,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

/// Returns a 3D vector `min_score`, such that `min_score[y][x][dir]` tells ut
/// the minimum score required to get to position (y,x) while facing direction
/// `dir`.
fn calc_min_scores(input: &Input) -> Vec<Vec<Vec<u64>>> {
    let Input {
        height,
        width,
        start_x,
        start_y,
        grid,
        ..
    } = input.clone();
    let mut min_scores = vec![vec![vec![u64::MAX; Direction::NUM_VARIANTS]; width]; height];

    let mut pq = BinaryHeap::new();
    pq.push(Node {
        score: 0,
        y: start_y,
        x: start_x,
        dir: Direction::Right,
    });

    // Runs a dijkstra, prioritising processing nodes that have the smallest
    // score we've seen so far.
    while !pq.is_empty() {
        let curr = pq.pop().unwrap();
        if min_scores[curr.y][curr.x][curr.dir as usize] != u64::MAX {
            continue;
        }
        min_scores[curr.y][curr.x][curr.dir as usize] = curr.score;

        // Try to go forward
        let dir = curr.dir.to_offset();
        // Don't need to check whether we are outside the grid or not, since
        // the walls will prevent us from doing so.
        let ny = ((curr.y as i64) + dir.y) as usize;
        let nx = ((curr.x as i64) + dir.x) as usize;
        if grid[ny][nx] != '#' {
            pq.push(Node {
                score: curr.score + 1,
                y: ny,
                x: nx,
                dir: curr.dir,
            });
        }

        // Try to turn left and right
        pq.push(Node {
            score: curr.score + 1000,
            y: curr.y,
            x: curr.x,
            dir: curr.dir.turn_left(),
        });

        pq.push(Node {
            score: curr.score + 1000,
            y: curr.y,
            x: curr.x,
            dir: curr.dir.turn_right(),
        });
    }

    min_scores
}

/// Returns the number of tiles that are on any best path.
fn find_seats(
    grid: &Vec<Vec<char>>,
    min_score: &Vec<Vec<Vec<u64>>>,
    y: usize,
    x: usize,
    dir: Direction,
    seen: &mut Vec<Vec<Vec<bool>>>,
) {
    if seen[y][x][dir as usize] {
        return;
    }
    seen[y][x][dir as usize] = true;

    let reverse_dir = dir.flip().to_offset();
    // Don't need to check whether we are outside the grid or not, since
    // the walls will prevent us from doing so.
    let py = ((y as i64) + reverse_dir.y) as usize;
    let px = ((x as i64) + reverse_dir.x) as usize;
    if grid[py][px] != '#' && min_score[py][px][dir as usize] + 1 == min_score[y][x][dir as usize] {
        find_seats(grid, min_score, py, px, dir, seen);
    }

    if min_score[y][x][dir.turn_left() as usize] + 1000 == min_score[y][x][dir as usize] {
        find_seats(grid, min_score, y, x, dir.turn_left(), seen);
    }

    if min_score[y][x][dir.turn_right() as usize] + 1000 == min_score[y][x][dir as usize] {
        find_seats(grid, min_score, y, x, dir.turn_right(), seen);
    }
}

pub fn solve() {
    let input = parser::parse();

    let min_score = calc_min_scores(&input);

    let mut seen = vec![vec![vec![false; Direction::NUM_VARIANTS]; input.width]; input.height];

    for dir in Direction::all() {
        find_seats(
            &input.grid,
            &min_score,
            input.end_y,
            input.end_x,
            dir,
            &mut seen,
        )
    }

    let mut num_seats = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            // If we have seen this tile in our `find_seats` function in any
            // of the 4 directions, then this is a potential seat.
            if seen[y][x].iter().any(|&b| b) {
                num_seats += 1;
            }
        }
    }

    println!("{num_seats}");
}
