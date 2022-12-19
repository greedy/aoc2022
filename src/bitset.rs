pub trait BitSet {
    fn new() -> Self;
    fn singleton(item: u32) -> Self;
    fn contains(&self, item: u32) -> bool;
    fn insert(&mut self, item: u32) -> bool;
    fn remove(&mut self, item: u32) -> bool;
}

pub trait BitLimits {
    const BITS: u32;
}

impl BitLimits for u64 {
    const BITS: u32= u64::BITS;
}

impl<T> BitSet for T
where
    T: Copy + BitLimits + std::ops::BitAnd<Output = Self> + std::ops::BitOrAssign + std::ops::BitXorAssign + std::ops::Shl<u8, Output = Self> + std::ops::Not + Eq,
    u8: Into<T>,
{
    fn new() -> Self {
        0.into()
    }

    fn singleton(item: u32) -> Self {
        assert!(item < Self::BITS);
        1.into() << item.try_into().unwrap()
    }

    fn contains(&self, item: u32) -> bool {
        if item >= Self::BITS { return false }
        0.into() != *self & Self::singleton(item)
    }

    fn insert(&mut self, item: u32) -> bool {
        assert!(item < Self::BITS);
        if self.contains(item) { return false; }
        *self |= Self::singleton(item);
        true
    }

    fn remove(&mut self, item: u32) -> bool {
        assert!(item < Self::BITS);
        if !self.contains(item) { return false; }
        *self ^= Self::singleton(item);
        true
    }
}

