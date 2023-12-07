# day 6

## notes

- speed is in mm/ms
- each second charged is +1 mm/ms
- first race is 7ms, record was 9mm
    - strategies
        - 1ms -> 6ms at 1mm/ms -> 6mm   (+6mm)
        - 2ms -> 5ms at 2mm/ms -> 10mm* (+4mm)
        - 3ms -> 4ms at 3mm/ms -> 12mm* (+2mm)
        - 4ms -> 3ms at 4mm/ms -> 12mm*
        - 5ms -> 2ms at 5mm/ms -> 10mm*
        - 6ms -> 1ms at 6mm/ms -> 6mm
    - 4 ways to win based on these strategies
    - is this curve representative?
- second race is 15ms, record was 40mm
    - strategies
        - 1ms -> 14ms at  1mm/ms -> 14mm  (+14mm)
        - 2ms -> 13ms at  2mm/ms -> 26mm  (+12mm)
        - 3ms -> 12ms at  3mm/ms -> 36mm  (+10mm)
        - 4ms -> 11ms at  4mm/ms -> 44mm* (+ 8mm)
        - 5ms -> 10ms at  5mm/ms -> 50mm* (+ 6mm)
        - 6ms ->  9ms at  6mm/ms -> 54mm* (+ 4mm)
        - 7ms ->  8ms at  7mm/ms -> 56mm* (+ 2mm)
        - 8ms ->  7ms at  8mm/ms -> 56mm*
        - 9ms ->  6ms at  9mm/ms -> 54mm*
        - ... repeats in reverse
- winning strategy: hold for half the time
    - held = duration / 2
    - speed = held
    - race = duration - held
    - distance = race * speed
- need to find out how many ways to beat the record
- if race time is *even*, result will be *odd* and vice versa
- the _rate of change_ is always in increments of two
- start with full simulation
    - easier to prove correct
    - use that as oracle to write the trickier work skipping version
- not even going to bother with parsing! just gonna type it in by hand.

### part 2

alright definitely gonna need that work skipping version. (_edit: no I won't, but I still had fun making it_) I set up the oracle test and I'm and benchmarking changes.

```sh
cargo test --lib day6 && cargo bench count_record_beaters
```

#### attempt #1: failed

all of the winning attempts will be in a row. we can keep track of when we enter the winning zone, and then bail when we exit the winning zone.

```diff
for charge in 1..time {
    let speed = charge;
    if (time - charge) * speed > record {
        count += 1;
+    } else {
+        if count > 0 {
+            break;
+        }
+    }
}
```

the extra branching turns out to be more expensive than the extra work!

```
count_record_beaters    time:   [45.338 µs 45.395 µs 45.505 µs]
                        change: [+55.434% +55.551% +55.717%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

#### attempt #2: success

alright, so what if we could figure out how to break without an extra condition?

we know something about the data: the optimal strategy is to charge the boat for half the race time. if we jump there, we can count unconditionally, then bail when the record is _not_ beat.

we'd have skipped over half the winners, so we'd double the result (taking into account whether the time was even or odd).

the bounds and adjustment math took me a few tries to get right, I always find this off-by-1 stuff fiddly. fortunately I was able to do debug this pretty mechanically because I had the oracle to property testing against. I really like this technique for refactoring and perfomance tuning!

here's the thing—I didn't really need to do this. I wasn't looking closely enough at the input and thought my `n` was much larger. 41,968,894 iterations is more than managable!

```
# oracle
Benchmark 1: target/release/main
  Time (mean ± σ):      18.9 ms ±   0.2 ms    [User: 17.8 ms, System: 0.6 ms]
  Range (min … max):    18.2 ms …  19.7 ms    136 runs

# work skipping
Benchmark 1: target/release/main
  Time (mean ± σ):      14.1 ms ±   0.3 ms    [User: 13.0 ms, System: 0.6 ms]
  Range (min … max):    13.5 ms …  15.2 ms    175 runs
```

my efforts did net me about 25% speedup on the large puzzle input! but we're talking milliseconds.

what about if the `time` _was_ as bad as I first thought? I bumped the time up from millions to billions, which is how I originally misread it, to see if my instinct was right.

```
# oracle (independent of record)
Benchmark 1: target/release/main
  Time (mean ± σ):     17.175 s ±  0.013 s    [User: 17.086 s, System: 0.004 s]
  Range (min … max):   17.159 s … 17.205 s    10 runs

# work skipping, original record
Benchmark 1: target/release/main
  Time (mean ± σ):      3.311 s ±  0.002 s    [User: 3.300 s, System: 0.001 s]
  Range (min … max):    3.309 s …  3.315 s    10 runs
```

so here is where we get some gains, but we're still scaling linearly on size of `time`. we _could_ make this `O(log n)` by binary searching our way to the other bound, but I think we can do even better by exploring the mathematical relationship between the record and the optimal distance.

#### attempt #3: mathradical

```
record: 99mm
max: 210mm

diff: 111mm

steps: ??? (eventually: 11)

2 4 6 8 10 12 14 16 18 20 22

2->6->12->20->30->42->56->72->90->110->132
1  2   3   4   5   6   7   8   9   10   11

10*11=110 (+20)
10*9 =90


11*12=132 (+22)
11*10=110


n * (n+1) = x + 2n
n * (n-1) = x

n^2 - n = x

110 = n^2 - n
0 = n^2 - n - 110
```

ah shit we got a polynomial. time to look up the quadratic equation

## puzzle

### part 1

The ferry quickly brings you across Island Island. After asking around, you discover that there is indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of water and the small island where the ferry has docked.

As you try to figure out what to do next, you notice a poster on a wall near the ferry dock. "Boat races! Open to the public! Grand prize is an all-expenses-paid trip to Desert Island!" That must be where the sand comes from! Best of all, the boat races are starting in just a few minutes.

You manage to sign up as a competitor in the boat races just in time. The organizer explains that it's not really a traditional race - instead, you will get a fixed amount of time during which your boat has to travel as far as it can, and you win if your boat goes the farthest.

As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed for each race and also the best distance ever recorded in that race. To guarantee you win the grand prize, you need to make sure you go farther in each race than the current record holder.

The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually toy boats, each with a big button on top. Holding down the button charges the boat, and releasing the button allows the boat to move. Boats move faster if their button was held longer, but time spent holding the button counts against the total race time. You can only hold the button at the start of the race, and boats don't move until the button is released.

For example:

```
Time:      7  15   30
Distance:  9  40  200
```

This document describes three races:

- The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
- The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
- The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.

Your toy boat has a starting speed of zero millimeters per millisecond. For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by one millimeter per millisecond.

So, because the first race lasts 7 milliseconds, you only have a few options:

- Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
- Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
- Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
- Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
- Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
- Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
- Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.

Since the current record for this race is 9 millimeters, there are actually 4 different ways you could win: you could hold the button for 2, 3, 4, or 5 milliseconds at the start of the race.

In the second race, you could hold the button for at least 4 milliseconds and at most 11 milliseconds and beat the record, a total of 8 different ways to win.

In the third race, you could hold the button for at least 11 milliseconds and no more than 19 milliseconds and still beat the record, a total of 9 ways you could win.

To see how much margin of error you have, determine the number of ways you can beat the record in each race; in this example, if you multiply these values together, you get 288 (4 * 8 * 9).

Determine the number of ways you could beat the record in each race. What do you get if you multiply these numbers together?

### part 2

As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces between the numbers on each line.

So, the example from before:

```
Time:      7  15   30
Distance:  9  40  200
```

...now instead means this:

```
Time:      71530
Distance:  940200
```

Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!

How many ways can you beat the record in this one much longer race?
