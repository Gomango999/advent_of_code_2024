use super::parser::*;
use std::fs::File;
use std::io::Write;

const ROOM_SIZE: Vec2 = Vec2 { x: 101, y: 103 };

type Room = [[char; ROOM_SIZE.x as usize]; ROOM_SIZE.y as usize];
type Seen = [[bool; ROOM_SIZE.x as usize]; ROOM_SIZE.y as usize];

fn produce_grid(robots: &Vec<Robot>) -> Room {
    let mut room = [['.'; ROOM_SIZE.x as usize]; ROOM_SIZE.y as usize];
    for robot in robots {
        room[robot.pos.y as usize][robot.pos.x as usize] = '#'
    }
    room
}

fn display_room(file: &mut File, room: &Room) {
    for y in 0..ROOM_SIZE.y {
        for x in 0..ROOM_SIZE.x {
            let ch = room[y as usize][x as usize];
            let ch = &[ch as u8];
            let _ = file.write_all(ch);
        }
        let _ = file.write_all(b"\n");
    }
}

fn is_inside(y: i64, x: i64, seen: &Seen) -> bool {
    let n = seen.len() as i64;
    let m = seen[0].len() as i64;
    (0..n).contains(&y) && (0..m).contains(&x)
}

const DY: [i64; 8] = [1, 1, 1, 0, -1, -1, -1, 0];
const DX: [i64; 8] = [1, 0, -1, -1, -1, 0, 1, 1];
fn get_cluster_size(y: usize, x: usize, room: &Room, seen: &mut Seen) -> u64 {
    if seen[y][x] {
        return 0;
    }
    seen[y][x] = true;
    if room[y][x] != '#' {
        return 0;
    }

    let mut size = 1;
    for dir in 0..8 {
        let ny = y as i64 + DY[dir];
        let nx = x as i64 + DX[dir];
        if !is_inside(ny, nx, seen) {
            continue;
        }
        size += get_cluster_size(ny as usize, nx as usize, room, seen)
    }
    size
}

/// The size of any given cluster for `is_christmas_tree()` to decide that
/// the room has ASCII art of a christmas tree in it.
const CLUSTER_THRESHOLD: u64 = 50;

/// Determines whether a particular room has a picture of a christmas tree in it.
fn is_christmas_tree(room: &Room) -> bool {
    // Theory: When a christmas tree is formed, there will be a much higher number
    // robots that are adjacent to each other than normal (in order to form lines
    // that we can see). We will DFS for a large cluster of connected robots.

    let mut seen: Seen = [[false; ROOM_SIZE.x as usize]; ROOM_SIZE.y as usize];
    for y in 0..ROOM_SIZE.y {
        for x in 0..ROOM_SIZE.x {
            let size = get_cluster_size(y as usize, x as usize, &room, &mut seen);
            if size > CLUSTER_THRESHOLD {
                return true;
            }
        }
    }
    false
}

pub fn solve() {
    let mut robots = parse();

    let mut file = File::create("./src/p14/out.txt").unwrap();

    for t in 1..10000 {
        robots = robots
            .into_iter()
            .map(|robot| {
                let new_pos = (robot.pos + robot.vel) % ROOM_SIZE;
                Robot {
                    pos: new_pos,
                    ..robot
                }
            })
            .collect();

        let room = produce_grid(&robots);
        if is_christmas_tree(&room) {
            let contents = format!("{t}:\n");
            let _ = file.write_all(contents.as_bytes());
            display_room(&mut file, &room)
        }
    }
}
