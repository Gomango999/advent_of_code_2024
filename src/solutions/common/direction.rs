use super::vec2::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const NUM_VARIANTS: usize = 4;

    pub fn all() -> [Direction; Direction::NUM_VARIANTS] {
        [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ]
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn to_offset(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0, -1),
            Direction::Right => Vec2::new(1, 0),
            Direction::Down => Vec2::new(0, 1),
            Direction::Left => Vec2::new(-1, 0),
        }
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
