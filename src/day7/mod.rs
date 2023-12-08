#![allow(dead_code)]

use std::collections::BTreeMap;

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

    fn cards(&self) -> [Card; 5] {
        [self.first, self.second, self.third, self.fourth, self.fifth]
    }

    fn get_type(&self) -> HandType {
        let mut seen: BTreeMap<Card, usize> = BTreeMap::new();
        for card in self.cards() {
            *seen.entry(card).or_default() += 1;
        }

        let mut count: BTreeMap<usize, usize> = BTreeMap::new();
        for (_, n) in seen {
            *count.entry(n).or_default() += 1;
        }

        if count.get(&5).is_some() {
            return HandType::FiveOfAKind;
        }

        if count.get(&4).is_some() {
            return HandType::FourOfAKind;
        }

        if count.get(&3).is_some() && count.get(&2).is_some() {
            return HandType::FullHouse;
        }

        if count.get(&3).is_some() {
            return HandType::ThreeOfAKind;
        }

        if count.get(&2).unwrap_or(&0) == &2 {
            return HandType::TwoPair;
        }

        if count.get(&2).is_some() {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    Jack,
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
            'J' => Self::Jack,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CardTable {
    hands: Vec<HandBet>,
}

impl CardTable {
    fn parse(s: &str) -> Self {
        let hands = s.lines().map(HandBet::parse).collect();
        Self { hands }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_get_type() {
        let hand = Hand::parse("32T3K");
        let result = hand.get_type();
        let expected = HandType::OnePair;
        assert_eq!(result, expected);
    }

    #[test]
    fn card_table_parse() {
        let result = CardTable::parse("32T3K  765\nT55J5 684");
        let expected = CardTable {
            hands: vec![
                HandBet {
                    hand: Hand {
                        first: Card::Three,
                        second: Card::Two,
                        third: Card::Ten,
                        fourth: Card::Three,
                        fifth: Card::King,
                    },
                    bet: Bet(765),
                },
                HandBet {
                    hand: Hand {
                        first: Card::Ten,
                        second: Card::Five,
                        third: Card::Five,
                        fourth: Card::Jack,
                        fifth: Card::Five,
                    },
                    bet: Bet(684),
                },
            ],
        };
        assert_eq!(result, expected);
    }

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
