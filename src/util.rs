#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Range {
    start: u32,
    end: u32,
}

impl Range {
    /// Creates a new range from the given start and end (exclusive).
    ///
    /// # Examples
    /// ```
    /// use aoc2023::util::Range;
    ///
    /// let range = Range::new(0, 10);
    /// ```
    pub fn new(start: u32, end: u32) -> Self {
        assert!(start <= end, "start must be <= end");
        Self { start, end }
    }

    /// Returns true if the given range overlaps with this range.
    ///
    /// # Examples
    /// ```
    /// use aoc2023::util::Range;
    ///
    /// let a = Range::new(0, 10);
    /// let b = Range::new(5, 15);
    /// let c = Range::new(10, 20);
    ///
    /// assert!(a.is_overlapping(&b));
    /// assert!(!a.is_overlapping(&c)); // touching is not overlapping
    /// ```
    pub fn is_overlapping(&self, other: &Self) -> bool {
        // get the sizes of the independent ranges, then collapse
        // the range using (min, max) of the pair. if the size is less
        // than the sum of the sizes, then there is overlap.

        let a = self.size();
        let b = other.size();
        let c = self.combine(&other).size();

        c < a + b
    }

    fn size(&self) -> u64 {
        (self.end - self.start) as u64
    }

    fn combine(&self, other: &Self) -> Self {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        Self::new(start, end)
    }

    /// Returns true if the given range is adjacent to this range.
    ///
    /// # Examples
    /// ```
    /// use aoc2023::util::Range;
    ///
    /// let a = Range::new(0, 10);
    /// let b = Range::new(10, 20);
    /// let c = Range::new(11, 30);
    ///
    /// assert!(a.is_adjacent(&b));
    /// assert!(!a.is_adjacent(&c));
    /// ```
    pub fn is_adjacent(&self, other: &Self) -> bool {
        let expanded = self.expand(1);
        expanded.is_overlapping(other)
    }

    fn expand(&self, n: u32) -> Self {
        Self {
            start: self.start.saturating_sub(n),
            end: self.end.saturating_add(n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn gen_range()(a: u32, b: u32) -> Range {
            let (start, end) = if a < b {
                (a, b)
            } else {
                let n = if a == b { 1 } else { 0 };
                (b, a + n)
            };
            Range::new(start, end)
        }
    }

    proptest! {
        #[test]
        fn is_overlapping_commutative(a in gen_range(), b in gen_range()) {
            assert_eq!(a.is_overlapping(&b), b.is_overlapping(&a));
        }
    }

    proptest! {
        #[test]
        fn is_adjacent_commutative(a in gen_range(), b in gen_range()) {
            assert_eq!(a.is_adjacent(&b), b.is_adjacent(&a));
        }
    }

    #[test]
    fn test_is_overlapping() {
        let a = Range::new(1, 2);
        let b = Range::new(0, 2);

        assert!(a.is_overlapping(&b));
        assert!(b.is_overlapping(&a));
    }

    #[test]
    fn test_is_not_overlapping() {
        let a = Range::new(0, 2);
        let b = Range::new(2, 4);

        assert!(!a.is_overlapping(&b));
        assert!(!b.is_overlapping(&a));
    }

    #[test]
    fn test_is_adjacent() {
        let a = Range::new(0, 10);
        let b = Range::new(10, 20);
        let c = Range::new(11, 30);

        assert!(a.is_adjacent(&b));
        assert!(!a.is_adjacent(&c));
    }

    #[test]
    fn test_is_adjacent_overflow() {
        let a = Range::new(u32::MIN, u32::MAX / 2);
        let b = Range::new(u32::MAX / 2, u32::MAX);

        assert!(a.is_adjacent(&b));
        assert!(b.is_adjacent(&a));
    }
}
