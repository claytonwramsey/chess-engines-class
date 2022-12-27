# COLL 110 HW1: Popcounts

A common construct in a chess engine is called a _bitboard_. A bitboard is a set of squares, backed
by a 64-bit integer.
This allows $O(1)$ performance on set intersection, union, symmetric difference, insertion, and
deletion.

We can start by imagining every square to be uniquely assigned a number.
Traditionally, the square A1 gets assigned the number 0, and the square H8 gets the number 63.

The squares in between are distributed as follows:

```text
8    | 56 57 58 59 60 61 62 63
7    | 48 49 50 51 52 53 54 55
6    | 40 41 42 43 44 45 46 47
7    | 32 33 34 35 36 37 38 39
4    | 24 25 26 27 28 29 30 31
3    | 16 17 18 19 20 21 22 23
2    |  8  9 10 11 12 13 14 15
1    |  0  1  2  3  4  5  6  7
-----+------------------------
     |  A  B  C  D  E  F  G  H
```

We can then create a one-to-one mapping betwen sets of squars and 64-bit integers.
Let $S \in \{0, 1\}^{64}$ We can then represent $S$ by a number as follows:

$$
BB(S) = \sum_{s \in S} 2^s
$$

For example, if we wanted to find an integer to represent the squares `{A1, D4, G3}`, we would first
convert this set of squares to integers: $\{0, 22, 27\}$.

We could then convert it to its value as a bitboard:

$$
\begin{aligned}
BB(\{0, 22, 27\}) &= 2^0 + 2^{22} + 2^{27} \\
&= 1 + 4,194,304 + 134,217,728 \\
&= 138,412,033
\end{aligned}
$$

Once we have a bitboard, we often have to compute the number of squares in a bitboard.
This operation is called _population count_, sometimes abbreviated as _popcnt_.

We can define population count as follows:

$$
\mathit{Popcount}(BB(S)) = |S|
$$

Computing population count is essential to board evaluation.
For instance, let's say you want to know how many points of material are on some board `b`.
You could simply write the following code:

```rust
fn total_material(b: &Board) -> u32 {
     // pretend that `pawns()`, `knights()`, etc return u64 bitboards for the occupancy of each
     // square
     b.pawns().count_ones()
          + 3 * b.knights().count_ones()
          + 3 * b.bishops().count_ones()
          + 5 * b.rooks().count_ones()
          + 9 * b.queens().count_ones()
}
```

In Rust, population count is already implemented for you via the function `u64::count_ones()` (and
likewise for all other integer types).
Typically, this implementation is the fastest in nearly all cases.
However, to get you all started with Rust,

## Your assignment

For your assignment, you will implement a few
