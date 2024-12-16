use std::ops::{Add, Div, Mul, Rem};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Vec2 { x, y }
    }

    pub fn to_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn to_usize(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, oth: Self) -> Self::Output {
        Vec2::new(self.x + oth.x, self.y + oth.y)
    }
}

impl Mul<i64> for Vec2 {
    type Output = Self;

    fn mul(self, oth: i64) -> Self::Output {
        Vec2::new(self.x * oth, self.y * oth)
    }
}
impl Div<i64> for Vec2 {
    type Output = Self;

    fn div(self, oth: i64) -> Self::Output {
        Vec2::new(self.x / oth, self.y / oth)
    }
}

impl Rem for Vec2 {
    type Output = Self;
    fn rem(self, oth: Vec2) -> Self::Output {
        Vec2::new(
            (self.x % oth.x + oth.x) % oth.x,
            (self.y % oth.y + oth.y) % oth.y,
        )
    }
}
