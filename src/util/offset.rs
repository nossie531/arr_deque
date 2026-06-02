//! Provider of [`Offset`].

use crate::util::*;

/// Offset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Offset {
    /// Direction.
    dir: Dir,
    /// Length.
    len: usize,
}

impl Offset {
    /// Creates a new instance.
    pub fn new(dir: Dir, len: usize) -> Self {
        let dir = Self::normalize_dir(dir, len);
        Self { dir, len }
    }

    /// Creates a new ofsset from `x` to `y`.
    pub fn measure(x: usize, y: usize) -> Self {
        Self::new(Dir::inc(x < y), x.abs_diff(y))
    }

    /// Creates a new value with reversed direction.
    pub fn flip(self) -> Self {
        Self::new(!self.dir, self.len)
    }

    /// Returns `true` if this offset has increasing direction.
    pub fn is_inc(&self) -> bool {
        self.dir.is_inc()
    }

    /// Returns direction.
    pub fn dir(&self) -> Dir {
        self.dir
    }

    /// Returns length.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Normalize direction with length.
    fn normalize_dir(dir: Dir, len: usize) -> Dir {
        if len == 0 { Dir::Inc } else { dir }
    }
}
