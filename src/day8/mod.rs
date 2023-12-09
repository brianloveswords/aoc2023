#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Id(char, char, char);

impl Id {
    fn is_end(&self) -> bool {
        self.0 == 'Z' && self.1 == 'Z' && self.2 == 'Z'
    }

    fn parse(s: &str) -> Self {
        let mut chars = s.trim().chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        let c = chars.next().unwrap();
        assert!(chars.next().is_none(), "expected end of string");
        Self(a, b, c)
    }
}

struct Network(BTreeMap<Id, Node>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    id: Id,
    left: Id,
    right: Id,
}

impl Node {
    fn is_end(&self) -> bool {
        self.id.is_end()
    }

    fn parse(s: &str) -> Self {
        let (id, pair) = s.split_once("=").expect(&format!("expected `=` (s: {s}"));
        let pair = pair
            .trim()
            .strip_prefix('(')
            .expect(&format!("expected `(` (s: {s}"))
            .strip_suffix(')')
            .expect(&format!("expected `)` (s: {s}"));
        let (left, right) = pair
            .trim()
            .split_once(",")
            .expect(&format!("expected `,` (s: {s}"));

        let id = Id::parse(id);
        let left = Id::parse(left);
        let right = Id::parse(right);
        Self { id, left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_parse() {
        let expect = Id('A', 'A', 'A');
        let result = Id::parse("AAA");
        assert_eq!(result, expect);
    }

    #[test]
    fn node_parse() {
        let expect = Node {
            id: Id::parse("AAA"),
            left: Id::parse("BBB"),
            right: Id::parse("ZZZ"),
        };
        let result = Node::parse("AAA = (BBB, ZZZ)");
        assert_eq!(result, expect);
    }
}
