/*

--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

struct Calibration(u32);

impl Calibration {
    fn combine(self, other: &Calibration) -> Calibration {
        Calibration(self.0 + other.0)
    }

    fn parse(s: &str) -> Self {
        // eprintln!("Parsing: {}", s);

        let digits: Vec<char> = s.chars().into_iter().filter(|c| c.is_digit(10)).collect();
        // eprintln!("Digits: {:?}", digits);

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();

        let value = format!("{}{}", first, last).parse::<u32>().unwrap();
        // eprintln!("Value: {}", value);

        Self(value)
    }

    fn parse_lines(s: &str) -> Vec<Self> {
        s.trim().lines().map(|l| Self::parse(l)).collect()
    }

    fn empty() -> Self {
        Self(0)
    }
}

pub struct DigitParser {
    input: String,
}

impl DigitParser {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn parse(&mut self) -> String {
        // result will be at most the length of the input, but possibly shorter
        let mut result = String::with_capacity(self.input.len());
        let mut buffer = String::with_capacity(5);

        for c in self.input.chars() {
            if !c.is_alphabetic() {
                result.push_str(&buffer);
                buffer.clear();

                result.push(c);
                continue;
            }
            buffer.push(c);
            buffer = replace_digit_string(&buffer);
        }

        result.push_str(&buffer);
        result
    }
}

fn replace_digit_string(s: &str) -> String {
    s.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

pub fn calibrate(s: &str) -> u32 {
    let result = Calibration::parse_lines(s)
        .into_iter()
        .fold(Calibration::empty(), |acc, c| acc.combine(&c));

    eprintln!("Calibration result: {}", result.0);
    result.0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART1: &str = include_str!("../inputs/examples/day1.part1.txt");
    const EXAMPLE_INPUT_PART2: &str = include_str!("../inputs/examples/day1.part2.txt");
    const REAL_INPUT: &str = include_str!("../inputs/real/day1.txt");

    #[test]
    fn part1_example() {
        let input = EXAMPLE_INPUT_PART1.to_string();
        let expected = 142;
        let result = calibrate(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn part1() {
        let input = REAL_INPUT.to_string();
        let result = calibrate(&input);
        println!("{result}");
    }

    #[test]
    fn part2_example() {
        let input = EXAMPLE_INPUT_PART2.to_string();
        let mut parser = DigitParser::new(input);
        let clean = parser.parse();
        eprintln!("{clean}");

        let expected = 281;
        let result = calibrate(&clean);
        assert_eq!(result, expected);
    }

    fn part2() {
        let input = REAL_INPUT.to_string();
        let mut parser = DigitParser::new(input);
        let clean = parser.parse();
        eprintln!("{clean}");

        calibrate(&clean);
    }
}
