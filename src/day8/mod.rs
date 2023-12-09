#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        match s.trim() {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("invalid instruction: {s}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Network(BTreeMap<Id, Node>);

impl Network {
    fn parse(s: &str) -> Self {
        let mut nodes = BTreeMap::new();
        for line in s.trim().lines() {
            let node = Node::parse(line);
            nodes.insert(node.id, node);
        }
        Self(nodes)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    id: Id,
    left: Id,
    right: Id,
}

impl Node {
    fn new(id: Id, left: Id, right: Id) -> Self {
        Self { id, left, right }
    }

    fn is_end(&self) -> bool {
        self.id.is_end()
    }

    fn parse(s: &str) -> Self {
        let (id, pair) = s
            .trim()
            .split_once("=")
            .expect(&format!("expected `=` (s: {s}"));
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
    fn instruction_parse() {
        let expect = Instruction::Left;
        let result = Instruction::parse("L");
        assert_eq!(result, expect);

        let expect = Instruction::Right;
        let result = Instruction::parse("R");
        assert_eq!(result, expect);
    }

    #[test]
    fn network_parse() {
        let input = "
        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        ";
        let expect = Network(
            vec![
                (Id::parse("AAA"), Node::parse("AAA = (BBB, BBB)")),
                (Id::parse("BBB"), Node::parse("BBB = (AAA, ZZZ)")),
                (Id::parse("ZZZ"), Node::parse("ZZZ = (ZZZ, ZZZ)")),
            ]
            .into_iter()
            .collect(),
        );
        let result = Network::parse(input);
        assert_eq!(result, expect);
    }

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
