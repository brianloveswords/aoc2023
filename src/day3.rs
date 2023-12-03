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

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Number {
        number: u32,
        line: usize,
        start: usize,
        end: usize,
    },
    Symbol {
        symbol: char,
        line: usize,
        position: usize,
    },
}

impl Token {
    fn line(&self) -> usize {
        match self {
            Token::Number { line, .. } => *line,
            Token::Symbol { line, .. } => *line,
        }
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
        self.next_while(|c| c == '.' || c == '\n');

        // keep track of starting position. we won't cross a line boundary.
        let line = self.line;
        let start_column = self.column;

        // try to parse a number
        if let Some(number) = self.next_while(|c| c.is_digit(10)) {
            let end_column = self.column;
            let number = number
                .parse()
                .expect("filtered for digits, should have a number");
            return Some(Token::Number {
                number,
                line,
                start: start_column,
                end: end_column,
            });
        }

        // everything else is a symbol
        let symbol = self.next()?;
        Some(Token::Symbol {
            symbol,
            line,
            position: start_column,
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_parse_token() {
        let lines = "467..114..\n&..35..633";
        let mut parser = SchematicParser::new(lines);
        let token = parser.parse_token();
        assert_eq!(
            token,
            Some(Token::Number {
                number: 467,
                line: 1,
                start: 1,
                end: 4,
            })
        );

        let token = parser.parse_token();
        assert_eq!(
            token,
            Some(Token::Number {
                number: 114,
                line: 1,
                start: 6,
                end: 9,
            })
        );

        let token = parser.parse_token();
        assert_eq!(
            token,
            Some(Token::Symbol {
                symbol: '&',
                line: 2,
                position: 1,
            })
        );

        let token = parser.parse_token();
        assert_eq!(
            token,
            Some(Token::Number {
                number: 35,
                line: 2,
                start: 4,
                end: 6,
            })
        );
    }

    #[test]
    fn test_find_neighbors() {
        let lines = "467..114..\n&..35..633";
        let mut parser = SchematicParser::new(lines);
        let mut line_map: BTreeMap<usize, Vec<Token>> = BTreeMap::new();

        while let Some(token) = parser.parse_token() {
            let line = line_map.entry(token.line()).or_default();
            line.push(token);
        }

        println!("{:#?}", line_map);
    }
}
