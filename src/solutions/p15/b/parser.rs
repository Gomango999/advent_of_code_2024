use super::super::Direction;
use super::room::{Object, Room};
use std::fs::File;
use std::io::{self, BufRead};

enum InputState {
    Grid,
    Instructions,
}

pub fn parse() -> (Room, Vec<Direction>) {
    let file = File::open("src/solutions/p15/in.txt").unwrap();
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
                let mut row = vec![];
                for ch in line.chars() {
                    match ch {
                        '#' => row.extend([Object::Wall, Object::Wall]),
                        '.' => row.extend([Object::Empty, Object::Empty]),
                        '@' => row.extend([Object::Robot, Object::Empty]),
                        'O' => row.extend([Object::BoxLeft, Object::BoxRight]),
                        _ => panic!("Unknown character!"),
                    }
                }
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
                        _ => panic!("Unknown instruction!"),
                    })
                    .collect();
                instructions.extend(row);
            }
        }
    }

    let room = Room::new(grid);
    (room, instructions)
}
