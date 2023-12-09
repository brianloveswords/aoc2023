#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Map {
    instructions: Instructions,
    network: Network,
}

impl Map {
    fn parse(s: &str) -> Self {
        let (instructions, network) = s
            .split_once("\n\n")
            .expect("invalid input: no double blank line");

        Self {
            instructions: Instructions::parse(instructions),
            network: Network::parse(network),
        }
    }

    fn navigate(self) -> usize {
        self.network.apply_instructions(self.instructions)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Instructions {
    data: Vec<Instruction>,
    index: usize,
    steps_taken: usize,
}

impl Instructions {
    fn parse(s: &str) -> Self {
        Self {
            data: Instruction::parse_all(s),
            index: 0,
            steps_taken: 0,
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn steps_taken(&self) -> usize {
        self.steps_taken
    }

    fn step(&mut self) -> Instruction {
        self.next().unwrap()
    }
}

impl Iterator for Instructions {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.data.get(self.index).copied();
        self.index += 1;
        if self.index == self.data.len() {
            self.index = 0;
        }
        self.steps_taken += 1;
        debug_assert!(
            self.steps_taken <= 1_000_000_000,
            "probably an infinite loop"
        );
        result
    }
}

impl Instruction {
    fn parse(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid instruction: {c}"),
        }
    }

    fn parse_all(s: &str) -> Vec<Self> {
        s.trim().chars().map(Self::parse).collect()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Id(char, char, char);

impl Id {
    fn start_node() -> Self {
        Self('A', 'A', 'A')
    }

    fn end_node() -> Self {
        Self('Z', 'Z', 'Z')
    }

    fn is_end(&self) -> bool {
        *self == Self::end_node()
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

    fn apply_instructions(&self, instructions: Instructions) -> usize {
        let mut current = self.0.get(&Id::start_node()).expect("no start node");
        for (step, instruction) in instructions.enumerate() {
            let next_id = current.apply(instruction);

            current = self
                .0
                .get(&next_id)
                .expect(&format!("no next node: {next_id:?}"));

            if current.is_end() {
                return step + 1;
            }
        }
        0 // instructions empty
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

    fn apply(&self, instruction: Instruction) -> Id {
        match instruction {
            Instruction::Left => self.left,
            Instruction::Right => self.right,
        }
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
    fn map_nagivate() {
        let input = "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ";

        let map = Map::parse(input);
        let result = map.navigate();
        let expect = 6;
        assert_eq!(result, expect);
    }

    #[test]
    fn map_parse() {
        let input = "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ";

        let result = Map::parse(input);
        let expect = Map {
            instructions: Instructions::parse("LLR"),
            network: Network::parse(
                "
                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)
            ",
            ),
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn network_apply_instructions() {
        let network = "
            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ";
        let network = Network::parse(network);
        let instructions = Instructions::parse("LLR");
        let result = network.apply_instructions(instructions);
        let expect = 6;
        assert_eq!(result, expect);
    }

    #[test]
    fn instructions_parse() {
        let mut instructions = Instructions::parse("LLR");

        assert_eq!(instructions.size(), 3);

        assert_eq!(instructions.step(), Instruction::Left);
        assert_eq!(instructions.step(), Instruction::Left);
        assert_eq!(instructions.step(), Instruction::Right);
        assert_eq!(instructions.step(), Instruction::Left);
        assert_eq!(instructions.step(), Instruction::Left);
        assert_eq!(instructions.step(), Instruction::Right);

        assert_eq!(instructions.steps_taken(), 6);
    }

    #[test]
    fn instruction_parse() {
        let expect = Instruction::Left;
        let result = Instruction::parse('L');
        assert_eq!(result, expect);

        let expect = Instruction::Right;
        let result = Instruction::parse('R');
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
