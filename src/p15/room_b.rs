use super::direction::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Object {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Object::Empty => '.',
            Object::Wall => '#',
            Object::BoxLeft => '[',
            Object::BoxRight => ']',
            Object::Robot => '@',
        };
        write!(f, "{}", ch)?;
        Ok(())
    }
}

type Grid = Vec<Vec<Object>>;

pub struct Room {
    height: usize,
    width: usize,
    grid: Grid,
}

impl Room {
    pub fn new(grid: Grid) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        Room {
            height,
            width,
            grid,
        }
    }

    /// Creates a box on cells (x, y) and (x+1, y)
    pub fn make_box(&mut self, y: usize, x: usize) {
        self.grid[y][x] = Object::BoxLeft;
        self.grid[y][x + 1] = Object::BoxRight;
    }

    /// Removes the box at (x, y)
    pub fn clear_box(&mut self, y: usize, x: usize) {
        self.grid[y][x] = Object::Empty;
        self.grid[y][x + 1] = Object::Empty;
    }

    /// Returns true and updates self if object is pushable
    /// Otherwise, returns false
    fn is_pushable(&self, dir: &Direction, y: usize, x: usize) -> bool {
        match self.grid[y][x] {
            Object::Wall => false,
            Object::Empty => true,
            Object::BoxLeft => match dir {
                Direction::Left => self.is_pushable(dir, y, x - 1),
                Direction::Right => self.is_pushable(dir, y, x + 2),
                Direction::Up => {
                    self.is_pushable(dir, y - 1, x) && self.is_pushable(dir, y - 1, x + 1)
                }
                Direction::Down => {
                    self.is_pushable(dir, y + 1, x) && self.is_pushable(dir, y + 1, x + 1)
                }
            },
            Object::BoxRight => self.is_pushable(dir, y, x - 1),
            Object::Robot => {
                let offset = dir_to_offset(dir);
                let y = (y as i64 + offset.y) as usize;
                let x = (x as i64 + offset.x) as usize;
                self.is_pushable(dir, y, x)
            }
        }
    }

    /// Tries to push the object at (x,y) in direction `dir`. If the object
    /// cannot be pushed, then no change occurs.
    fn push(&mut self, dir: &Direction, y: usize, x: usize) {
        match self.grid[y][x] {
            Object::Wall | Object::Empty => (),
            Object::BoxLeft => match dir {
                Direction::Left => {
                    if self.is_pushable(dir, y, x - 1) {
                        self.push(dir, y, x - 1);
                        self.clear_box(y, x);
                        self.make_box(y, x - 1);
                    }
                }
                Direction::Right => {
                    if self.is_pushable(dir, y, x + 2) {
                        self.push(dir, y, x + 2);
                        self.clear_box(y, x);
                        self.make_box(y, x + 1);
                    }
                }
                Direction::Up => {
                    if self.is_pushable(dir, y - 1, x) && self.is_pushable(dir, y - 1, x + 1) {
                        self.push(dir, y - 1, x);
                        self.push(dir, y - 1, x + 1);
                        self.clear_box(y, x);
                        self.make_box(y - 1, x);
                    }
                }
                Direction::Down => {
                    if self.is_pushable(dir, y + 1, x) && self.is_pushable(dir, y + 1, x + 1) {
                        self.push(dir, y + 1, x);
                        self.push(dir, y + 1, x + 1);
                        self.clear_box(y, x);
                        self.make_box(y + 1, x);
                    }
                }
            },
            Object::BoxRight => {
                // The left side of the box is always responsible for pushing logic.
                // The right side simply delegates to it's left side counterpart.
                self.push(dir, y, x - 1)
            }
            Object::Robot => {
                let offset = dir_to_offset(dir);
                let ny = (y as i64 + offset.y) as usize;
                let nx = (x as i64 + offset.x) as usize;
                if self.is_pushable(dir, ny, nx) {
                    self.push(dir, ny, nx);
                    self.grid[y][x] = Object::Empty;
                    self.grid[ny][nx] = Object::Robot;
                }
            }
        }
    }

    fn find_robot(&self) -> (usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == Object::Robot {
                    return (y, x);
                }
            }
        }
        panic!("No robot found?!");
    }

    pub fn update(&mut self, dir: &Direction) {
        let (robot_y, robot_x) = self.find_robot();
        self.push(dir, robot_y, robot_x);
    }

    pub fn compute_gps_sum(&self) -> u64 {
        let mut total = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == Object::BoxLeft {
                    total += 100 * y + x;
                }
            }
        }
        total as u64
    }
}

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for object in row {
                write!(f, "{}", object)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
