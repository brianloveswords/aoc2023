#![allow(dead_code)]

pub fn part1() -> usize {
    todo!();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Hand {
    first: Card,
    second: Card,
    third: Card,
    fourth: Card,
    fifth: Card,
}

impl Hand {
    pub fn parse(s: &str) -> Self {
        let mut cards = s.chars();
        let first = Card::parse(cards.next().unwrap());
        let second = Card::parse(cards.next().unwrap());
        let third = Card::parse(cards.next().unwrap());
        let fourth = Card::parse(cards.next().unwrap());
        let fifth = Card::parse(cards.next().unwrap());

        if let Some(c) = cards.next() {
            panic!("too many cards, found trailing {c}");
        }

        Self {
            first,
            second,
            third,
            fourth,
            fifth,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Knight,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(s: char) -> Self {
        match s {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Knight,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("invalid card: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_parse() {
        let result = Hand::parse("32T3K");
        let expected = Hand {
            first: Card::Three,
            second: Card::Two,
            third: Card::Ten,
            fourth: Card::Three,
            fifth: Card::King,
        };
        assert_eq!(result, expected);
    }
}
