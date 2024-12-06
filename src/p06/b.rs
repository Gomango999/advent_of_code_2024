use super::parser;

type Grid = Vec<Vec<char>>;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(dir: &Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_offset(dir: &Direction) -> (i32, i32) {
        match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn get_dimensions(grid: &Grid) -> (i32, i32) {
    let rows = grid.len();
    let cols = grid[0].len();
    (rows as i32, cols as i32)
}

// Returns the (x,y) coordinates of the guard's starting position
fn find_start(grid: &Grid) -> (i32, i32) {
    let (n, m) = get_dimensions(grid);
    for y in 0..n {
        for x in 0..m {
            if grid[y as usize][x as usize] == '^' {
                return (x, y);
            }
        }
    }
    return (0, 0); // will never reach here
}

fn is_inside_grid(x: &i32, y: &i32, grid: &Grid) -> bool {
    let (n, m) = get_dimensions(grid);
    (0..n).contains(&y) && (0..m).contains(&x)
}

#[derive(Clone, Copy, Debug)]
struct State {
    x: i32,
    y: i32,
    dir: Direction,
}

impl State {
    fn new(grid: &Grid) -> Self {
        let (x, y) = find_start(&grid);
        State {
            x,
            y,
            dir: Direction::Up,
        }
    }

    // Returns `None` if there is no possible next move (the guard is stuck)
    // Otherwise returns the new state
    fn get_next_state(&self, grid: &Grid) -> Option<State> {
        let mut state = self.clone();
        let tried_directions = 0;
        while tried_directions < 4 {
            // Try move forward
            let (ox, oy) = Direction::get_offset(&state.dir);
            let (nx, ny) = (state.x + ox, state.y + oy);

            if !is_inside_grid(&nx, &ny, &grid) {
                (state.x, state.y) = (nx, ny);
                return Some(state);
            }

            if grid[ny as usize][nx as usize] != '#' {
                (state.x, state.y) = (nx, ny);
                return Some(state);
            }

            // Didn't work, try turning right and try again
            state.dir = Direction::turn_right(&state.dir);
        }
        None
    }

    // Makes a move if it can, otherwise does nothing.
    fn update_state(&mut self, grid: &Grid) {
        if let Some(new_state) = self.get_next_state(grid) {
            *self = new_state
        }
    }

    fn is_inside_grid(&self, grid: &Grid) -> bool {
        is_inside_grid(&self.x, &self.y, &grid)
    }
}

// Places a wall in front of the guard, then simulates the guard movements
// from the beginning
// Panics if called with a state and grid where the guard is trapped.
fn check_cycle(state: &State, grid: &Grid) -> bool {
    // The guard cannot be trapped on regular patrol, so safe to unwrap
    let State {
        x: wall_x,
        y: wall_y,
        dir: _,
    } = state.get_next_state(&grid).unwrap();
    if !is_inside_grid(&wall_x, &wall_y, &grid) {
        return false;
    }
    let is_already_obstructed = grid[wall_y as usize][wall_x as usize] == '#';
    let is_starting_position = grid[wall_y as usize][wall_x as usize] == '^';
    if is_already_obstructed || is_starting_position {
        return false;
    }

    // Clone the entire grid and make a wall in front.
    let mut grid = grid.clone();
    grid[wall_y as usize][wall_x as usize] = '#';

    let (num_rows, num_cols) = get_dimensions(&grid);
    // seen[y][x][d] means that the guard has gone to [x][y] with direction d before
    let mut seen = vec![vec![vec![false; 4]; num_cols as usize]; num_rows as usize];

    // Simulate guard movement
    let mut state = State::new(&grid);
    while state.is_inside_grid(&grid) {
        // If we've been in this position before, then we've found a cycle!
        if seen[state.y as usize][state.x as usize][state.dir as usize] {
            return true;
        }
        seen[state.y as usize][state.x as usize][state.dir as usize] = true;

        state.update_state(&grid);
    }
    return false;
}

pub fn solve() {
    // From part a, we know that the guard naturally reaches ~5200 spaces left
    // to her own devices. Thus there are only 5200 spots to test placing an
    // obstacle at. Each one requires the guard to walk for at most 130 * 130 * 4
    // = 67600 tiles before we can confirm if there's a loop (and on average,
    // alot less than that.) This means about ~350mil operations, which is
    // feasibly run. Takes about ~15s on my machine

    let grid = parser::parse();
    let (num_rows, num_cols) = get_dimensions(&grid);

    let mut makes_cycle = vec![vec![false; num_cols as usize]; num_rows as usize];
    let mut state = State::new(&grid);
    while state.is_inside_grid(&grid) {
        if check_cycle(&state, &grid) {
            let next_state = state.get_next_state(&grid).unwrap();
            makes_cycle[next_state.y as usize][next_state.x as usize] = true;
        }
        state.update_state(&grid);
    }

    // Count up unique tiles that could cause a cycle
    let mut count = 0;
    for y in 0..num_rows {
        for x in 0..num_cols {
            if makes_cycle[y as usize][x as usize] {
                count += 1;
            }
        }
    }
    println!("{count}");
}
