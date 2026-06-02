//! Provider of [`Dir`].

use core::ops::Not;

/// Direction.
#[derive(Clone, Copy, Debug)]
pub enum Dir {
    /// Decreasing direction.
    Dec,
    /// Increasing direction.
    Inc,
}

impl Dir {
    /// Create a decreasing value if given falg is true.
    pub fn dec(value: bool) -> Self {
        if value { Self::Dec } else { Self::Inc }
    }

    /// Create a increasing value if given falg is true.
    pub fn inc(value: bool) -> Self {
        if value { Self::Inc } else { Self::Dec }
    }

    /// Returns `true` if this is decreasing direction.
    pub fn is_dec(&self) -> bool {
        !self.is_inc()
    }

    /// Returns `true` if this is increasing direction.
    pub fn is_inc(&self) -> bool {
        matches!(self, Self::Inc)
    }

    /// Returns `0` for decreasing or `1` for increasing.
    pub fn binary(&self) -> usize {
        self.is_inc() as usize
    }
}

impl Not for Dir {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Dec => Self::Inc,
            Self::Inc => Self::Dec,
        }
    }
}
