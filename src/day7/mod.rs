#![allow(dead_code)]

use std::{cmp::Ordering, collections::BTreeMap};

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day7.txt");
pub const REAL: &str = include_str!("../../inputs/real/day7.txt");

pub fn part1(s: &str) -> usize {
    CardTable::parse(s).winnings()
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
        let ord = self.get_type().partial_cmp(&other.get_type());
        if ord != Some(Ordering::Equal) {
            return ord;
        }

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

enum Jokers {
    _____,
    J____,
    JJ___,
    JJJ__,
    JJJJ_,
    JJJJJ,
}

impl Jokers {
    fn from(n: usize) -> Self {
        match n {
            0 => Self::_____,
            1 => Self::J____,
            2 => Self::JJ___,
            3 => Self::JJJ__,
            4 => Self::JJJJ_,
            5 => Self::JJJJJ,
            _ => panic!("invalid number of jokers: {n}"),
        }
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

    fn become_the_joker(&mut self) -> Self {
        self.first.jokerify();
        self.second.jokerify();
        self.third.jokerify();
        self.fourth.jokerify();
        self.fifth.jokerify();
        *self
    }

    fn cards(&self) -> [Card; 5] {
        [self.first, self.second, self.third, self.fourth, self.fifth]
    }

    fn get_type(&self) -> HandType {
        let mut jokers = 0;
        let mut seen: BTreeMap<Card, usize> = BTreeMap::new();
        for card in self.cards() {
            if card.is_joker() {
                jokers += 1;
            }
            *seen.entry(card).or_default() += 1;
        }

        let mut count: BTreeMap<usize, usize> = BTreeMap::new();
        for (_, n) in seen {
            *count.entry(n).or_default() += 1;
        }

        let jokers = Jokers::from(jokers);

        if count.get(&5).is_some() {
            return HandType::FiveOfAKind;
        }

        use Jokers::*;

        if count.get(&4).is_some() {
            return match jokers {
                _____ => HandType::FourOfAKind,
                J____ => HandType::FiveOfAKind,
                JJ___ => panic!("impossible hand: 2 jokers, 4 of a kind"),
                JJJ__ => panic!("impossible hand: 3 jokers, 4 of a kind"),
                JJJJ_ => HandType::FiveOfAKind,
                JJJJJ => panic!("impossible hand: 5 jokers, 4 of a kind"),
            };
        }

        if count.get(&3).is_some() && count.get(&2).is_some() {
            return match jokers {
                _____ => HandType::FullHouse,
                J____ => panic!("impossible hand: 1 joker, full house"),
                JJ___ => HandType::FiveOfAKind,
                JJJ__ => HandType::FiveOfAKind,
                JJJJ_ => panic!("impossible hand: 4 jokers, full house"),
                JJJJJ => panic!("impossible hand: 5 jokers, full house"),
            };
        }

        if count.get(&3).is_some() {
            return match jokers {
                _____ => HandType::ThreeOfAKind,
                J____ => HandType::FourOfAKind,
                JJ___ => panic!("impossible hand: 2 jokers, 3 of a kind"),
                JJJ__ => HandType::FourOfAKind,
                JJJJ_ => HandType::FiveOfAKind,
                JJJJJ => panic!("impossible hand: 5 jokers, 3 of a kind"),
            };
        }

        if count.get(&2).unwrap_or(&0) == &2 {
            return match jokers {
                _____ => HandType::TwoPair,
                J____ => HandType::FullHouse,
                JJ___ => HandType::FourOfAKind,
                JJJ__ => panic!("impossible hand: 3 jokers, 2 pair"),
                JJJJ_ => panic!("impossible hand: 4 jokers, 2 pair"),
                JJJJJ => panic!("impossible hand: 5 jokers, 2 pair"),
            };
        }

        if count.get(&2).is_some() {
            return match jokers {
                _____ => HandType::OnePair,
                J____ => HandType::ThreeOfAKind,
                JJ___ => HandType::ThreeOfAKind,
                JJJ__ => panic!("impossible hand: 3 jokers, 1 pair"),
                JJJJ_ => panic!("impossible hand: 4 jokers, 1 pair"),
                JJJJJ => panic!("impossible hand: 5 jokers, 1 pair"),
            };
        }

        return match jokers {
            _____ => HandType::HighCard,
            J____ => HandType::OnePair,
            JJ___ => panic!("impossible hand: 2 jokers, high card"),
            JJJ__ => panic!("impossible hand: 3 jokers, high card"),
            JJJJ_ => panic!("impossible hand: 4 jokers, high card"),
            JJJJJ => panic!("impossible hand: 5 jokers, high card"),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
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

    fn is_joker(&self) -> bool {
        *self == Self::Joker
    }

    fn jokerify(&mut self) {
        if *self == Self::Jack {
            *self = Self::Joker;
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

impl PartialOrd for HandBet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for HandBet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
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

    fn winnings(&mut self) -> usize {
        let mut rank = self.hands.len();
        self.hands.sort();
        self.hands.reverse();

        let mut winnings = 0;
        for hand in &self.hands {
            winnings += hand.winnings(rank);
            rank -= 1;
        }

        winnings
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

    #[test]
    fn card_table_winnings() {
        let mut table = CardTable::parse(EXAMPLE);
        let result = table.winnings();
        let expected = 6440;
        assert_eq!(result, expected);
    }

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
        assert!(Card::Joker < Card::Two);
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
    fn hand_get_joker_type_five_of_a_kind() {
        let hand = Hand::parse("TTJJJ").become_the_joker();
        let result = hand.get_type();
        let expected = HandType::FiveOfAKind;
        assert_eq!(result, expected);
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
