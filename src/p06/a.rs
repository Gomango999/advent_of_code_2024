use super::parser;

enum Direction {
    Up,
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

fn get_dimensions(grid: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = grid.len();
    let cols = grid[0].len();
    (rows as i32, cols as i32)
}

// Returns the (x,y) coordinates of the guard's starting position
fn find_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
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

struct Guard {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Guard {
    fn new(grid: &Vec<Vec<char>>) -> Self {
        let (x, y) = find_start(&grid);
        Guard {
            x,
            y,
            dir: Direction::Up,
        }
    }

    fn make_move(self: &mut Self, grid: &Vec<Vec<char>>) {
        let tried_directions = 0;
        while tried_directions < 4 {
            // try move forward
            let (ox, oy) = Direction::get_offset(&self.dir);
            let (nx, ny) = (self.x + ox, self.y + oy);

            if !is_inside_grid(&nx, &ny, &grid) {
                self.x = nx;
                self.y = ny;
                return;
            }

            if grid[ny as usize][nx as usize] != '#' {
                self.x = nx;
                self.y = ny;
                return;
            }

            // didn't work, try turning right and try again
            self.dir = Direction::turn_right(&self.dir)
        }
        // We can only reach this spot if the guard ends up being surrounded
        // on all 4 sides by obstructions.
        panic!("Guard is stuck!");
    }
}

fn is_inside_grid(x: &i32, y: &i32, grid: &Vec<Vec<char>>) -> bool {
    let (n, m) = get_dimensions(grid);
    (0..n).contains(&y) && (0..m).contains(&x)
}

pub fn solve() {
    let grid = parser::parse();
    let (num_rows, num_cols) = get_dimensions(&grid);

    // move the guard until they're outside.
    let mut guard = Guard::new(&grid);
    let mut seen = vec![vec![false; num_cols as usize]; num_rows as usize];
    while is_inside_grid(&guard.x, &guard.y, &grid) {
        seen[guard.y as usize][guard.x as usize] = true;
        guard.make_move(&grid);
    }

    // count up tiles seen by the guard.
    let mut count = 0;
    for y in 0..num_rows {
        for x in 0..num_cols {
            if seen[y as usize][x as usize] {
                count += 1;
            }
        }
    }
    println!("{count}");
}
