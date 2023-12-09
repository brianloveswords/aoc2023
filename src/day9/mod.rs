#![allow(dead_code)]

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day9.txt");
pub const REAL: &str = include_str!("../../inputs/real/day9.txt");

pub fn part1(input: &str) -> isize {
    let report = Report::parse(input);
    report.predict_next_total()
}

pub fn part2(input: &str) -> isize {
    let report = Report::parse(input);
    report.predict_prior_total()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Report {
    histories: Vec<History>,
}

impl Report {
    fn new(histories: Vec<History>) -> Self {
        Self { histories }
    }

    pub fn parse(input: &str) -> Self {
        let histories = input.trim().split("\n").map(History::parse).collect();
        Self { histories }
    }

    fn predict_next_total(&self) -> isize {
        self.histories.iter().map(History::predict_next).sum()
    }

    pub fn predict_prior_total(&self) -> isize {
        self.histories.iter().map(History::predict_prior).sum()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct History {
    data: Vec<isize>,
}

impl History {
    fn new(data: Vec<isize>) -> Self {
        Self { data }
    }

    fn predict_next(&self) -> isize {
        predict(self.data.clone(), vec![], CarryMode::Next)
            .iter()
            .sum()
    }

    fn predict_prior(&self) -> isize {
        predict(self.data.clone(), vec![], CarryMode::Prior)
            .into_iter()
            .reduce(|a, b| b - a)
            .expect("predict is empty")
    }

    pub fn parse(input: &str) -> Self {
        let data = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        Self { data }
    }
}

fn has_converged(data: &Vec<isize>) -> bool {
    !data.iter().any(|i| i != &0)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CarryMode {
    Prior,
    Next,
}

impl CarryMode {
    fn is_next(&self) -> bool {
        match self {
            CarryMode::Prior => true,
            CarryMode::Next => false,
        }
    }

    fn is_prior(&self) -> bool {
        !self.is_next()
    }
}

fn predict(data: Vec<isize>, mut carry: Vec<isize>, mode: CarryMode) -> Vec<isize> {
    match mode {
        CarryMode::Prior => carry.push(*data.first().expect("empty series")),
        CarryMode::Next => carry.push(*data.last().expect("empty series")),
    }
    if has_converged(&data) {
        if mode.is_next() {
            carry.reverse();
        }

        return carry;
    }
    let mut new_data = vec![];
    for window in data.windows(2) {
        let a = window[0];
        let b = window[1];
        new_data.push(b - a);
    }
    predict(new_data, carry, mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_predict_prior() {
        let series = History::parse("0 3 6 9 12 15");
        let result = series.predict_prior();
        assert_eq!(result, -3);

        let series = History::parse("1 3 6 10 15 21");
        let result = series.predict_prior();
        assert_eq!(result, 0);

        let series = History::parse("10  13  16  21  30  45");
        let result = series.predict_prior();
        assert_eq!(result, 5);
    }

    #[test]
    fn report_predict_next_total() {
        let report = Report::parse(EXAMPLE);
        let result = report.predict_next_total();
        assert_eq!(result, 114);
    }

    #[test]
    fn report_parse() {
        let input = "1 7 2 1 \n 1 2 3 4";
        let expect = Report::new(vec![History::parse("1 7 2 1"), History::parse("1 2 3 4")]);
        let result = Report::parse(input);
        assert_eq!(result, expect);
    }

    #[test]
    fn history_predict_next() {
        let series = History::parse("0 3 6 9 12 15");
        let result = series.predict_next();
        assert_eq!(result, 18);

        let series = History::parse("1 3 6 10 15 21");
        let result = series.predict_next();
        assert_eq!(result, 28);

        let series = History::parse("10  13  16  21  30  45");
        let result = series.predict_next();
        assert_eq!(result, 68);
    }

    #[test]
    fn history_parse() {
        let input = "10  13  16  21  30  45";
        let expect = History::new(vec![10, 13, 16, 21, 30, 45]);
        let result = History::parse(input);
        assert_eq!(result, expect);
    }
}
