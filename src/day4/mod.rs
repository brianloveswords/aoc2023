#![allow(dead_code)]

use std::collections::HashSet;

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day4.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Card {
    id: CardId,
    goal: NumberSet,
    hand: NumberSet,
}

impl Card {
    fn winners(&self) -> NumberSet {
        let (_, winners) = self.score_with_winners();
        winners
    }

    fn score(&self) -> usize {
        let (score, _) = self.score_with_winners();
        score
    }

    fn score_with_winners(&self) -> (usize, NumberSet) {
        let winners = self.goal.find_winners(&self.hand);
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

        Ok(Self { id, goal, hand })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CardId(usize);

impl CardId {
    fn new(n: usize) -> Self {
        Self(n)
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

    fn parse(s: &str) -> Result<Self, String> {
        let set: Result<HashSet<_>, _> = s.trim().split_whitespace().map(Number::parse).collect();
        Ok(Self::new(set?))
    }

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

pub fn part1(_s: &str) -> usize {
    todo!("write this")
}

#[cfg(test)]
mod tests {
    use super::*;

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
        println!("{:?}", card);
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
