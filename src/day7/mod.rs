#![allow(dead_code)]

use std::{cmp::Ordering, collections::BTreeMap};

pub fn part1() -> usize {
    todo!();
}

#[derive(Debug, PartialEq, Eq, Ord, Hash, Clone, Copy)]
pub struct Hand {
    first: Card,
    second: Card,
    third: Card,
    fourth: Card,
    fifth: Card,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.first.cmp(&other.first);
        if ord != Ordering::Equal {
            return Some(ord);
        }

        let ord = self.second.cmp(&other.second);
        if ord != Ordering::Equal {
            return Some(ord);
        }

        let ord = self.third.cmp(&other.third);
        if ord != Ordering::Equal {
            return Some(ord);
        }

        let ord = self.fourth.cmp(&other.fourth);
        if ord != Ordering::Equal {
            return Some(ord);
        }

        let ord = self.fifth.cmp(&other.fifth);
        if ord != Ordering::Equal {
            return Some(ord);
        }

        None
    }
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

    fn winnings(&self, rank: usize) -> usize {
        self.bet.0 * rank
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn card_table_winnings() {
    //     let table = CardTable::parse("TTTTT 10\nTTTT3 10\nTTT33 10");
    //     let result = table.winnings();
    //     let expected = 6;
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn hand_ordering_greater() {
        let hand1 = Hand::parse("TTTTT");
        let hand2 = Hand::parse("TTTT3");
        let result = hand1.cmp(&hand2);
        let expected = Ordering::Greater;
        assert_eq!(result, expected);
    }

    #[test]
    fn hand_ordering_less() {
        let hand1 = Hand::parse("2AAAA");
        let hand2 = Hand::parse("A2222");
        let result = hand1.cmp(&hand2);
        let expected = Ordering::Less;
        assert_eq!(result, expected);
    }

    #[test]
    fn card_ordering() {
        assert!(Card::Two < Card::Three);
        assert!(Card::King < Card::Ace);
    }

    #[test]
    fn hand_type_ordering() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn hand_get_type_five_of_a_kind() {
        let hand = Hand::parse("TTTTT");
        let result = hand.get_type();
        let expected = HandType::FiveOfAKind;
        assert_eq!(result, expected);
    }

    #[test]
    fn hand_get_type_four_of_a_kind() {
        let hand = Hand::parse("TTTT3");
        let result = hand.get_type();
        let expected = HandType::FourOfAKind;
        assert_eq!(result, expected);
    }

    #[test]
    fn hand_get_type_full_house() {
        let hand = Hand::parse("TTT33");
        let result = hand.get_type();
        let expected = HandType::FullHouse;
        assert_eq!(result, expected);
    }

    #[test]
    fn hand_get_type_three_of_a_kind() {
        let hand = Hand::parse("TTT32");
        let result = hand.get_type();
        let expected = HandType::ThreeOfAKind;
        assert_eq!(result, expected);
    }

    #[test]
    fn hand_get_type_two_pair() {
        let hand = Hand::parse("TT332");
        let result = hand.get_type();
        let expected = HandType::TwoPair;
        assert_eq!(result, expected);
    }

    #[test]
    fn hand_get_type_one_pair() {
        let hand = Hand::parse("32T3K");
        let result = hand.get_type();
        let expected = HandType::OnePair;
        assert_eq!(result, expected);
    }

    #[test]
    fn hand_get_type_high_card() {
        let hand = Hand::parse("32TKA");
        let result = hand.get_type();
        let expected = HandType::HighCard;
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
