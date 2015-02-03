pub trait UncheckedNumberConversions {
    fn to_usize(&self) -> usize;
}

impl UncheckedNumberConversions for u32 {
    fn to_usize(&self) -> usize {
        *self as usize // Rust doesn't support < 32 bit pointers
    }
}
