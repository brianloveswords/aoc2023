pub const EXAMPLE: &str = include_str!("../../inputs/examples/day4.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Number(usize);

impl Number {
    fn new(n: usize) -> Self {
        Self(n)
    }

    fn parse(s: &str) -> Result<Self, String> {
        s.trim()
            .parse::<usize>()
            .map(Self::new)
            .map_err(|e| format!("failed to parse number: {e}"))
    }

    fn parse_list(s: &str) -> Result<Vec<Self>, String> {
        s.trim().split_whitespace().map(Self::parse).collect()
    }
}

pub fn part1(_s: &str) -> usize {
    todo!("write this")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers() {
        let input = "41      48 83  86  17  ";
        let numbers = Number::parse_list(input).expect("valid input");
        let expected: Vec<_> = vec![41, 48, 83, 86, 17]
            .into_iter()
            .map(Number::new)
            .collect();
        assert_eq!(numbers.len(), 5);
        assert_eq!(numbers, expected);
    }
}
