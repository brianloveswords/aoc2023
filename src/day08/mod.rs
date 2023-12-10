#![allow(dead_code)]

use crate::util;
use std::{collections::BTreeMap, fmt};

pub const EXAMPLE: &str = include_str!("../../inputs/examples/day08.txt");
pub const REAL: &str = include_str!("../../inputs/real/day08.txt");

pub fn part1(s: &str) -> usize {
    Map::parse(s).navigate()
}

pub fn part2(s: &str) -> usize {
    Map::parse(s).navigate_ghosts()
}

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

    fn navigate_ghosts(self) -> usize {
        self.network.apply_ghost_instructions(self.instructions)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

    #[cfg(test)]
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

    fn is_ghost_end(&self) -> bool {
        self.2 == 'Z'
    }

    fn is_ghost_start(&self) -> bool {
        self.2 == 'A'
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

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(a, b, c) = self;
        write!(f, "{}{}{}", a, b, c)
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

    fn ghost_start_ids(&self) -> impl Iterator<Item = &Id> {
        self.0.keys().filter(|id| id.is_ghost_start())
    }

    fn apply_instructions(&self, instructions: Instructions) -> usize {
        let mut step = 0;
        let mut current = self.0.get(&Id::start_node()).expect("no start node");
        for instruction in instructions {
            step += 1;
            let next_id = current.apply(instruction);

            current = self
                .0
                .get(&next_id)
                .expect(&format!("no next node: {next_id}"));

            if current.is_end() {
                break;
            }
        }
        step
    }

    fn apply_ghost_instructions(&self, instructions: Instructions) -> usize {
        // full simulation is gonna be too slow, so we gotta use math
        // once we figure out how many steps it is for each ghost, we can
        // find the least common multiple of the set to figure out when
        // they'll all be on the end node at the same time

        self.ghost_start_ids()
            .map(|id| self.0.get(id).expect(&format!("no start node: {id}")))
            .map(|node| {
                let mut step = 0;
                let mut current = node;
                let map = &self.0;

                for instruction in instructions.clone() {
                    step += 1;
                    let next_id = current.apply(instruction);

                    current = map
                        .get(&next_id)
                        .expect(&format!("no next node: {next_id}"));

                    if current.is_ghost_end() {
                        let end_id = current.id;
                        println!("found end at step {end_id}, {step} steps");
                        break;
                    }
                }
                step
            })
            .fold(1, util::least_common_multiple)
    }
}

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

    fn is_ghost_end(&self) -> bool {
        self.id.is_ghost_end()
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
    use proptest::prelude::*;

    #[test]
    fn network_apply_ghost_instructions() {
        let map = "
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ";
        let map = Map::parse(map);
        let result = map.navigate_ghosts();
        assert_eq!(result, 6);
    }

    #[test]
    fn network_start_nodes() {
        let network = "
            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ";
        let network = Network::parse(network);
        assert_eq!(
            network.ghost_start_ids().collect::<Vec<_>>(),
            vec![&Id::parse("11A"), &Id::parse("22A")]
        );
    }

    proptest! {
        #[test]
        fn id_is_ghost_start_ends_in_a(a: char, b: char) {
            let id = Id(a, b, 'A');
            assert_eq!(id.is_ghost_start(), true);

        }
        #[test]
        fn id_is_ghost_end_ends_in_z(a: char, b: char) {
            let id = Id(a, b, 'Z');
            assert_eq!(id.is_ghost_end(), true);

        }
    }

    #[test]
    fn id_is_ghost_start() {
        let id = Id('Z', 'Z', 'A');
        assert_eq!(id.is_ghost_start(), true);

        let id = Id('Z', 'A', 'Z');
        assert_eq!(id.is_ghost_start(), false);
    }

    #[test]
    fn id_is_ghost_end() {
        let id = Id('J', 'J', 'Z');
        assert_eq!(id.is_ghost_end(), true);

        let id = Id('Z', 'Z', 'A');
        assert_eq!(id.is_ghost_end(), false);
    }

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
