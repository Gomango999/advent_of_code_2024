use super::vec2::*;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn dir_to_offset(dir: &Direction) -> Vec2 {
    match dir {
        Direction::Up => Vec2::new(0, -1),
        Direction::Right => Vec2::new(1, 0),
        Direction::Down => Vec2::new(0, 1),
        Direction::Left => Vec2::new(-1, 0),
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^")?,
            Direction::Right => write!(f, ">")?,
            Direction::Down => write!(f, "v")?,
            Direction::Left => write!(f, "<")?,
        }
        Ok(())
    }
}
