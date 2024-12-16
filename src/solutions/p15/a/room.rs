use super::super::Direction;
use super::super::Vec2;

type GridA = Vec<Vec<char>>;

#[derive(Clone, Debug)]
pub struct Room {
    height: usize,
    width: usize,
    grid: GridA,
    robot: Vec2,
}

impl Room {
    pub fn new(grid: GridA) -> Self {
        let height = grid.len();
        let width = grid[0].len();

        let mut robot = Vec2::new(0, 0);
        'find_robot: for y in 0..height {
            for x in 0..width {
                if grid[y][x] == '@' {
                    robot = Vec2::new(x as i64, y as i64);
                    break 'find_robot;
                }
            }
        }
        assert_ne!(robot, Vec2::new(0, 0));

        Room {
            height,
            width,
            grid,
            robot,
        }
    }

    pub fn update(&mut self, dir: &Direction) {
        let mut grid = self.grid.clone();
        let mut pos = self.robot.clone();

        // keep moving in the direction until we hit a wall
        let mut previous_object = '.';
        const TIMEOUT_THRESHOLD: u64 = 100;
        for _ in 0..TIMEOUT_THRESHOLD {
            let y = pos.y as usize;
            let x = pos.x as usize;

            // Move previous object into new position, and store the current
            // object.
            (previous_object, grid[y][x]) = (grid[y][x], previous_object);

            if previous_object == '.' {
                // Moved something into air. We can end the simulation.
                let robot = self.robot + dir.to_offset();
                self.grid = grid;
                self.robot = robot;
                return;
            }

            if previous_object == '#' {
                // Tried to move something into a wall. This is illegal, so
                // we return the original unchanged
                return;
            }

            pos = pos + dir.to_offset();
        }
        panic!("Timeout while simulating")
    }

    pub fn compute_gps_sum(&self) -> u64 {
        let mut total = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == 'O' {
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
            let row: String = row.iter().collect();
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}
