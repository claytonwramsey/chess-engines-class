# COLL 110 HW1: Popcounts

A common construct in a chess engine is called a _bitboard_. A bitboard is a set of squares,
represented as a 64-bit integer.
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

We can then create a one-to-one mapping betwen sets of squares and 64-bit integers.
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

Here's how we can compute some important setwise operations with a bitboard "`a`" representing some
set $A$:

|  **Mathematical operation**   |      **Bitboard implementation**      |
| :---------------------------: | :-----------------------------------: |
|         $ A \cup B $          |               `a \| b`                |
|         $ A \cap B $          |                `a & b`                |
|      $ A \backslash B $       |               `a & !b`                |
|    $ A \bigtriangleup B $     |                `a ^ b`                |
| $ \min(A): A \neq \emptyset $ |         `a.trailing_zeros()`          |
| $ \max(A): A \neq \emptyset $ |       `64 - a.leading_zeros()`        |
|  $\{x: x > 0, x + 1 \in A\}$  |               `a >> 1`                |
| $\{x: x < 64, x - 1 \in A\}$  |               `a << 1`                |
| $ A \backslash \{\min(A)\} $  |             `a & (a - 1)`             |
|            $\|A\|$            | to be implemented in this assignment! |

Once we have a bitboard, we often have to compute the number of squares in a bitboard.
This operation is called _population count_, sometimes abbreviated as _popcnt_.

We can define population count as follows:

$$
\text{Popcount}(BB(S)) = |S|
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
Typically, this implementation is the fastest in nearly all cases - but that won't stop us from
trying our own implementations!

## Your assignment

For your assignment, you will implement a few population counting algorithms.
Each of these will be relatively easy (I hope).
A mathematical description of each of the algorithms is given below.

### The naive method

The most naive way of computing population count is by checking every single square to see if it's
contained in a board.

$$
\text{Popcount}(BB(S)) = \sum_{s \in 0..64} I(s \in S)
$$

(where $I$ is the Boolean indicator function).

This method is pretty slow, because it always takes 64 checks to create the total.
However, because the algorithm's execution is independent of the value of $S$, it can be unrolled by
a compiler for surprisingly good performance.

### Kernighan's method

This is a very old trick, attributed to Brian Kernighan (one of the creators of the Unix operating
system).

$$
\text{Popcount}(BB(S)) = \left\{
    \begin{array}{lr}
        0 & S = \emptyset \\
        1 + |S \backslash \{\min(S)\}| & \text{otherwise}
    \end{array}
\right\}
$$

This method is fastest on sparsely-populated bitboard (and grows linearly with the population of
the bitboard).

### Lookup tables

We can also try to make our computation more efficient with a lookup table.
In our case, we'll make a lookup table for the population of 8-bit numbers, and sum those
populations up independently.

$$
\text{Popcount}(BB(S)) = \sum_{i = 0}^{7} \text{Popcount}(BB(S \cap \{8i, 8i + 1, \cdots 8i + 7\}))
$$

You can use even large lookup tables (such as 16-bit ones), but you get diminishing returns here due
to cache pollution.

## Benchmarking

A `main` function is given in this homework for benchmarking these implementations against the
standard library.
To use it, simply execute `cargo run --release` in the terminal.
It will give some reports on the performance of each function.
**If you simply use VS Code's "Run" button over the main method, you will not get a fully optimized
build (and therefore your results will be meaningless)**.

## Grading

Any implementation which is correct (i.e. implements all methods shown here) and passes all tests
will receive full credit.
Ample partial credit will be given for incomplete solutions.
To check that your implementation gives the right results, run `cargo test`.

I also recommend using `cargo clippy`, as it will give you some convenient tips, and `cargo fmt` to
auto-format your code.

Extra credit will be given for interesting analysis of the benchmarking results.
