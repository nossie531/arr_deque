pub enum RingBufAlign {
    Front,
    Center,
    Back,
    Wrap,
}

impl RingBufAlign {
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Front, Self::Center, Self::Back, Self::Wrap].into_iter()
    }

    pub fn calc_ring_start(&self, len: usize, capacity: usize) -> usize {
        match self {
            Self::Front => 0,
            Self::Center => (capacity - len) / 2,
            Self::Back => capacity - len.max(1),
            Self::Wrap => capacity - (len / 2).max(1),
        }
    }
}
