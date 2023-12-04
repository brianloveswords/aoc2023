# day 3

## notes

would like to avoid keeping track of the bounds of the schematic.

could probably be fun at some point to look into a parser combinator e.g. [nom](https://github.com/rust-bakery/nom), but for now I'm still okay hand-rolling the parsers for extra practice and more control over what gets tracked.

### `Range`

moved `Range` to util, could be useful for other things, also added some property tests to it.

`is_overlapping` uses range sizes instead of bounds because that's more intuitive to me specifically, but it requires being able to store an integer twice the capacity of the `start` and `end`. another potential benefit is that it has fewer branching than the bounds check version.

could be fun to use that as an oracle to build & test the version that does all the bounds checking, then benchmarking to see if there's any performance difference at all. not really expecting there to be a significant one, but it's a good excuse to setup benchmarking suite.

#### benchmark results

always find the bounds checking version hard to get right, having a known-good oracle to work from was great.

```rust
proptest! {
    #[test]
    fn is_overlapping_oracle_test(a in gen_range(), b in gen_range()) {
        assert_eq!(a.is_overlapping_bounds(&b), a.is_overlapping_sizes(&b));
        assert_eq!(b.is_overlapping_bounds(&a), b.is_overlapping_sizes(&a));
    }
}
```

assuming I set up the benchmark right, [criterion](https://bheisler.github.io/criterion.rs/book/criterion_rs.html) is telling me the bounds version is a performance regression, but we're talking fractions of a nanosecond.

```
is_overlapping          time:   [3.0799 ns 3.0832 ns 3.0870 ns]
                        change: [+6.8911% +7.0197% +7.1413%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

overlapping is a part of adjacency checking, and that's a significant enough part of the workload that the change is meaningful enough in the full program that the regression is noticible there, too.

```
day3::part1             time:   [399.80 µs 400.44 µs 401.12 µs]
                        change: [+3.4578% +3.9077% +4.3164%] (p = 0.00 < 0.05)
                        Performance has regressed.

day3::part2             time:   [226.95 µs 227.47 µs 228.07 µs]
                        change: [+3.5294% +3.8345% +4.1292%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

## puzzle

### part 1

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

### part 2

The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?
