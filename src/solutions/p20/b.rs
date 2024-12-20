use super::parser::*;
use std::collections::VecDeque;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_shortest_dists(maze: &Maze, start: (usize, usize)) -> Vec<Vec<u64>> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut shortest_dists = vec![vec![u64::MAX; maze.width]; maze.height];

    while let Some(((x, y), distance)) = queue.pop_front() {
        if shortest_dists[y][x] != u64::MAX {
            // We've seen this node already
            continue;
        }
        shortest_dists[y][x] = distance;

        for (dx, dy) in &DIRECTIONS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !maze.is_inside((nx, ny)) {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);

            if maze.grid[ny][nx] != '#' {
                queue.push_back(((nx, ny), distance + 1))
            }
        }
    }

    shortest_dists
}

fn manhattan_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> u64 {
    let (x1, y1) = (x1 as isize, y1 as isize);
    let (x2, y2) = (x2 as isize, y2 as isize);
    ((x1 - x2).abs() + (y1 - y2).abs()) as u64
}

pub fn solve() {
    let maze = parse();

    let shortest_dist_to_start = get_shortest_dists(&maze, maze.start);
    let shortest_dist_to_end = get_shortest_dists(&maze, maze.end);
    let original_time = shortest_dist_to_start[maze.end.1][maze.end.0];

    // Calculate a vector containing all possible time saves from using cheats
    const MAX_CHEAT_DIST: u64 = 20;
    let mut times_saved = vec![];
    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.grid[y][x] == '#' {
                continue;
            }
            if shortest_dist_to_start[y][x] == u64::MAX {
                continue;
            }

            // Loop through all possible endpoints of a cheat starting here.
            for ny in 0..maze.height {
                for nx in 0..maze.width {
                    let cheat_dist = manhattan_distance((x, y), (nx, ny));
                    if cheat_dist > MAX_CHEAT_DIST {
                        continue;
                    }
                    if shortest_dist_to_end[ny][nx] == u64::MAX {
                        continue;
                    }

                    let cheated_time =
                        shortest_dist_to_start[y][x] + cheat_dist + shortest_dist_to_end[ny][nx];
                    if cheated_time < original_time {
                        let time_save = (original_time - cheated_time) as u64;
                        times_saved.push(time_save);
                    }
                }
            }
        }
    }

    let num_cheats = times_saved.into_iter().filter(|&x| x >= 100).count();
    println!("{num_cheats}");
}
