use super::length::Len;
use std::ops::{Add, Sub};

#[derive(Clone, Debug, Default)]
pub struct Vec2(pub Len, pub Len);

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Vec2 {
    fn x(&self) -> &Len {
        &self.0
    }

    fn y(&self) -> &Len {
        &self.1
    }

    fn width(&self) -> &Len {
        self.x()
    }

    fn height(&self) -> &Len {
        self.y()
    }
}
