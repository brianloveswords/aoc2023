#![allow(dead_code)]

use std::ops;

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day05.txt");
pub const REAL: &str = include_str!("../../inputs/real/day05.txt");

pub fn part1(s: &str) -> u64 {
    let almanac = Almanac::parse(s);
    let results = almanac.process_all_seeds();
    *(results.iter().min().unwrap())
}

pub fn part2(s: &str) -> u64 {
    let almanac = Almanac::parse(s);
    let results = almanac.process_seed_ranges();
    *(results.iter().min().unwrap())
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
pub struct SeedIterator {
    ranges: Vec<ops::Range<u64>>,
    idx: usize,
}

impl SeedIterator {
    fn new(ranges: Vec<ops::Range<u64>>) -> Self {
        Self { ranges, idx: 0 }
    }
    fn empty() -> Self {
        Self {
            ranges: vec![],
            idx: 0,
        }
    }

    fn add_range(&mut self, range: ops::Range<u64>) {
        self.ranges.push(range);
    }
}

impl Iterator for SeedIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.ranges.get_mut(self.idx)?;

        if let Some(n) = current.next() {
            Some(n)
        } else {
            self.idx += 1;
            println!("advancing to next range: {}", self.idx);

            self.next()
        }
    }
}

impl Almanac {
    fn process_all_seeds(&self) -> Vec<u64> {
        self.seeds.iter().map(|s| self.process_seed(*s)).collect()
    }

    fn process_seed_ranges(&self) -> Vec<u64> {
        self.seed_range_iter()
            .map(|s| self.process_seed(s))
            .collect()
    }

    pub fn seed_range_iter(&self) -> SeedIterator {
        let mut result = SeedIterator::empty();
        for pair in self.seeds.chunks(2) {
            assert!(pair.len() == 2, "expected exactly 2 seeds, got {pair:?}");

            let start = pair[0];
            let end = start + pair[1];
            result.add_range(start..end);
        }

        result
    }

    fn process_seed(&self, seed: u64) -> u64 {
        let mut result = seed;
        for map in &self.maps {
            result = map.apply(result) as u64;
        }
        result
    }

    pub fn parse(s: &str) -> Self {
        let blocks: Vec<_> = s.split("\n\n").collect();

        let seeds = blocks[0]
            .split_once(':')
            .expect("no seeds")
            .1
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u64>().expect("not a number"))
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
                            .map(|s| s.parse::<u64>().expect("not a number"))
                            .collect::<Vec<_>>();
                        assert!(numbers.len() == 3, "expected exactly 3 numbers");
                        Conversion::new(numbers[0], numbers[1], numbers[2])
                    })
                    .collect();
                Map::new(result)
            })
            .collect();

        Almanac { seeds, maps }
    }
}

#[derive(Debug)]
struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn new(conversions: Vec<Conversion>) -> Self {
        Self { conversions }
    }

    fn apply(&self, input: u64) -> u64 {
        for conversion in &self.conversions {
            if let Some(result) = conversion.attempt(input) {
                return result;
            }
        }
        input
    }
}

struct Conversion {
    start: u64,
    end: u64,
    offset: u64,
    apply: Box<dyn Fn(u64) -> u64>,
}

impl std::fmt::Debug for Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conversion")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("offset", &self.offset)
            .finish()
    }
}

impl Conversion {
    fn new(destination: u64, source: u64, length: u64) -> Self {
        let offset = if destination > source {
            destination - source
        } else {
            source - destination
        };

        let combine = if destination > source {
            u64::saturating_add
        } else {
            u64::saturating_sub
        };

        let op = Box::new(move |i| (combine)(i, offset));
        let start = source;
        let end = source + length;

        Self {
            start,
            end,
            offset,
            apply: op,
        }
    }

    fn matches(&self, input: u64) -> bool {
        input >= self.start && input < self.end
    }

    fn apply(&self, input: u64) -> u64 {
        if self.matches(input) {
            return (self.apply)(input);
        }
        input
    }

    fn attempt(&self, input: u64) -> Option<u64> {
        if self.matches(input) {
            return Some((self.apply)(input));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 35);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(REAL), 173706076);
    }

    #[test]
    fn almanac_process_seed() {
        let almanac = Almanac::parse(EXAMPLE);
        assert_eq!(almanac.process_seed(79), 82);
        assert_eq!(almanac.process_seed(14), 43);
        assert_eq!(almanac.process_seed(55), 86);
        assert_eq!(almanac.process_seed(13), 35);
    }

    #[test]
    fn conversion_apply() {
        let conversion = Conversion::new(20, 30, 5);
        assert_eq!(conversion.apply(29), 29);
        assert_eq!(conversion.apply(30), 20);
        assert_eq!(conversion.apply(31), 21);
        assert_eq!(conversion.apply(32), 22);
        assert_eq!(conversion.apply(33), 23);
        assert_eq!(conversion.apply(34), 24);
        assert_eq!(conversion.apply(35), 35);
    }

    #[test]
    fn map_apply() {
        let map = Map::new(vec![
            Conversion::new(30, 20, 1),
            Conversion::new(32, 22, 2),
            Conversion::new(35, 25, 3),
        ]);

        assert_eq!(map.apply(19), 19);
        assert_eq!(map.apply(20), 30);
        assert_eq!(map.apply(21), 21);
        assert_eq!(map.apply(22), 32);
        assert_eq!(map.apply(23), 33);
        assert_eq!(map.apply(24), 24);
        assert_eq!(map.apply(25), 35);
        assert_eq!(map.apply(26), 36);
        assert_eq!(map.apply(27), 37);
        assert_eq!(map.apply(28), 28);
    }
}
