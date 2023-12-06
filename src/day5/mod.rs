#![allow(dead_code)]

use crate::util::Range;

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day5.txt");

pub fn part1(s: &str) -> usize {
    let blocks: Vec<_> = s.split("\n\n").collect();

    let seeds = blocks[0]
        .split_once(':')
        .expect("no seeds")
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("not a number"))
        .collect::<Vec<_>>();

    let maps: Vec<_> = blocks[1..]
        .iter()
        .map(|block| {
            let result: Vec<_> = block
                .split_once(':')
                .expect("no seeds")
                .1
                .trim()
                .lines()
                .map(|line| {
                    let numbers = line
                        .split_whitespace()
                        .map(|s| s.parse::<u32>().expect("not a number"))
                        .collect::<Vec<_>>();
                    assert!(numbers.len() == 3, "expected exactly 3 numbers");
                    Conversion::new(numbers[0], numbers[1], numbers[2])
                })
                .collect();
            Map::new(result)
        })
        .collect();

    let almanac = Almanac { seeds, maps };

    println!("{almanac:?}");
    println!("{blocks:?}");

    0
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn new(conversions: Vec<Conversion>) -> Self {
        Self { conversions }
    }
}

#[derive(Debug)]
enum Sign {
    Positive,
    Negative,
}

struct Conversion {
    range: Range,
    offset: u32,
    apply: Box<dyn Fn(u32) -> u32>,
}

impl std::fmt::Debug for Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conversion")
            .field("range", &self.range)
            .field("offset", &self.offset)
            .finish()
    }
}

impl Conversion {
    fn new(destination: u32, source: u32, length: u32) -> Self {
        let offset = if destination > source {
            destination - source
        } else {
            source - destination
        };

        let combine = if destination > source {
            u32::saturating_add
        } else {
            u32::saturating_sub
        };

        let op = Box::new(move |i| (combine)(i, offset));
        let range = Range::new(source, source + length);

        Self {
            range,
            offset,
            apply: op,
        }
    }

    fn matches(&self, input: u32) -> bool {
        self.range.includes(input)
    }

    fn apply(&self, input: u32) -> u32 {
        if self.matches(input) {
            return (self.apply)(input);
        }
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_run() {
        let conversion = Conversion::new(20, 30, 5);
        assert_eq!(conversion.apply(29), 29);
        assert_eq!(conversion.apply(30), 20);
        assert_eq!(conversion.apply(31), 21);
        assert_eq!(conversion.apply(32), 22);
        assert_eq!(conversion.apply(33), 23);
        assert_eq!(conversion.apply(34), 24);
        assert_eq!(conversion.apply(35), 35);
    }
}
