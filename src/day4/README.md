# day 4

## notes

### part 1

- Card
    - winning: Vec<Number>
    - in_hand: Vec<Number>


    - parse()
        - (card_number, all_numbers) = split on `:`
        - (winning_list, in_hand_list) = split on `|`
        - numbers = split on `whitespace`

### part 2
- alright so these things are tribbles now, they will duplicate based on winning numbers
- if Card 4 has 3 winners, would get copies of Cards {5, 6, 7}
- it is known that cards will not cause copies past end of table

- CardTable
    - card_map: BTreeMap<CardId, Card>
    - result_map: BTreeMap<CardId, Vec<Card>
    - pending_stack: Vec<Card>
    - processed_count: usize

    - initialize by putting everything on the pending stack

    - next()
        - pop off the pending stack
        - add to processed count
        - does it already exist in result map?
            - yes: push the result to the stack; return
            - no: continue
        - find card IDs of next cards
            - n = winners.len()
            - card_id.next_ids(n)
        - get result
            - ids.map(|id| card_map.get(id))
        - store result in result_map
        - push result to stack


#### benchmarking

trying out [hyperfine](https://github.com/sharkdp/hyperfine) for some benchmarking with `hyperfine --warmup 3`

```
Benchmark 1: target/release/main
  Time (mean ± σ):      1.729 s ±  0.015 s    [User: 1.410 s, System: 0.299 s]
  Range (min … max):    1.711 s …  1.767 s    10 runs
```

this is not bad, a twitter pal of mine who I suspect is using python got [3 seconds](https://twitter.com/haxor/status/1731721271896121632) for their solution.

for my first attempt, I chose to fully clone cards, bumping `generation` on each copy, since I figured it'd be useful for debugging—I knew it was gonna get out of hand real quick with the number of scratchcards. I also included a rich return struct for `step()` so I could inspect the state at each stage.

I have a working version and I'm trying to see how fast I can get it so I'm gonna remove all that. this lets me use `CardId`s instead of full cards in a bunch of places.

making those changes shaves off about 90% of the running time:

```
Benchmark 1: target/release/main
  Time (mean ± σ):     175.9 ms ±   0.5 ms    [User: 172.7 ms, System: 1.8 ms]
  Range (min … max):   175.1 ms … 176.9 ms    16 runs
```

out of curiosity, I wanted to how much it would hurt performance to remove the result cache:

```diff
pub fn step(&mut self) -> Option<usize> {
    let id = match self.pending.pop_front() {
        Some(card) => card,
        None => return None,
    };

    self.processed += 1;

    let card = self.cards.get(&id).unwrap();

-    if let Some(ids) = self.results.get(&id) {
-        self.pending.extend(ids);
-        return Some(ids.len());
-    }

    let copies = card.id.next_ids(card.winners().len());
    self.pending.extend(&next_ids);
-    self.results.insert(card.id, copies.clone());
    Some(copies.len())
}
```

and it destroys all the previous gains, making it 10x slower again.

```
Benchmark 1: target/release/main
  Time (mean ± σ):      1.704 s ±  0.018 s    [User: 1.692 s, System: 0.003 s]
  Range (min … max):    1.684 s …  1.748 s    10 runs
```

so around 170ms is probably close to the limit of brute forcing the result by simulating the whole thing. there might be a way to compute this without having to simulate it.

looking at the puzzle, each card produces these other cards:

```
1 = {2,3,4,5}
2 = {3,4}
3 = {4,5}
4 = {5}
5 = {}
6 = {}
```

to count how many cards are produced, we need to look up all prior cards, filter in only the cards that produce this type of card, and add up how many of _those_ cards are produced.

```
one = 1
two = (one=1) + 1 = 2
three = (one=1) + (two=2) + 1 = 4
four = (one=1) + (two=2) + (three=4) + 1 = 8
five = (one=1) + (three=4) + (four=8) + 1 = 14
six = 1
```

yeah, I think this would work, adding all that up gets to 30. let's try it out!

```rust
// note: cards must be sorted
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
```

now we're talking

```
Benchmark 1: target/release/main
  Time (mean ± σ):       3.0 ms ±   0.1 ms    [User: 2.2 ms, System: 0.5 ms]
  Range (min … max):     2.8 ms …   3.5 ms    929 runs
```


## puzzle

### part 1

The gondola takes you up. Strangely, though, the ground doesn't seem to be coming with you; you're not climbing a mountain. As the circle of Snow Island recedes below you, an entire new landmass suddenly appears above you! The gondola carries you to the surface of the new island and lurches into the station.

As you exit the gondola, the first thing you notice is that the air here is much warmer than it was on Snow Island. It's also quite humid. Is this where the water source is?

The next thing you notice is an Elf sitting on the floor across the station in what seems to be a pile of colorful square cards.

"Oh! Hello!" The Elf excitedly runs over to you. "How may I be of service?" You ask about water sources.

"I'm not sure; I just operate the gondola lift. That does sound like something we'd have, though - this is Island Island, after all! I bet the gardener would know. He's on a different island, though - er, the small kind surrounded by water, not the floating kind. We really need to come up with a better naming scheme. Tell you what: if you can help me with something quick, I'll let you borrow my boat and you can go visit the gardener. I got all these scratchcards as a gift, but I can't figure out what I've won."

The Elf leads you over to the pile of colorful cards. There, you discover dozens of scratchcards, all with their opaque covering already scratched off. Picking one up, it looks like each card has two lists of numbers separated by a vertical bar (|): a list of winning numbers and then a list of numbers you have. You organize the information into a table (your puzzle input).

As far as the Elf has been able to figure out, you have to figure out which of the numbers you have appear in the list of winning numbers. The first match makes the card worth one point and each match after the first doubles the point value of that card.

For example:

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have, four of them (48, 83, 17, and 86) are winning numbers! That means card 1 is worth 8 points (1 for the first match, then doubled three times for each of the three matches after the first).

Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
Card 4 has one winning number (84), so it is worth 1 point.
Card 5 has no winning numbers, so it is worth no points.
Card 6 has no winning numbers, so it is worth no points.
So, in this example, the Elf's pile of scratchcards is worth 13 points.

Take a seat in the large pile of colorful cards. How many points are they worth in total?

### part 2

Just as you're about to report your findings to the Elf, one of you realizes that the rules have actually been printed on the back of every card this whole time.

There's no such thing as "points". Instead, scratchcards only cause you to win more scratchcards equal to the number of winning numbers you have.

Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)

This time, the above example goes differently:

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

- Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
- Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
- Your copy of card 2 also wins one copy each of cards 3 and 4.
- Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
- Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
- Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
- Your one instance of card 6 (one original) has no matching numbers and wins no more cards.

Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!

Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, how many total scratchcards do you end up with?
