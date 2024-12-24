//! This solution took many many tries:
//!
//!     1. Greedy solution, where we simply assume that any path will work
//!
//!     2. Optimised Greedy solution, where we notice that certain patterns
//!     are bad. E.g. `>` then `^` is always bad, and should instead be replaced
//!     with `^` then `>`
//!
//!     3. Dynamic Programming Solution, where notice that each move between two
//!     keys are independent of each other because of the need to reset all robot
//!     arm positions to 'A'.
//!
//! Only after a considerable amount of coding and trying the third approach did we
//! get the AC!

use super::parser::*;
use super::Vec2;
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

// Converts a given button to a vector coordinate. This works regardless of whether
// we give it a button from the numpad or a robot's input. The coordinates are
// always set such that 'A' is at (0,0).
fn char_to_location(ch: char) -> Vec2 {
    match ch {
        'A' => Vec2::new(0, 0),
        // numpad
        '0' => Vec2::new(-1, 0),
        '1' => Vec2::new(-2, -1),
        '2' => Vec2::new(-1, -1),
        '3' => Vec2::new(0, -1),
        '4' => Vec2::new(-2, -2),
        '5' => Vec2::new(-1, -2),
        '6' => Vec2::new(0, -2),
        '7' => Vec2::new(-2, -3),
        '8' => Vec2::new(-1, -3),
        '9' => Vec2::new(0, -3),
        // directional keypad
        '^' => Vec2::new(-1, 0),
        '<' => Vec2::new(-2, 1),
        'v' => Vec2::new(-1, 1),
        '>' => Vec2::new(0, 1),
        // error
        _ => panic!("Invalid character!"),
    }
}

/// Calculates the list of directions that need to be pressed to get from point
/// A to point B. These directions will be chosen in such a way that they
/// minimise the number of moves needed to create the key-presses for each move.
fn get_possible_paths(start: &Vec2, &end: &Vec2) -> Vec<Code> {
    // Calculate the moves that we need to make in each direction.
    type Move = ((char, i64), (char, i64));
    let mut moves: Move = (('?', 0), ('?', 0));

    if start.x < end.x {
        moves.0 = ('>', (start.x - end.x).abs());
    } else if start.x > end.x {
        moves.0 = ('<', (start.x - end.x).abs());
    }

    if start.y < end.y {
        moves.1 = ('v', (start.y - end.y).abs());
    } else if start.y > end.y {
        moves.1 = ('^', (start.y - end.y).abs());
    }

    // These are the two different ways we can move between two points.
    // E.g. If `moves1` represents `vv<`, then `moves2` represents `<vv`
    let moves1 = moves;
    let moves2 = (moves.1, moves.0);

    /// Checks if the moves are valid by checking whether it will exit the
    /// numpad space. Note this works for both the numpad and directional keypad.
    fn is_move_valid(moves: &Move, start: &Vec2, end: &Vec2) -> bool {
        if start.x == -2 && end.y == 0 && (moves.0 .0 == 'v' || moves.0 .0 == '^') {
            return false;
        } else if start.y == 0 && end.x == -2 && moves.0 .0 == '<' {
            return false;
        } else {
            return true;
        }
    }

    /// Given a set of moves, returns an option containing the expanded moves.
    /// E.g. if moves = (('^', 2), ('<', 1)), then the output will be ['^','^','<', 'A']
    fn expand_moves(moves: &Move) -> Code {
        let mut path = vec![];
        for (direction, count) in [moves.0, moves.1] {
            if direction != '?' {
                for _ in 0..count {
                    path.push(direction);
                }
            }
        }
        path.push('A');

        path
    }

    let paths = [moves1, moves2]
        .iter()
        .filter(|moves| is_move_valid(moves, &start, &end))
        .map(|moves| expand_moves(moves))
        .unique()
        .collect();
    paths
}

#[allow(dead_code)]
fn print_code(code: &Code) {
    for ch in code {
        print!("{ch}");
    }
}

static CACHE: OnceLock<Mutex<HashMap<(Vec2, Vec2, u64), u64>>> = OnceLock::new();

/// Gets the complexity of moving a robot arm from start to end.
fn get_complexity_pair(start: &Vec2, end: &Vec2, num_keypads: u64) -> u64 {
    let cache_key = (start.clone(), end.clone(), num_keypads);
    {
        let cache = CACHE
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap();
        if let Some(&cached_result) = cache.get(&cache_key) {
            return cached_result;
        }
    }

    let paths = get_possible_paths(start, end);
    let complexity = paths
        .into_iter()
        .map(|path| get_complexity(&path, num_keypads - 1))
        .min()
        .expect("There should be at least one path");

    {
        let mut cache = CACHE
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap();
        cache.insert(cache_key, complexity);
    }

    complexity
}

/// Gets the complexity of typing a given code, assuming that all robot arms start at A
fn get_complexity(code: &Code, num_keypads: u64) -> u64 {
    if num_keypads == 0 {
        // We can just type the code directly.
        return code.len() as u64;
    }

    // Break up longer codes into each individual segments.
    // We add an 'A' in front to simulate the starting position of the robot arms.
    let mut code = code.clone();
    code.insert(0, 'A');
    let sum = code
        .windows(2)
        .map(|window| {
            let &[start, end] = window else {
                panic!("Code should have exactly 2 elements");
            };
            let start = char_to_location(start);
            let end = char_to_location(end);
            get_complexity_pair(&start, &end, num_keypads)
        })
        .sum::<u64>();

    sum
}

fn get_code_numeric(code: &Code) -> u64 {
    let numeric_string: String = code.into_iter().take(code.len() - 1).collect();
    numeric_string.parse::<u64>().unwrap()
}

pub fn solve() {
    let codes = parse();

    // The number of _directional_ keypads on top of the original numeric keypad.
    const NUM_KEYPADS: u64 = 26;

    let complexity_sum: u64 = codes
        .into_iter()
        .map(|code| {
            let complexity = get_complexity(&code, NUM_KEYPADS);
            let code_numeric = get_code_numeric(&code);
            // println!("{complexity} {code_numeric}");
            complexity as u64 * code_numeric
        })
        .sum();

    println!("{complexity_sum}");
}
