//! My first attempt for this problem was a greedy solution. I conclude that
//! when moving two keys on a keypad, the order in which you do it doesn't matter
//! provided that you take the shortest manhattan distance path, and you group
//! the same directions together. For example, to go from 7 to 2 on the numpad,
//! you can do either DDR or RDD. DRD is always suboptimal since on the robot
//! that has to enter these keys, you have to waste unecessary time moving from
//! D to R to D again when you can ust handle al the Ds together.
//!
//! However, after implementing this and submitting, it didn't work. I ended
//! up coding a BFS style solution for A, which literally looks through
//! every possible state of robot arm positions across all 3 robots to find
//! the shortest number of moves required to get the output.
//!
//! Using my solution for part A, I was able to compare the outputs and see why
//! my solution seemed to be giving longer answers. Here is one such example
//! where my BFS outperformed.
//!
//! BFS Solution
//! <vA<AA>^>AvAA^<A>A<v<A>^>AAAvA^A<v<A>A^>AAvA^A<A>A<v<A>A^>A<A>vA^A (66)
//!   v <<   A >>  ^ A   <   AAA > A   < v  AA > A ^ A   < v  A ^  > A
//!          <       A       ^^^   A        vv   >   A        v      A
//!                  0             8                 3               A
//! Greedy Solution
//! v<A<AA>>^AvAA<^A>Av<<A>>^AAAvA^Av<A>^Av<<A>>^AAvA<^A>Av<A<A>>^AvA<^A>A (70)
//!   v <<   A >>  ^ A   <   AAA > A  v  A   <   AA >  ^ A  v <   A >  ^ A
//!          <       A       ^^^   A     >       vv      A        v      A
//!                  0             8                     3               A
//!
//! Looking at these two, I can see that the BFS solution was able to save 4
//! moves by grouping together a "v" press while moving back to the "<", whereas
//! the greedy solution separated the move to and back from the "<" into it's
//! own "group". That is, the BFS had "<vAA>A^A" to input "vv>A", whereas the
//! greedy did "vA<AA>^A" to output ">vvA". The problem is that the "A<A" sequence
//! requires moving all the way to the left button and back with nothing else,
//! wherease the BFS solution decides to do "(A)<vA" instead, which picks up
//! an extra "v" press for free on the way back.
//!
//! The lesson is that the pattern v> is always preferable to >v. And in fact
//! any pattern that requires a left move by itself is always worse. Therefore:
//! >v is worse than v>
//! v< is worse than <v
//! ^> is worse than >^
//! Also now that we know order does matter, the simplification I made about
//! allowing the robot arm to exit the grid now no longer applies.
//!
//! After implementing the optimisation, I'm realising that actually computing
//! the code that needs to be pressed is growing quite long. It seems that
//! the code roughly doubles for every layer of indirection, so we'll end up
//! with a string that is 2^25 ~= 33,554,432 bytes long. However in practice,
//! at iteration 21, we use 2.5 billion bytes, and we run out of memory.
//!
//! We'll need to optimise this to only keep track of the number of _pairs_ of
//! each direction.

use super::parser::*;
use super::Vec2;

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
fn get_path(start: &Vec2, &end: &Vec2) -> Code {
    // Calculate that the (unoptimised) set of moves that we need to make.
    // Moves stores two tuples: (direction, number of moves)
    let mut moves = (('?', 0), ('?', 0));

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

    // Next, we optimise them to remove bad patterns that will cause extra
    // key presses. These are resolved by swapping the two directions.
    match (moves.0 .0, moves.1 .0) {
        ('>', 'v') => {
            moves = (moves.1, moves.0);
        }
        ('v', '<') => {
            moves = (moves.1, moves.0);
        }
        ('^', '>') => {
            moves = (moves.1, moves.0);
        }
        _ => {}
    }

    // Finally, we confirm that this pattern does not let us go to (-2, 0), which
    // is illegal on both the numpad and keypad. This can always be resolved by
    // simply swapping the two directions.
    if start.x == -2 && end.y == 0 && moves.0 .0 == 'v' {
        moves = (moves.1, moves.0);
    } else if start.y == 0 && end.x == -2 && moves.0 .0 == '<' {
        moves = (moves.1, moves.0);
    }

    // Finally, we convert out moves into a list of characters.
    let mut path = vec![];

    for (direction, count) in [moves.0, moves.1] {
        for _ in 0..count {
            path.push(direction);
        }
    }

    path.push('A');

    path
}

fn get_inputs(code: &Code) -> Code {
    // We add an 'A' to the front, to model the idea that the robot arm starts
    // off at A, and needs to travel to the first digit.
    let mut code = code.clone();
    code.insert(0, 'A');
    let mut input = vec![];
    for window in code.windows(2) {
        let &[start_ch, end_ch] = window else {
            panic!()
        };
        let start = char_to_location(start_ch);
        let end = char_to_location(end_ch);
        let path = get_path(&start, &end);
        input.extend(path)
    }
    input
}

#[allow(dead_code)]
fn print_code(code: &Code) {
    let code_str: String = code.iter().cloned().collect();
    println!("{}", code_str);
}

fn get_code_input(code: &Code, num_robots: usize) -> Code {
    let mut code = code.clone();
    print_code(&code);
    for i in 0..num_robots {
        code = get_inputs(&code);
        println!("{i} {}", code.len());
        // print_code(&code);
    }
    code
}

fn get_code_numeric(code: &Code) -> u64 {
    let numeric_string: String = code.into_iter().take(code.len() - 1).collect();
    numeric_string.parse::<u64>().unwrap()
}

pub fn solve() {
    let codes = parse();

    const NUM_ROBOTS: usize = 25;

    let complexity_sum: u64 = codes
        .into_iter()
        .map(|code| {
            let code_input = get_code_input(&code, NUM_ROBOTS);
            let complexity = code_input.len();
            let code_numeric = get_code_numeric(&code);
            println!("{complexity} {code_numeric}");
            complexity as u64 * code_numeric
        })
        .sum();
    println!("{complexity_sum}");
}
