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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Bet(usize);

impl Bet {
    fn parse(s: &str) -> Self {
        Self(s.parse().expect(&format!("invalid bet {s}")))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct HandBet {
    hand: Hand,
    bet: Bet,
}

impl HandBet {
    fn parse(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let hand = Hand::parse(parts.next().expect("missing hand"));
        let bet = Bet::parse(parts.next().expect("missing bet"));
        Self { hand, bet }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_bet_parse() {
        let result = HandBet::parse("32T3K  765");
        let expected = HandBet {
            hand: Hand {
                first: Card::Three,
                second: Card::Two,
                third: Card::Ten,
                fourth: Card::Three,
                fifth: Card::King,
            },
            bet: Bet(765),
        };
        assert_eq!(result, expected);
    }

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
