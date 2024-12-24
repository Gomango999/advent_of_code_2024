use super::parser::*;
use std::collections::{HashMap, VecDeque};

fn position_to_button_keypad(pos: (i64, i64)) -> char {
    match pos {
        (1, 0) => '^',
        (2, 0) => 'A',
        (0, 1) => '<',
        (1, 1) => 'v',
        (2, 1) => '>',
        _ => panic!("Invalid position! {:?}", pos),
    }
}

fn position_to_button_numpad(pos: (i64, i64)) -> char {
    match pos {
        (0, 0) => '7',
        (1, 0) => '8',
        (2, 0) => '9',
        (0, 1) => '4',
        (1, 1) => '5',
        (2, 1) => '6',
        (0, 2) => '1',
        (1, 2) => '2',
        (2, 2) => '3',
        (1, 3) => '0',
        (2, 3) => 'A',
        _ => panic!("Invalid position! {:?}", pos),
    }
}

fn get_code_complexity(code: &Code) -> u64 {
    // State stores a state where:
    // 1. have typed the first `upto` characters of `code`,
    // 2. have robot 1's arm positioned at `(x1, y1)`
    // 3. have robot 2's arm positioned at `(x2, y2)` and
    // 4. have robot 3's arm positioned at `(x3, y3)`
    #[derive(Hash, Clone, Copy, PartialEq, Eq)]
    struct State<'a> {
        // how many numbers in the original code we have typed.
        upto: usize,
        code: &'a Code,
        // robot 1's arm position
        x1: i64,
        y1: i64,
        // robot 2's arm position
        x2: i64,
        y2: i64,
        // robot 3's arm position
        x3: i64,
        y3: i64,
    }

    impl<'a> State<'a> {
        // `is_state_valid` checks that all the robot arms are pointing to valid
        // locations on the numpad/keypad.
        fn is_valid(&self) -> bool {
            self.upto <= self.code.len()
                && (0..3).contains(&self.x1)
                && (0..4).contains(&self.y1)
                && (self.x1, self.y1) != (0, 3)
                && (0..3).contains(&self.x2)
                && (0..2).contains(&self.y2)
                && (self.x2, self.y2) != (0, 0)
                && (0..3).contains(&self.x3)
                && (0..2).contains(&self.y3)
                && (self.x3, self.y3) != (0, 0)
        }

        /// Simulates the current button press on robot 3 on the current state.
        /// It will return None if a robot arm moves outside the containing box
        /// or if the first first robot tries to output a character that isn't
        /// part of the code.
        fn simulate(&self, input: char) -> Option<Self> {
            // If we have already typed the entire code, we don't need to simulate
            // any further.
            if self.upto == self.code.len() {
                return None;
            }

            let mut state = self.clone();
            match input {
                '^' => state.y3 -= 1,
                'v' => state.y3 += 1,
                '<' => state.x3 -= 1,
                '>' => state.x3 += 1,
                'A' => {
                    let input = position_to_button_keypad((state.x3, state.y3));

                    match input {
                        '^' => state.y2 -= 1,
                        'v' => state.y2 += 1,
                        '<' => state.x2 -= 1,
                        '>' => state.x2 += 1,
                        'A' => {
                            let input = position_to_button_keypad((state.x2, state.y2));
                            match input {
                                '^' => state.y1 -= 1,
                                'v' => state.y1 += 1,
                                '<' => state.x1 -= 1,
                                '>' => state.x1 += 1,
                                'A' => {
                                    let input = position_to_button_numpad((state.x1, state.y1));
                                    if input != state.code[state.upto] {
                                        return None;
                                    }
                                    state.upto += 1;
                                }
                                _ => panic!("Invalid command!"),
                            }
                        }
                        _ => panic!("Invalid command!"),
                    }
                }
                _ => panic!("Invalid command!"),
            }

            if !state.is_valid() {
                None
            } else {
                Some(state)
            }
        }
    }

    impl std::fmt::Display for State<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "upto:{} r1:({}, {}) r2:({}, {}) r3:({}, {})",
                self.upto, self.x1, self.y1, self.x2, self.y2, self.x3, self.y3
            )
        }
    }

    // min_inputs.get(state) is the minimum number of inputs the Historians have
    // to make to reach a certain state.
    let mut min_inputs: HashMap<State, u64> = HashMap::new();
    let mut prev_state: HashMap<State, Option<State>> = HashMap::new();

    let mut queue = VecDeque::new();
    // Robot arms are all on their respective 'A' buttons.
    let start = State {
        upto: 0,
        code,
        x1: 2,
        y1: 3,
        x2: 2,
        y2: 0,
        x3: 2,
        y3: 0,
    };
    queue.push_back((start, None, 0));

    while let Some((curr, prev, num_inputs)) = queue.pop_front() {
        if min_inputs.contains_key(&curr) {
            continue;
        }
        min_inputs.insert(curr, num_inputs);
        prev_state.insert(curr, prev);

        let possible_inputs = ['^', '<', '>', 'v', 'A'];
        for input in possible_inputs {
            if let Some(next) = curr.simulate(input) {
                queue.push_back((next, Some(curr), num_inputs + 1));
            }
        }
    }

    // We use the fact that we know that every code ends with 'A' to only check
    // the case in `min_inputs` where all robot arms are on the 'A' position.
    let end = State {
        upto: code.len(),
        code,
        x1: 2,
        y1: 3,
        x2: 2,
        y2: 0,
        x3: 2,
        y3: 0,
    };
    let &num_inputs = min_inputs.get(&end).expect("Couldn't type the code?!");

    num_inputs
}

fn get_code_numeric(code: &Code) -> u64 {
    let numeric_string: String = code.into_iter().take(code.len() - 1).collect();
    numeric_string.parse::<u64>().unwrap()
}

pub fn solve() {
    let codes = parse();

    let complexity_sum: u64 = codes
        .into_iter()
        .map(|code| {
            let complexity = get_code_complexity(&code);
            let numeric_part = get_code_numeric(&code);
            // println!("{complexity} {numeric_part}");
            complexity as u64 * numeric_part
        })
        .sum();
    println!("{complexity_sum}");
}
