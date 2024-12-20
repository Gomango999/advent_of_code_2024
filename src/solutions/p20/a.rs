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

pub fn solve() {
    let maze = parse();

    let shortest_dist_to_start = get_shortest_dists(&maze, maze.start);
    let shortest_dist_to_end = get_shortest_dists(&maze, maze.end);
    let original_time = shortest_dist_to_start[maze.end.1][maze.end.0];

    // Calculate a vector containing all possible time saves from using cheats
    let mut times_saved = vec![];
    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.grid[y][x] == '#' {
                continue;
            }
            if shortest_dist_to_start[y][x] == u64::MAX {
                continue;
            }

            // Calculate all possible offsets after a cheat starting at (x,y)
            let double_move = DIRECTIONS.map(|(x, y)| (x * 2, y * 2));
            let diagonal_move: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
            let offsets = double_move.into_iter().chain(diagonal_move.into_iter());

            // Calculate time saves in all directions
            for (dx, dy) in offsets {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if !maze.is_inside((nx, ny)) {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);
                if maze.grid[ny][nx] == '#' {
                    continue;
                }
                if shortest_dist_to_end[y][x] == u64::MAX {
                    continue;
                }

                let cheated_time = shortest_dist_to_start[y][x] + 2 + shortest_dist_to_end[ny][nx];
                if cheated_time < original_time {
                    let time_save = (original_time - cheated_time) as u64;
                    times_saved.push(time_save);
                }
            }
        }
    }

    // use counter::Counter;
    // let counts: Counter<_> = times_saved.iter().cloned().collect();
    // println!("{counts:?}");

    let num_cheats = times_saved.into_iter().filter(|&x| x >= 100).count();
    println!("{num_cheats}");
}
