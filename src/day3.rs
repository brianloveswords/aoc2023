#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet};

/*
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn overlaps(&self, other: &Self) -> bool {
        if other.start >= self.start && other.start < self.end {
            return true;
        }

        if other.end < self.end && other.end > self.start {
            return true;
        }

        return false;
    }

    fn expand(&self, n: usize) -> Self {
        Self {
            start: self.start - n,
            end: self.end + n,
        }
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        let expanded = self.expand(1);
        expanded.overlaps(other)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Token {
    Part {
        number: usize,
        line: usize,
        range: Range,
    },
    Symbol {
        symbol: char,
        line: usize,
        range: Range,
    },
}

impl Token {
    fn line(&self) -> usize {
        match self {
            Token::Part { line, .. } => *line,
            Token::Symbol { line, .. } => *line,
        }
    }

    fn get_part_number(&self) -> Option<usize> {
        match self {
            Token::Part { number, .. } => Some(*number),
            _ => None,
        }
    }

    fn range(&self) -> Range {
        match self {
            Token::Part { range, .. } => *range,
            Token::Symbol { range, .. } => *range,
        }
    }

    fn is_symbol(&self) -> bool {
        match self {
            Token::Symbol { .. } => true,
            _ => false,
        }
    }

    fn is_possible_gear(&self) -> bool {
        match self {
            Token::Symbol { symbol, .. } => *symbol == '*',
            _ => false,
        }
    }

    fn is_part(&self) -> bool {
        match self {
            Token::Part { .. } => true,
            _ => false,
        }
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        // check if it's the same line, or one above, or one below
        let this_line = self.line();
        let other_line = other.line();

        if other_line != this_line && other_line + 1 != this_line && other_line - 1 != this_line {
            eprintln!("lines don't match: this: {this_line}, other: {other_line}");
            return false;
        }

        let this_range = self.range();
        let other_range = other.range();
        this_range.is_adjacent(&other_range)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SchematicParser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> SchematicParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.bytes.get(self.offset).map(|b| char::from(*b))
    }

    fn next_while<F>(&mut self, mut f: F) -> Option<&'a str>
    where
        F: FnMut(char) -> bool,
    {
        let start = self.offset;
        while let Some(c) = self.peek() {
            if !f(c) {
                break;
            }
            self.next();
        }

        if self.offset == start {
            return None;
        }

        Some(&self.input[start..self.offset])
    }

    fn next(&mut self) -> Option<char> {
        let c = self.bytes.get(self.offset).map(|b| char::from(*b))?;
        self.offset += 1;
        self.column += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        }
        Some(c)
    }

    fn parse_token(&mut self) -> Option<Token> {
        // drop whitespace
        self.next_while(|c| c == '.' || c.is_whitespace());

        // keep track of starting position. we won't cross a line boundary.
        let line = self.line;
        let start_column = self.column;

        // try to parse a number
        if let Some(number) = self.next_while(|c| c.is_digit(10)) {
            let number = number
                .parse()
                .expect("filtered for digits, should have a number");
            let range = Range::new(start_column, self.column);
            return Some(Token::Part {
                number,
                line,
                range,
            });
        }

        // everything else is a symbol
        let symbol = self.next()?;
        let range = Range::new(start_column, self.column);
        Some(Token::Symbol {
            symbol,
            line,
            range,
        })
    }
}

fn part2(s: &str) -> usize {
    let mut parser = SchematicParser::new(s);
    let mut part_map: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut gear_map: BTreeMap<_, Vec<_>> = BTreeMap::new();

    while let Some(token) = parser.parse_token() {
        let token_line = token.line();
        let map = match token {
            Token::Part { .. } => &mut part_map,
            t @ Token::Symbol { .. } if t.is_possible_gear() => &mut gear_map,
            Token::Symbol { .. } => continue,
        };

        let line = map.entry(token_line).or_default();
        line.push(token);
    }

    let mut total_ratio = 0;

    for (line, symbols) in gear_map {
        let above = part_map.get(&(line - 1));
        let below = part_map.get(&(line + 1));
        let same = part_map.get(&line);

        let mut all = vec![];
        if let Some(above) = above {
            all.extend(above);
        }
        if let Some(below) = below {
            all.extend(below);
        }
        if let Some(same) = same {
            all.extend(same);
        }

        for symbol in symbols {
            let mut parts = vec![];

            for part in all.iter() {
                if symbol.is_adjacent(part) {
                    parts.push(*part);
                }
            }

            if parts.len() != 2 {
                eprintln!("symbol {symbol:?}: not a gear, expected 2 parts, found {parts:?}");
                continue;
            }

            let ratio1 = parts[0].get_part_number().expect("should be a part");
            let ratio2 = parts[1].get_part_number().expect("should be a part");

            let ratio = ratio1 * ratio2;
            total_ratio += ratio;
        }
    }

    total_ratio
}

fn part1(input: &str) -> usize {
    let mut parser = SchematicParser::new(input);
    let mut part_map: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut gear_map: BTreeMap<_, Vec<_>> = BTreeMap::new();

    while let Some(token) = parser.parse_token() {
        let token_line = token.line();
        let map = match token {
            Token::Part { .. } => &mut part_map,
            Token::Symbol { .. } => &mut gear_map,
        };

        let line = map.entry(token_line).or_default();
        line.push(token);
    }

    let mut found_parts = BTreeSet::new();

    for (line, symbols) in gear_map {
        let above = part_map.get(&(line - 1));
        let below = part_map.get(&(line + 1));
        let same = part_map.get(&line);

        let mut all = vec![];
        if let Some(above) = above {
            all.extend(above);
        }
        if let Some(below) = below {
            all.extend(below);
        }
        if let Some(same) = same {
            all.extend(same);
        }

        for symbol in symbols {
            for number in all.iter() {
                if symbol.is_adjacent(number) {
                    found_parts.insert(*number);
                }
            }
        }
    }

    // println!("found numbers: {:?}", found_numbers);
    let total = found_parts
        .iter()
        .map(|num| num.get_part_number().expect("should be a number"))
        .sum::<usize>();

    total
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_token() {
        let lines = "467..114..\n&..35..633";
        let mut parser = SchematicParser::new(lines);
        let token = parser.parse_token();
        assert_eq!(
            token,
            Some(Token::Part {
                number: 467,
                line: 1,
                range: Range::new(1, 4),
            })
        );

        let token = parser.parse_token();
        assert_eq!(
            token,
            Some(Token::Part {
                number: 114,
                line: 1,
                range: Range::new(6, 9),
            })
        );

        let token = parser.parse_token();
        assert_eq!(
            token,
            Some(Token::Symbol {
                symbol: '&',
                line: 2,
                range: Range::new(1, 2),
            })
        );

        let token = parser.parse_token();
        assert_eq!(
            token,
            Some(Token::Part {
                number: 35,
                line: 2,
                range: Range::new(4, 6),
            })
        );
    }

    #[test]
    fn test_is_adjacent() {
        let lines = ".1.\n*..";
        let mut parser = SchematicParser::new(lines);
        let number = parser.parse_token().unwrap();
        assert_eq!(number.is_part(), true);

        let symbol = parser.parse_token().unwrap();
        assert_eq!(symbol.is_symbol(), true);

        let adjacent = number.is_adjacent(&symbol);
        assert_eq!(adjacent, true);
    }

    #[test]
    fn test_part1_example() {
        let input = include_str!("../inputs/examples/day3.txt");
        let total = part1(input);
        assert_eq!(total, 4361);
    }

    #[test]
    fn test_part1_real() {
        let input = include_str!("../inputs/real/day3.txt");
        let total = part1(input);
        println!("part1: {}", total);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../inputs/examples/day3.txt");
        let total = part2(input);
        assert_eq!(total, 467835);
    }
}
