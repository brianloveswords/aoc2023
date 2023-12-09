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

struct Node {
    id: Id,
    left: Box<Node>,
    right: Box<Node>,
}

impl Node {
    fn is_end(&self) -> bool {
        self.id.is_end()
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
}
