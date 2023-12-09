#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Series {
    data: Vec<usize>,
}

impl Series {
    fn new(data: Vec<usize>) -> Self {
        Self { data }
    }

    fn has_converged(&self) -> bool {
        let set = self.data.iter().collect::<HashSet<_>>();
        set.len() == 1
    }

    fn predict_next(&self) -> usize {
        let mut lasts = vec![];
        let mut data = self.data.clone();

        while !has_converged(&data) {
            let last = data.last().expect("empty series");
            lasts.push(*last);

            let mut new_data = vec![];
            for window in data.windows(2) {
                let a = window[0];
                let b = window[1];
                new_data.push(b - a);
            }
            eprintln!("new_data: {:?}", new_data);
            data = new_data;
        }
        lasts.iter().sum()
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

fn has_converged(data: &Vec<usize>) -> bool {
    let set = data.iter().collect::<HashSet<_>>();
    set.len() == 1 && set.contains(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn series_predict_next() {
        let series = Series::parse("0 3 6 9 12 15");
        let result = series.predict_next();
        assert_eq!(result, 18);

        let series = Series::parse("1 3 6 10 15 21");
        let result = series.predict_next();
        assert_eq!(result, 28);

        let series = Series::parse("10  13  16  21  30  45");
        let result = series.predict_next();
        assert_eq!(result, 68);
    }

    #[test]
    fn series_parse() {
        let input = "10  13  16  21  30  45";
        let expect = Series::new(vec![10, 13, 16, 21, 30, 45]);
        let result = Series::parse(input);
        assert_eq!(result, expect);
    }
}
