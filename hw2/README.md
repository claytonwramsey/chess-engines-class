# COLL 110 HW2: Perft

Chess is a very complicated game.

In fact, one of the largest parts of a chess engine is the move generator - the code responsible for
generating the set of legal moves.
However, it's very difficult to verify that your move generator was written correctly, and that any
optimizations you make in the future don't cause bizarre edge-case bugs.
Additionally, it's very difficult to benchmark the performance of just the move generator in a way
which neatly reflects the ways it's used in a real search.

Enter _perft_, which is short for performance testing.
The core idea of perft is that for some starting board $b$ and some depth $d$, we count the number
of legal sequences of moves of length $d$ plies starting from $b$.
At a depth of zero, the number of legal sequences is always 1.

Because perft searches a full game tree, it tends to use roughly the same distribution of positions
as a real search in a chess engine does.
Additionally, because it covers so many positions (typically millions or even billions), it allows
us to quickly verify the correctness of our program.
However, perft is not a complete substitute for all unit testing, because tracking down a bug from
an incorrect perft result is quite hard.

We can formalize perft mathematically, as shown below.
We consider $\text{Moves}(b)$ to be a function which returns the set of all legal moves on $b$
and $\text{Make(m, b)}$ to be the state of $b$ after making the move $m$.

$$
\text{Perft}(b, d) =
\begin{cases}
    1 & d = 0 \\
    \sum_{m \in \text{Moves}(b)} \text{Perft}(\text{Make}(m, b), d - 1) & \text{otherwise}
\end{cases}
$$

## Your assignment

Your assignment is to implement a perft routine in Rust.
You should do this by filling out the body of `perft()` in
[`src/movegen/mod.rs`](src/movegen/mod.rs).

I have already provided starter code for you which contains implementations of move generation and
board move-making.
`Board::from_fen` will convert a Forsyth-Edwards notation (FEN) string into a Board.
`movegen::get_moves` will return a `Vec` containing the set of all legal moves on a `Board`.
`Board::make_move` will make a move on a board.
Note that since `make_move` mutates the board it is called on, you will need to manually copy the
board at every step.

## Grading

Your grade for this assignment will be the proportion of perft unit tests that your code passes.
There are six total tests.
You can run them all from the command line by executing `cargo test perft`, which executes every
test with `perft` in its name.

If all six unit tests pass, you will receive a grade of 100% for this assignment.

## Benchmarking results

When you're done writing your perft code, you can check how fast it is by running the main
executable with `cargo run --release`.
It will take a little while to run, so be patient!

My implementation of perft for this assignment was able to run at 137 million nodes/second.
On the engine that I built in my free time it runs at 240 million nodes/second.
Some of the fastest perft implementations run at 2 billion nodes/second.
If yours runs any faster than any of these, let me know!

## Tips

- You can run `cargo doc --open` to open a book containing the documentation for all the code in
  this assignment in a human-readable way.
- You can format your code with `cargo fmt` to make it less painful to look at.
- You can use `cargo clippy` to get a highly-opinionated linter to review your code.
- If you want to make your perft routine really fast, try profiling it and generating a flamegraph
  with [inferno](https://github.com/jonhoo/inferno).
  This will provide really useful information on where the slow parts of your code are.
- Email me if you get stuck!
