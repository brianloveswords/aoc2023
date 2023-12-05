use std::collections::{BTreeMap, HashSet, VecDeque};

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day4.txt");
pub const REAL: &str = include_str!("../../inputs/real/day4.txt");

pub fn part1(s: &str) -> usize {
    let winners: Vec<_> = s
        .lines()
        .map(Card::parse)
        .map(Result::unwrap)
        .map(|c| c.score_with_winners())
        .collect();

    winners.iter().map(|(score, _)| score).sum()
}

pub fn part2(s: &str) -> usize {
    let table = CardTable::parse(s).expect("invalid input");
    table.process_scratchcards_with_math()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CardTable {
    cards: BTreeMap<CardId, Card>,
    results: BTreeMap<CardId, Vec<CardId>>,
    pending: VecDeque<CardId>,
    processed: usize,
}

impl CardTable {
    fn new(cards: impl Iterator<Item = Card>) -> Self {
        let mut card_map = BTreeMap::new();
        let mut pending_stack = VecDeque::new();

        for card in cards {
            card_map.insert(card.id, card.clone());
            pending_stack.push_back(card.id);
        }

        Self {
            cards: card_map,
            pending: pending_stack,
            results: BTreeMap::new(),
            processed: 0,
        }
    }

    #[allow(dead_code)]
    fn process_scratchcards(mut self) -> usize {
        let mut n = 0;
        while self.step().is_some() {
            n += 1;
            debug_assert!(n < 10_000_000, "infinite loop detected: {n} > 10,000,000")
        }
        self.processed
    }

    // note: cards must be in order and 1-indexed
    fn process_scratchcards_with_math(self) -> usize {
        let mut results: BTreeMap<&CardId, HashSet<CardId>> = BTreeMap::new();

        for id in &self.pending {
            let card = self.cards.get(&id).unwrap();
            let ids: HashSet<_> = card.id.next_ids(card.winners().len()).into_iter().collect();
            results.insert(id, ids);
        }

        let mut total = 0;
        let mut counts: BTreeMap<&CardId, usize> = BTreeMap::new();

        for id in &self.pending {
            let count = results
                .iter()
                .take_while(|(k, _)| k.0 < id.0)
                .filter(|(_, ids)| ids.contains(id))
                .map(|(k, _)| counts.get(k).expect("should have been inserted"))
                .sum::<usize>()
                + 1;

            counts.insert(id, count);
            total += count;
        }

        total
    }

    pub fn step(&mut self) -> Option<usize> {
        let id = match self.pending.pop_front() {
            Some(card) => card,
            None => return None,
        };

        self.processed += 1;

        let card = self.cards.get(&id).unwrap();

        if let Some(ids) = self.results.get(&id) {
            self.pending.extend(ids);
            return Some(ids.len());
        }

        let copies = card.id.next_ids(card.winners().len());
        self.pending.extend(&copies);
        self.results.insert(card.id, copies.clone());
        Some(copies.len())
    }

    fn parse(s: &str) -> Result<Self, String> {
        let cards = s
            .trim()
            .lines()
            .map(Card::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(cards.into_iter()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Card {
    id: CardId,
    generation: usize,
    goal: NumberSet,
    hand: NumberSet,
}

impl Card {
    fn winners(&self) -> NumberSet {
        self.goal.find_winners(&self.hand)
    }

    fn score_with_winners(&self) -> (usize, NumberSet) {
        let winners = self.winners();
        let len = winners.len();
        if len == 0 {
            return (0, NumberSet::empty());
        }
        (2usize.pow(winners.len() as u32 - 1), winners)
    }

    fn parse(s: &str) -> Result<Self, String> {
        let (id, numbers) = s
            .split_once(':')
            .ok_or_else(|| "failed to parse card: no ':' found")?;

        let id = CardId::parse(id)?;

        let (goal, hand) = numbers
            .split_once('|')
            .ok_or_else(|| format!("failed to parse card: no '|' found"))?;

        let goal = NumberSet::parse(goal)?;
        let hand = NumberSet::parse(hand)?;

        Ok(Self {
            id,
            goal,
            hand,
            generation: 1,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct CardId(usize);

impl CardId {
    fn new(n: usize) -> Self {
        Self(n)
    }

    fn next_ids(&self, n: usize) -> Vec<Self> {
        (self.0 + 1..=self.0 + n).map(Self::new).collect()
    }

    fn parse(s: &str) -> Result<Self, String> {
        s.trim()
            .strip_prefix("Card ")
            .ok_or_else(|| format!("failed to parse card id: not a card '{s}'"))
            .and_then(Self::parse_inner)
    }

    fn parse_inner(s: &str) -> Result<Self, String> {
        s.trim()
            .parse::<usize>()
            .map(Self::new)
            .map_err(|e| format!("failed to parse card id: {e}"))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Number(usize);

impl Number {
    fn new(n: usize) -> Self {
        Self(n)
    }

    fn parse(s: &str) -> Result<Self, String> {
        s.trim()
            .parse::<usize>()
            .map(Self::new)
            .map_err(|e| format!("failed to parse number: {e}"))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct NumberSet(HashSet<Number>);

impl NumberSet {
    fn new(set: HashSet<Number>) -> Self {
        Self(set)
    }

    fn empty() -> Self {
        Self::new(HashSet::new())
    }

    fn parse(s: &str) -> Result<Self, String> {
        let set: Result<HashSet<_>, _> = s.trim().split_whitespace().map(Number::parse).collect();
        Ok(Self::new(set?))
    }

    #[cfg(test)]
    fn must_parse(s: &str) -> Self {
        Self::parse(s).expect("valid input")
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn find_winners(&self, other: &Self) -> Self {
        Self::new(self.0.intersection(&other.0).copied().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(REAL), 23441);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 30);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(REAL), 5923918);
    }

    #[test]
    fn card_ids_next_ids() {
        assert_eq!(CardId(1).next_ids(3), vec![CardId(2), CardId(3), CardId(4)]);
    }

    #[test]
    fn card_table_step() {
        let mut table = CardTable::parse(EXAMPLE).expect("valid input");

        let step = table.step().expect("step");
        assert_eq!(step, 4);

        let step = table.step().expect("step");
        assert_eq!(step, 2);

        assert_eq!(table.processed, 2);
    }

    #[test]
    fn card_table_parse() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 18 25 71 33 19 | 71 33 19 25  6 18  3  4
        ";

        let table = CardTable::parse(input).expect("valid input");

        assert_eq!(table.cards.len(), 2);
        assert_eq!(table.results.len(), 0);
        assert_eq!(table.pending.len(), 2);
        assert_eq!(table.processed, 0);
    }

    #[test]
    fn card_score() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::parse(input).expect("valid input");
        let (score, winners) = card.score_with_winners();
        assert_eq!(winners, NumberSet::must_parse("48 83 17 86"));
        assert_eq!(winners.len(), 4);
        assert_eq!(score, 8);
    }

    #[test]
    fn parse_card() {
        let input = "Card  109 : 41     48 83  86  17 | 20  1 82  48   41 17 9";
        let card = Card::parse(input).expect("valid input");
        assert_eq!(card.id, CardId::new(109));
        assert_eq!(card.goal.len(), 5);
        assert_eq!(card.hand.len(), 7);
        assert_eq!(card.goal, NumberSet::must_parse("41 48 83 86 17"));
        assert_eq!(card.hand, NumberSet::must_parse("20  1 82 48 41 17  9"));
    }

    #[test]
    fn parse_card_id() {
        let input = "Card  109 ";
        let card_id = CardId::parse(input).expect("valid input");
        assert_eq!(card_id, CardId::new(109));
    }

    #[test]
    fn parse_numbers() {
        let input = "41     48 83  86  17  ";
        let numbers = NumberSet::parse(input).expect("valid input");
        let expected: HashSet<_> = vec![41, 48, 83, 86, 17]
            .into_iter()
            .map(Number::new)
            .collect();
        assert_eq!(numbers.len(), 5);
        assert_eq!(numbers, NumberSet(expected));
    }
}
