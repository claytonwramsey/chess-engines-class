# Bitboards and Board Representation

COLL 110 notes: lecture 3

This is meant to be a rough summary - for more detail, ask me or try looking stuff up on your own.

## Background

During a search, we'll spend most of our time generating, making, and unmaking moves.
Accordingly, we need a board representation which is highly performant to enable quick searching.

In particular, we need a board representation which allows us to quickly:

- Query the location of a piece (e.g. "where is the white king?").

- Determine the type of a piece at a location (e.g. "what piece is on A1?").

- Mutate the locations of pieces.

- Add and remove pieces.

### Mailboxes

The most intuitive approach to implementing board representation is called a _mailbox_.
In a mailbox representation, each square is associated with a value representing its contents.
In Rust, one might represent it as follows:

```rust
struct MailBoard {
    mailboxes: [Option<(Piece, Color)>; 64],
}
```

The primary benefit of a mailbox representation is that it allows very quick per-square lookup.
However, it's bad for by-piece lookup and slow for move generation.

### Piece lists

An alternate approach to storing board information is to store a list of locations for each piece
and color.
This gives very quick piece location lookup, but is slow for by-square lookup (requiring iteration
through every single list).

```rust
struct ListBoard {
    knights: [Vec<Square>; 2],
    bishops: [Vec<Square>; 2],
    rooks: [Vec<Square>; 2],
    queens: [Vec<Square>; 2],
    pawns: [Vec<Square>; 2],
    kings: [Vec<Square>; 2],
}
```

---

It's certainly possible to combine piece lists and mailboxes for decent performance.
However, a better solution exists...

## Bitboards

As you may remember from the homework, a bitboard is a representation for a set of squares.

To do so, we start by giving each square a unique number identifying it:

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

We can then construct a unique number $\text{BB}(S) \in \mathbb{N} \cup \{0\}$ representing each subset $S$ of $\{0, 1, \cdots 63\}$.

$$
\text{BB}(S) = \sum_{s \in S} 2^s
$$

This allows us to use bitwise operators to efficiently perform setwise operations on bitboards.

### Using bitboards to represent a board

We can compose bitboards to represent a board's state by storing a bitboard for every piece and
color.

```rust
struct Board {
    white: Bitboard,
    black: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    pawns: Bitboard,
    kings: Bitboard,
}
```

Then, to extract information about a board, we can use our bitwise operations to efficiently
compute our queries:

```rust
fn find_white_king(b: &Board) -> Square {
    (b.white & b.kings).trailing_zeros()
}
```

In order to fully implement all the rules of chess, we have to add a little extra information to represent castling rights, en passant, the 50-move rule, and the player to move.

```rust
struct Board {
    white: Bitboard,
    black: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    pawns: Bitboard,
    kings: Bitboard,
    /// bitwise flags for castling rights
    castle_rigths: u8,
    /// square where pawns can move via en passant
    ep_square: Option<Square>,
    /// number of plies since last pawn push or capture
    rule50: u8,
    /// color of player whose turn it is
    player: Color,
}
```

### Tricks with bitboards

We often need to iterate over all the squares of a bitboard.
Here's a slow method:

```rust
for sq in 0..64 {
    if bb & 1 << sq == 0 {
        continue;
    }
    // do the iteration step here
}
```

However, this is pretty slow - we do 64 checks in the worst-case of iterating over an empty
bitboard.
Here's a faster way:

```rust
while bb != 0 {
    let sq = bb.trailing_zeros();
    // do the iteration step here
    bb &= bb - 1;
}
```

## Zobrist hashing

We have a problem.
In order to fully implement all of chess, we need to also implement the 3-move-repetition rule.
One way of implementing this is to story the history of an entire game, and check against it every
time we want to make a move.

```rust
/// Returns true if reaching state `b` would be a 3-move repetition.
fn is_repetition(history: &[Board], b: &Board) -> bool {
    history.iter().map(|b1| b1 == b).count() >= 2
}
```

This is too slow.
We could try using a hash-table instead:

```rust
fn is_repetition(num_reps: &HashMap<Board, u8>, b: &Board) -> bool {
    num_reps.get(b).map_or(false, |&num| num >= 2)
}
```

However, this requires the slow operations of constructing a hash-key for a board and comparing two
boards for equality.

Enter Zobrist keys:

For each `(piece, square, color)` triple $(p, s, c)$, construct a key
$Z(p, s, c) \in \mathbb{Z}_2^b$ for some number of bits $b$.
Typically, these keys are simply constructed by pseudorandom generation at compile time.

Then, define the hash-key $H(b)$ for some board $b$ as follows:

$$
H(b) = \sum_{\text{piece $p$ at square $s$ with color $c$ in $b$}} Z(p, s, c)
$$

In practice, each Zobrist key $Z(p, s, c)$ will be a 64-bit integer (`u64` in Rust).
Fans of abstract algebra will recall that summation in $\mathbb{Z}_2^b$ is actually just the bitwise
XOR operation on integers, making the construction of this hash-key really fast.

Then, we can incrementally generate our hash-key whenever we make a move:

```rust
/// Compute the new hash key which will be generated after playing `m` on `b`.
fn new_hash_key(b: &Board, m: Move) -> u64 {
    let mut new_key: u64 = b.hash_key();

    let fsq: Square = m.starting_square();
    let tsq: Square = m.target_square();

    new_key ^= Z(b.type_at(fsq), fsq, b.color_at(fsq));
    new_key ^= Z(b.type_at(fsq), tsq, b.color_at(fsq));

    if m.is_capture() {
        new_key ^= Z(b.type_at(tsq), tsq, b.color_at(tsq));
    }

    new_key
}
```

We can then use those hash-keys for very efficient repetition lookup:

```rust
fn is_repetition(num_reps: &HashMap<u64, u8>, b: &Board) -> bool {
    num_reps.get(&b.hash_key()).map_or(false, |&num| num >= 2)
}
```

**Note**: in practice, we also include Zobrist keys for the player to move, the castling rights,
and the en passant square, so the true hash-key logic is more complex.

### Linear independence

Two boards $b_1, b_2$ will encounter a hash collision if their hash-keys are equal.
In other words, if the following expression is true:

$$
\sum_{\text{piece $p$ at square $s$ with color $c$ in $b_1$}} Z(p, s, c)
=
\sum_{\text{piece $p$ at square $s$ with color $c$ in $b_2$}} Z(p, s, c)
$$

We want to minimize the probability of a hash collision, so we need to maximize the amount of
Zobrist keys that have to be summed together to make two differnt sums equal.

More mathematically speaking, we want to find a set of Zobrist keys $Z$ such that its the size of
its minimal linearly dependent subset is maximized.

In other words, we want to find the following:

$$
\underset{Z}{\operatorname{argmax}}  \left(\min_{K \subseteq Z} |K| : \sum_{k \in K} = 0 \right)
$$

Note that there does not exist a linearly independent set of Zobrist keys - we need to generate
around 800 total and they are elements of a vector space of dimension 64.
