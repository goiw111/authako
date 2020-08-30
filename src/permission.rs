use std::ops::{BitOr, BitAnd};

pub enum Permission {
    Create = 0b0001,
    Peruse = 0b0010,
    Update = 0b0100,
    Delete = 0b1000,
    None   = 0b0000,
}

impl BitOr for Permission {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitAnd for Permission {
    type Output = u8;
    fn bitand(self, rhs: Self) -> Self::Output {
        self as u8 & rhs as u8
    }
}
