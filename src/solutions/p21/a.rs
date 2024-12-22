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
        'v' => Vec2::new(-1, 1),
        '<' => Vec2::new(-2, 1),
        '>' => Vec2::new(0, 1),
        // error
        _ => panic!("Invalid character!"),
    }
}

/// Calculates the list of directions that need to be pressed to get from point
/// A to point B. Importantly, this function always prioritises grouping same
/// directional movements together. E.g. You will never get `<^<`, as this
/// function will group the two left moves together.
fn get_path(start: &Vec2, &end: &Vec2) -> Code {
    let mut path = vec![];

    let mut start = start.clone();
    let force_vertical_first = start.y == 0;
    while start != end {
        if !force_vertical_first && start.x < end.x {
            path.push('>');
            start.x += 1;
        } else if !force_vertical_first && start.x > end.x {
            path.push('<');
            start.x -= 1;
        } else if start.y < end.y {
            path.push('v');
            start.y += 1;
        } else if start.y > end.y {
            path.push('^');
            start.y -= 1;
        } else if start.x < end.x {
            path.push('>');
            start.x += 1;
        } else if start.x > end.x {
            path.push('<');
            start.x -= 1;
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

fn print_code(code: &Code) {
    let code_str: String = code.iter().cloned().collect();
    println!("{}", code_str);
}

fn get_code_input(code: &Code) -> Code {
    print_code(&code);
    let robot_1_input = get_inputs(&code);
    print_code(&robot_1_input);
    let robot_2_input = get_inputs(&robot_1_input);
    print_code(&robot_2_input);
    let robot_3_input = get_inputs(&robot_2_input);
    print_code(&robot_3_input);
    robot_3_input
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
            let code_input = get_code_input(&code);
            let complexity = code_input.len();
            let code_numeric = get_code_numeric(&code);
            println!("{complexity} {code_numeric}");
            complexity as u64 * code_numeric
        })
        .sum();
    println!("{complexity_sum}");
}
