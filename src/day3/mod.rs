use crate::util::{Parser, Range};
use std::collections::{BTreeMap, HashSet};

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day3.txt");
pub const REAL: &str = include_str!("../../inputs/real/day3.txt");

pub fn part1(input: &str) -> u32 {
    let mut parser = SchematicParser::new(input);

    // since we only need to for adjacency between gears and parts, not all
    // tokens, we can skip some work later by tracking them separately up front.
    //
    // fast lookup by line number is also beneficial for us because we only need
    // to look at neighboring lines to determine gear adjacency.

    let mut part_map = LineTokenMap::new();
    let mut symbol_map = LineTokenMap::new();
    while let Some(token) = parser.parse_token() {
        let map = match token {
            Token::Part { .. } => &mut part_map,
            Token::Symbol { .. } => &mut symbol_map,
        };
        map.insert(token);
    }

    // with an engine schematic like the following:
    //
    // *11*
    //
    // we'd end up finding `11` twice, once for each symbol. we only want to
    // count each part once, so we'll use a HashSet to track parts.

    let mut adjacent_parts: HashSet<Token> = HashSet::new();
    for (line, symbols) in symbol_map {
        let is_adjacent = |t: &&Token| symbols.iter().any(|s| s.is_adjacent(t));
        let surrounding_lines = line - 1..=line + 1;

        let adjacent = surrounding_lines
            .filter_map(|l| part_map.get(l))
            .flatten()
            .filter(is_adjacent);

        adjacent_parts.extend(adjacent);
    }

    adjacent_parts
        .iter()
        .map(|p| p.try_part_number().unwrap())
        .sum()
}

pub fn part2(s: &str) -> u32 {
    let mut parser = SchematicParser::new(s);

    // similar to part 1, but we can save even more work by only tracking
    // the gears instead of all the symbols. we still need to track all parts.
    let mut part_map = LineTokenMap::new();
    let mut gear_map = LineTokenMap::new();
    while let Some(token) = parser.parse_token() {
        let map = match token {
            t if t.is_part() => &mut part_map,
            t if t.is_possible_gear() => &mut gear_map,
            _ => continue,
        };
        map.insert(token);
    }

    let mut total_ratio = 0;
    for (line, gears) in gear_map {
        let nearby = part_map.nearby_tokens(line);

        // spec requires us to have exactly two parts attached to a gear,
        // so we bail early if we find anything different.
        for gear in gears {
            let adjacent = nearby
                .iter()
                .filter(|p| gear.is_adjacent(p))
                .map(|p| p.try_part_number())
                .collect::<Result<Vec<_>, String>>()
                .expect("should only be parts in the part_map");

            if adjacent.len() != 2 {
                continue;
            }

            total_ratio += adjacent[0] * adjacent[1];
        }
    }

    total_ratio
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Token {
    /// Represents a part number in the engine schematic, e.g. `312`
    Part {
        number: u32,
        line: u32,
        range: Range,
    },

    /// Represents a symbol in the engine schematic, e.g. `*`
    Symbol {
        symbol: char,
        line: u32,
        range: Range,
    },
}

impl Token {
    fn line(&self) -> u32 {
        match self {
            Token::Part { line, .. } => *line,
            Token::Symbol { line, .. } => *line,
        }
    }

    fn try_part_number(&self) -> Result<u32, String> {
        match self {
            Token::Part { number, .. } => Ok(*number),
            _ => Err(format!("not a part: {self:?}")),
        }
    }

    fn range(&self) -> Range {
        match self {
            Token::Part { range, .. } => *range,
            Token::Symbol { range, .. } => *range,
        }
    }

    #[allow(unused)]
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

    /// Returns true if the `other` token is adjacent to this token.
    ///
    /// In the example below, all the `*` would be considered adjacent to `77`.
    ///
    /// ```txt
    /// ......
    /// .****.
    /// .*77*.
    /// .****.
    /// ......
    /// ```
    ///
    fn is_adjacent(&self, other: &Self) -> bool {
        self.is_adjacent_y_axis(other) && self.is_adjacent_x_axis(other)
    }

    fn is_adjacent_y_axis(&self, other: &Self) -> bool {
        let this_line = self.line();
        (this_line - 1..=this_line + 1).contains(&other.line())
    }

    fn is_adjacent_x_axis(&self, other: &Self) -> bool {
        self.range().is_adjacent(&other.range())
    }
}

#[derive(Debug)]
struct SchematicParser<'a>(Parser<'a>);

impl<'a> SchematicParser<'a> {
    fn new(input: &'a str) -> Self {
        Self(Parser::new(input))
    }

    fn parse_token(&mut self) -> Option<Token> {
        let p = &mut self.0;

        // drop whitespace. whatever comes after will be a token or EOF
        p.next_while(|c| c == '.' || c.is_whitespace());

        // we won't cross a line boundary during token parsing
        let line = p.line();

        // keep track of starting position for building the token range
        let start_column = p.column();

        // try to parse a number
        if let Some(number) = p.next_while(|c| c.is_digit(10)) {
            let range = Range::new(start_column, p.column());
            let number = number
                .parse()
                .expect("filtered for digits, should have a number");
            return Some(Token::Part {
                number,
                line,
                range,
            });
        }

        // everything else is a symbol
        let symbol = p.next()?;
        let range = Range::new(start_column, p.column());
        Some(Token::Symbol {
            symbol,
            line,
            range,
        })
    }
}

/// LineTokenMap is a map of line numbers to tokens on that line
#[derive(Debug, PartialEq, Eq)]
struct LineTokenMap(BTreeMap<usize, Vec<Token>>);

impl LineTokenMap {
    /// Creates a new empty LineTokenMap
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Inserts a token into the map
    fn insert(&mut self, token: Token) {
        let line = token.line();
        let line_tokens = self.0.entry(line as usize).or_default();
        line_tokens.push(token);
    }

    /// Returns tokens from the line above, the line below, and the current line
    fn nearby_tokens(&self, line: usize) -> Vec<&Token> {
        let nearby_lines = line - 1..=line + 1;
        nearby_lines.filter_map(|l| self.get(l)).flatten().collect()
    }

    /// Returns a reference to the tokens on the given line.
    ///
    /// Returns `None` if the line is empty.
    fn get(&self, line: usize) -> Option<&Vec<Token>> {
        self.0.get(&line)
    }
}

impl IntoIterator for LineTokenMap {
    type Item = (usize, Vec<Token>);
    type IntoIter = std::collections::btree_map::IntoIter<usize, Vec<Token>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
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
        let total = part1(EXAMPLE);
        assert_eq!(total, 4361);
    }

    #[test]
    fn test_part1_real() {
        let total = part1(REAL);
        println!("part1: {}", total);
    }

    #[test]
    fn test_part2_example() {
        let total = part2(EXAMPLE);
        assert_eq!(total, 467835);
    }

    #[test]
    fn test_part2_real() {
        let total = part2(REAL);
        println!("part2: {}", total);
        assert_eq!(total, 81166799);
    }
}
