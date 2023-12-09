#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Series {
    data: Vec<usize>,
}

impl Series {
    fn new(data: Vec<usize>) -> Self {
        Self { data }
    }

    fn parse(input: &str) -> Self {
        let data = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn series_parse() {
        let input = "10  13  16  21  30  45";
        let expect = Series::new(vec![10, 13, 16, 21, 30, 45]);
        let result = Series::parse(input);
        assert_eq!(result, expect);
    }
}
