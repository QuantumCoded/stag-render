use std::ops::{Add, Sub};

#[derive(Clone, Debug, Default)]
pub struct Len {
    pub ch: f32,
    pub em: f32,
    pub px: f32,
}

impl Add for Len {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ch: self.ch + rhs.ch,
            em: self.em + rhs.em,
            px: self.px + rhs.px,
        }
    }
}

impl Sub for Len {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ch: self.ch - rhs.ch,
            em: self.em - rhs.em,
            px: self.px - rhs.px,
        }
    }
}

impl Len {
    pub fn as_px(&self) -> f32 {
        // TODO: font_size and font_ratio will need to be passed in
        // these will be assumed to be 20px and 0.6 respectively

        self.ch * 20.0 * 0.6 + self.em * 20.0 + self.px
    }
}
