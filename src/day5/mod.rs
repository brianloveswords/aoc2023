#![allow(dead_code)]

use crate::util::Range;

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day5.txt");

pub fn part1(s: &str) -> u32 {
    let almanac = Almanac::parse(s);
    println!("{almanac:?}");
    almanac.process_seed(14)
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Almanac {
    fn process_all_seeds(&self) -> Vec<u32> {
        self.seeds.iter().map(|s| self.process_seed(*s)).collect()
    }

    fn process_seed(&self, seed: u32) -> u32 {
        let mut result = seed;
        for map in &self.maps {
            result = map.apply(result) as u32;
        }
        result
    }

    fn parse(s: &str) -> Self {
        let blocks: Vec<_> = s.split("\n\n").collect();

        let seeds = blocks[0]
            .split_once(':')
            .expect("no seeds")
            .1
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("not a number"))
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

    fn apply(&self, input: u32) -> u32 {
        for conversion in &self.conversions {
            if let Some(result) = conversion.attempt(input) {
                return result;
            }
        }
        input
    }
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

    fn attempt(&self, input: u32) -> Option<u32> {
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
