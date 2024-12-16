use super::parser;
use super::Direction;

fn get_dimensions(grid: &Vec<Vec<char>>) -> (i64, i64) {
    let rows = grid.len();
    let cols = grid[0].len();
    (rows as i64, cols as i64)
}

// Returns the (x,y) coordinates of the guard's starting position
fn find_start(grid: &Vec<Vec<char>>) -> (i64, i64) {
    let (n, m) = get_dimensions(grid);
    for y in 0..n {
        for x in 0..m {
            if grid[y as usize][x as usize] == '^' {
                return (x, y);
            }
        }
    }
    panic!("Couldn't find the start position!")
}

struct Guard {
    x: i64,
    y: i64,
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
            // Try to move forward
            let (ox, oy) = self.dir.to_offset().to_tuple();
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

            // We were blocked by a wall, turn right and try again
            self.dir = self.dir.turn_right();
        }
        // We can only reach this spot if the guard ends up being surrounded
        // on all 4 sides by obstructions.
        panic!("Guard is stuck!");
    }
}

fn is_inside_grid(x: &i64, y: &i64, grid: &Vec<Vec<char>>) -> bool {
    let (n, m) = get_dimensions(grid);
    (0..n).contains(&y) && (0..m).contains(&x)
}

pub fn solve() {
    let grid = parser::parse();
    let (num_rows, num_cols) = get_dimensions(&grid);

    // Simulate the guard until they exit the grid
    let mut guard = Guard::new(&grid);
    let mut seen = vec![vec![false; num_cols as usize]; num_rows as usize];
    while is_inside_grid(&guard.x, &guard.y, &grid) {
        seen[guard.y as usize][guard.x as usize] = true;
        guard.make_move(&grid);
    }
}
