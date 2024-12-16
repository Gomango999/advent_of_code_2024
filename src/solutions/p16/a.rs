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

pub fn solve() {
    let Input {
        height,
        width,
        start_x,
        start_y,
        end_x,
        end_y,
        grid,
    } = parser::parse();

    let mut min_score = vec![vec![vec![u64::MAX; Direction::NUM_VARIANTS]; width]; height];

    let mut pq = BinaryHeap::new();
    pq.push(Node {
        score: 0,
        y: start_y,
        x: start_x,
        dir: Direction::Right,
    });

    while !pq.is_empty() {
        let curr = pq.pop().unwrap();
        if min_score[curr.y][curr.x][curr.dir as usize] != u64::MAX {
            continue;
        }
        min_score[curr.y][curr.x][curr.dir as usize] = curr.score;

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

    let mut best_score = u64::MAX;
    for dir in Direction::all() {
        best_score = std::cmp::min(best_score, min_score[end_y][end_x][dir as usize]);
    }
    println!("{best_score}");
}
