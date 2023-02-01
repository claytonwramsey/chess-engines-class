# Move generation

note - chess club people are making an IM league (<https://linktr.ee/chess.at.rice>)

## Representation

- carry from-square (6 bits) and to-square (6 bits) as packed data structure

- also include promotion type - 2 bits to represent 4 promotion types

- to pack into a 16-bit integer, we would have 2 bits leftover.
  Use these for flags (mark as promotion, en passant, or castle)

## Pseudolegal moves

_pseudolegal_: (adj) (of a move) able to be played if the rules regarding king safety, the 50 move
rule, draw by repetition, and draw by material did not exist.

### Easy pieces: knights and kings

- Store a lookup table of bitboards for each square in the board.

```rust
const KNIGHT_MOVES: [Bitboard; 64] = /* ... */
const KING_MOVES: [Bitboard; 64] = /* ... */
```

- For each square `sq` $\in \{0, 1, \cdots 63\}$, `KNIGHT_MOVES[i]` corresponds to the set of
  squares reachable by a knight on `sq` (likewise for kings).

- Compute moves as follows:

```rust
for from_sq in board[Piece::Knight] & board[board.player] {
    for to_sq in KNIGHT_MOVES[from_sq as usize] {
        let m = Move::normal(from_sq, to_sq);
        println!("{m}");
    }
}
```

### Pawns: wonky

- Store a lookup table for attacks on each side

```rust
const PAWN_ATTACKS: [[Bitboard; 64]; 2];
```

- Use bitshifts to compute movement (means you only iterate through the set of legal moves)

```rust
// imagine white to move
let singlemoves = board[Piece::Pawn] << 8 & !board.occupancy();
let doublemoves = singlemoves << 8 & !board.occupancy() & RANK_4;

for to_sq in singlemoves {
    let from_sq = to_sq - 8;
    let m = Move::normal(from_sq, to_sq);
}

for to_sq in doublemoves {
    let from_sq = to_sq - 16;
    let m = Move::normal(from_sq, to_sq);
}
```

- just manually check the en passant case

### Rooks bishops, queens: extra wonky

The intuitive way to compute rook and bishop attacks is to linearly iterate out from the square
containing the piece to move on each ray until we encounter an occupied square.
However, this is quite slow.

We would really like to be able to create an $O(1)$ method to get all of the squares attacked by a
piece.

Important concept: _attacks_; the set of squares visible to a piece on a square.

TODO: actually finish these notes.
