//! Pure application state; no UI, I/O or global mutable state.
#[derive(Default)]
pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn increment(&mut self) -> u32 {
        self.value = self.value.saturating_add(1);
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;

    #[test]
    fn increment_saturates_instead_of_wrapping() {
        let mut model = Counter { value: u32::MAX };
        assert_eq!(model.increment(), u32::MAX);
    }

    #[test]
    fn instances_are_independent() {
        let mut a = Counter::default();
        let mut b = Counter::default();
        assert_eq!(a.increment(), 1);
        assert_eq!(a.increment(), 2);
        assert_eq!(b.increment(), 1);
    }
}
