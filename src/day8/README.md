# day 8

## notes

- first line is instructions, `L` for left, `R` for right
- `AAA` is the start, `ZZZ` is the end
- example network:

```
AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
```

- example of path for two instructions

```
L  => AAA->BBB->DDD->DDD->DDD...
LR => AAA->BBB->EEE->EEE->EEE...
RL => AAA->CCC->ZZZ
R  => AAA->CCC->GGG->GGG->GGG...
```

- dead ends! 1-distance cycles in disguise
- could have n-distance cycles!
    - if `DDD = (AAA, DDD)`, the path would cycle from the start
- make sure the path never loops back on itself?
    - gotta keep track of the path
        - really just the nodes, don't need edges
    - if the node has been previously seen, we have a cycle
- ah, but the instructions can cause a path that does cycle, but eventually terminates

```
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
```

```
AAA-[L]->BBB-[L]->AAA-[R]->BBB-[L]->AAA-[L]->BBB-[R]->ZZZ
```

- for now I'm just gonna do full simulation, just walk everything and count
    - escape hatch would be to have a maximum of like a billion steps
- worth keeping track of node visits, some other metadata anyway

## puzzle

### part 1

You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

```
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
```

Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

```
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
```

Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
