use std::{cmp::Ordering, collections::BTreeMap};

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day7.txt");
pub const REAL: &str = include_str!("../../inputs/real/day7.txt");

pub fn part1(s: &str) -> usize {
    CardTable::parse(s).winnings()
}

pub fn part2(s: &str) -> usize {
    CardTable::parse_with_jokers(s).winnings()
}

// this is an odd representation but it makes visually
// parsing some of the logic easier when trying to figure
// out if a hand type is valid against a given set of jokers
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

#[derive(Debug, Clone)]
struct MatchCounter(BTreeMap<usize, usize>);

impl MatchCounter {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn add(&mut self, n: usize) {
        *self.0.entry(n).or_default() += 1;
    }

    fn check(&self, n: usize) -> bool {
        self.0.get(&n).map(|&n| n == 0).unwrap_or(false)
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Hash, Clone)]
pub struct Hand {
    first: Card,
    second: Card,
    third: Card,
    fourth: Card,
    fifth: Card,
    type_: Option<HandType>,
}

impl Hand {
    pub fn new(first: Card, second: Card, third: Card, fourth: Card, fifth: Card) -> Self {
        Self {
            first,
            second,
            third,
            fourth,
            fifth,
            type_: None,
        }
    }

    pub fn parse_no_cache(s: &str) -> Self {
        let mut cards = s.chars();
        let first = Card::parse(cards.next().unwrap());
        let second = Card::parse(cards.next().unwrap());
        let third = Card::parse(cards.next().unwrap());
        let fourth = Card::parse(cards.next().unwrap());
        let fifth = Card::parse(cards.next().unwrap());

        if let Some(c) = cards.next() {
            panic!("too many cards, found trailing {c}");
        }

        Self::new(first, second, third, fourth, fifth)
    }

    pub fn parse(s: &str) -> Self {
        let mut this = Self::parse_no_cache(s);
        this.pre_cache_type();
        this
    }

    pub fn parse_with_jokers(s: &str) -> Self {
        let mut this = Self::parse_no_cache(s);
        this.jokerify();
        this
    }

    fn jokerify(&mut self) {
        self.type_ = None;

        self.first.jokerify();
        self.second.jokerify();
        self.third.jokerify();
        self.fourth.jokerify();
        self.fifth.jokerify();

        self.pre_cache_type();
    }

    fn cards(&self) -> [Card; 5] {
        [self.first, self.second, self.third, self.fourth, self.fifth]
    }

    fn pre_cache_type(&mut self) {
        self.type_ = Some(self.get_type());
    }

    fn get_type(&self) -> HandType {
        if let Some(type_) = self.type_ {
            return type_;
        }

        // keep track of how many times we see each card. we'll need
        // to be able to pull out jokers specifically, but otherwise
        // the count will be more important than the card itself.
        let mut seen: BTreeMap<Card, usize> = BTreeMap::new();
        for card in self.cards() {
            *seen.entry(card).or_default() += 1;
        }

        let jokers = {
            let n = *seen.entry(Card::Joker).or_default();
            Jokers::from(n)
        };

        // we want a map that answers "how many times do we see `n`
        // matching cards?", so if we had a FiveOfAKind, we'd have
        // a map with a single entry of (5, 1). if we had a FullHouse,
        // we'd have a map with two entries of (2, 1) and (3, 1).
        let mut count: BTreeMap<usize, usize> = BTreeMap::new();
        for (_, n) in seen {
            *count.entry(n).or_default() += 1;
        }

        use HandType::*;
        use Jokers::*;

        if count.get(&1) == Some(&5) {
            return match jokers {
                _____ => HighCard,
                J____ => OnePair,
                JJ___ => panic!("impossible: 2 jokers, HighCard"),
                JJJ__ => panic!("impossible: 3 jokers, HighCard"),
                JJJJ_ => panic!("impossible: 4 jokers, HighCard"),
                JJJJJ => panic!("impossible: 5 jokers, HighCard"),
            };
        }

        if count.get(&2) == Some(&1) && count.get(&1) == Some(&3) {
            return match jokers {
                _____ => OnePair,
                J____ => ThreeOfAKind,
                JJ___ => ThreeOfAKind,
                JJJ__ => panic!("impossible: 3 jokers, OnePair"),
                JJJJ_ => panic!("impossible: 4 jokers, OnePair"),
                JJJJJ => panic!("impossible: 5 jokers, OnePair"),
            };
        }

        if count.get(&2) == Some(&2) && count.get(&1) == Some(&1) {
            return match jokers {
                _____ => TwoPair,
                J____ => FullHouse,
                JJ___ => FourOfAKind,
                JJJ__ => panic!("impossible: 3 jokers, TwoPair"),
                JJJJ_ => panic!("impossible: 4 jokers, TwoPair"),
                JJJJJ => panic!("impossible: 5 jokers, TwoPair"),
            };
        }

        if count.get(&3) == Some(&1) && count.get(&1) == Some(&2) {
            return match jokers {
                _____ => ThreeOfAKind,
                J____ => FourOfAKind,
                JJ___ => panic!("impossible: 2 jokers, ThreeOfAKind"),
                JJJ__ => FourOfAKind,
                JJJJ_ => FiveOfAKind,
                JJJJJ => panic!("impossible: 5 jokers, ThreeOfAKind"),
            };
        }

        if count.get(&3) == Some(&1) && count.get(&2) == Some(&1) {
            return match jokers {
                _____ => FullHouse,
                J____ => panic!("impossible: 1 joker, FullHouse"),
                JJ___ => FiveOfAKind,
                JJJ__ => FiveOfAKind,
                JJJJ_ => panic!("impossible: 4 jokers, FullHouse"),
                JJJJJ => panic!("impossible: 5 jokers, FullHouse"),
            };
        }

        if count.get(&4) == Some(&1) && count.get(&1) == Some(&1) {
            return match jokers {
                _____ => FourOfAKind,
                J____ => FiveOfAKind,
                JJ___ => panic!("impossible: 2 jokers, FourOfAKind"),
                JJJ__ => panic!("impossible: 3 jokers, FourOfAKind"),
                JJJJ_ => FiveOfAKind,
                JJJJJ => panic!("impossible: 5 jokers, FourOfAKind"),
            };
        }

        if count.get(&5) == Some(&1) {
            return FiveOfAKind;
        }

        unreachable!("impossible: {:?}", self);
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    fn jokerify(&mut self) {
        self.hand.jokerify();
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

    fn parse_with_jokers(s: &str) -> Self {
        let mut table = Self::parse(s);
        table.jokerify();
        table
    }

    fn jokerify(&mut self) {
        for hand in &mut self.hands {
            hand.jokerify();
        }
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
    fn part1_real() {
        assert_eq!(part1(REAL), 249638405);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(REAL), 249776650);
    }

    #[test]
    fn card_table_winnings_jokers() {
        let mut table = CardTable::parse_with_jokers(EXAMPLE);
        let result = table.winnings();
        let expected = 5905;
        assert_eq!(result, expected);
    }

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
        let mut hand = Hand::parse("TTJJJ");
        hand.jokerify();

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
}
