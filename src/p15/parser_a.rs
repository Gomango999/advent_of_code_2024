use super::direction::Direction;
use super::room_a::Room;
use std::fs::File;
use std::io::{self, BufRead};

enum InputState {
    Grid,
    Instructions,
}

pub fn parse() -> (Room, Vec<Direction>) {
    let file = File::open("src/p15/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut state = InputState::Grid;

    let mut grid = vec![];
    let mut instructions = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        match state {
            InputState::Grid => {
                if line.trim().is_empty() {
                    state = InputState::Instructions;
                    continue;
                }
                let row: Vec<char> = line.chars().collect();
                grid.push(row)
            }
            InputState::Instructions => {
                let row: Vec<Direction> = line
                    .chars()
                    .map(|ch| match ch {
                        '^' => Direction::Up,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        'v' => Direction::Down,
                        _ => panic!("Impossible!"),
                    })
                    .collect();
                instructions.extend(row);
            }
        }
    }

    let room = Room::new(grid);
    (room, instructions)
}
