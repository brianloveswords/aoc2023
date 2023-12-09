pub fn solve_quadratic(a: i32, b: i32, c: i32) -> (f64, f64) {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;
    let discriminant = b.powi(2) - 4.0 * a * c;
    let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (x1, x2)
}

pub fn greatest_common_factor(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / greatest_common_factor(a, b)
}

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
        self.is_overlapping_sizes(other)
    }

    #[allow(dead_code)]
    fn is_overlapping_bounds(&self, other: &Self) -> bool {
        // .xxx.
        // xx...
        let is_prefix = other.start <= self.start && other.end > self.start;

        // .xxx.
        // ...xx
        let is_suffix = other.start < self.end && other.end >= self.end;

        // .xxx.
        // ..x..
        let is_contained = other.start >= self.start && other.end <= self.end;

        is_prefix || is_suffix || is_contained
    }

    #[allow(dead_code)]
    fn is_overlapping_sizes(&self, other: &Self) -> bool {
        // get the sizes of the independent ranges, then collapse the
        // range using (min, max) of the pair. if the collapsed size is
        // less than the sum of the independent sizes, there is overlap.

        let a = (self.end - self.start) as u64;
        let b = (other.end - other.start) as u64;
        let c = {
            let min = self.start.min(other.start);
            let max = self.end.max(other.end);
            (max - min) as u64
        };

        c < a + b
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

    /// Returns true if the range includes the given value.
    ///
    /// # Examples
    /// ```
    /// use aoc2023::util::Range;
    ///
    /// let range = Range::new(0, 10);
    ///
    /// assert!(range.includes(0));
    /// assert!(range.includes(5));
    /// assert!(range.includes(9));
    /// assert!(!range.includes(10));
    /// ```
    pub fn includes(&self, value: u32) -> bool {
        value >= self.start && value < self.end
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Parser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    offset: usize,
    line: u32,
    column: u32,
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given input.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    /// Returns the current offset into the input.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the current line number.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns the current column number.
    pub fn column(&self) -> u32 {
        self.column
    }

    /// Returns the next byte without consuming it.
    ///
    /// # Examples
    /// ```
    /// use aoc2023::util::Parser;
    ///
    /// let mut parser = Parser::new("abc");
    /// assert_eq!(parser.peek(), Some('a'));
    /// assert_eq!(parser.peek(), Some('a'));
    ///
    /// // now we consume it
    /// assert_eq!(parser.next(), Some('a'));
    ///
    /// assert_eq!(parser.peek(), Some('b'));
    /// ```
    pub fn peek(&self) -> Option<char> {
        self.bytes.get(self.offset).map(|b| char::from(*b))
    }

    /// Returns a string slice containing all the characters that
    /// match the given predicate, consuming them in the process.
    ///
    /// Returns `None` if no characters match the predicate.
    ///
    /// # Examples
    /// ```
    /// use aoc2023::util::Parser;
    ///
    /// let mut parser = Parser::new("abc123");
    /// assert_eq!(parser.next_while(|c| c.is_alphabetic()), Some("abc"));
    /// assert_eq!(parser.next_while(|c| c.is_numeric()), Some("123"));
    /// assert_eq!(parser.next_while(|c| c.is_alphabetic()), None);
    /// ```
    pub fn next_while<F>(&mut self, mut f: F) -> Option<&'a str>
    where
        F: FnMut(char) -> bool,
    {
        let start = self.offset;
        while let Some(c) = self.peek() {
            if !f(c) {
                break;
            }
            self.advance_cursors(c);
        }

        if self.offset == start {
            return None;
        }

        Some(&self.input[start..self.offset])
    }

    /// Returns next byte, consuming it.
    ///
    /// # Examples
    /// ```
    /// use aoc2023::util::Parser;
    ///
    /// let mut parser = Parser::new("abc");
    /// assert_eq!(parser.next(), Some('a'));
    /// assert_eq!(parser.next(), Some('b'));
    /// assert_eq!(parser.next(), Some('c'));
    /// assert_eq!(parser.next(), None);
    /// ```
    pub fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.advance_cursors(c);
        Some(c)
    }

    fn advance_cursors(&mut self, c: char) {
        self.offset += 1;
        self.column += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_greatest_common_factor() {
        assert_eq!(greatest_common_factor(12083, 20513), 281);
    }

    #[test]
    fn test_least_common_multiple() {
        assert_eq!(least_common_multiple(12083, 20513), 882059);
    }

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

    prop_compose! {
        fn gen_ascii_char()(c in "[a-z]") -> char {
            c.chars().next().unwrap()
        }
    }

    proptest! { // Parser property tests
        #[test]
        fn peek_does_not_advance(a in gen_ascii_char(), b in "[a-z]*") {
            let value = format!("{}{}", a, b);
            let p = Parser::new(&value);

            for _ in 0..value.len() {
                assert_eq!(p.peek(), Some(a));
            }

            assert_eq!(p.offset(), 0);
            assert_eq!(p.line(), 1);
            assert_eq!(p.column(), 1);
        }

        #[test]
        fn next_while(a in "[0-9]+", b in "[a-z]+", c in "[0-9]+") {
            let a = a.as_str();
            let b = b.as_str();
            let c = c.as_str();

            let value = format!("{}{}{}", a, b, c);
            let mut p = Parser::new(&value);

            assert_eq!(p.next_while(|c| c.is_numeric()), Some(a));
            assert_eq!(p.next_while(|c| c.is_alphabetic()), Some(b));
            assert_eq!(p.next_while(|c| c.is_numeric()), Some(c));
            assert_eq!(p.next_while(|_| true), None);
        }
    }

    proptest! { // Range property tests
        #[test]
        fn is_overlapping_commutative(a in gen_range(), b in gen_range()) {
            assert_eq!(a.is_overlapping(&b), b.is_overlapping(&a));
        }

        #[test]
        fn is_overlapping_oracle_test(a in gen_range(), b in gen_range()) {
            assert_eq!(a.is_overlapping_bounds(&b), a.is_overlapping_sizes(&b));
            assert_eq!(b.is_overlapping_bounds(&a), b.is_overlapping_sizes(&a));
        }

        #[test]
        fn is_adjacent_commutative(a in gen_range(), b in gen_range()) {
            assert_eq!(a.is_adjacent(&b), b.is_adjacent(&a));
        }
    }

    #[test]
    fn test_includes() {
        let a = Range::new(0, 10);
        assert!(a.includes(0));
        assert!(a.includes(5));
        assert!(a.includes(9));
        assert!(!a.includes(10));
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
