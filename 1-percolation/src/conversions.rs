use std::u32;

pub trait AsUsizeConverter {
    /// Converts the value of `self` to an `usize`.
    #[inline(always)]
    fn as_usize(&self) -> usize;
}

impl AsUsizeConverter for u32 {
    #[inline(always)]
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

pub trait TryU32Converter {
    /// Converts the value of `self` to an `u32` or panics if it fails
    #[inline(always)]
    fn try_u32(&self) -> u32;
}

impl TryU32Converter for usize {
    #[inline(always)]
    fn try_u32(&self) -> u32 {
        assert!((*self as u64) <= (u32::MAX as u64), format!("{} is too big for a u32", *self));
        *self as u32
    }
}
