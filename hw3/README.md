# COLL 110 HW 3: Principal Variation Search

Congratulations for making it this far!
I think this is going to be the hardest of all homework assignments - accordingly, I'll be lenient
with grading on this one.

We've now built up enough of our engine to be able to actually start doing engine-y things with it.
Accordingly, we need a new search algorithm to be able to search trees.

## Your assignment

Your assignment today is to implement principal variation search for our chess engine.
Starter code has been provided for you; all that is left to do is populate the function `pvs()` in
[src/enigne/search.rs](src/engine/search.rs).
Principal variation search, also known as NegaScout, is an algorithm for very efficiently searching
trees.
It consists of three core parts:

1. Greedily search the heuristically-best (but not proven-best) line.
1. Attempt to prove that the greedily-searched PV is best via zero-window search.
1. If the proof fails, try a full-window search to prove that it works.

Like other tree searches, we use a heuristic for evaluating positions at the leaves of our search
and also have a candidacy function for sorting moves by their quality.

As always, recall that my academic integrity policy allows any and all forms of collaboration,
including the Internet, so long as you cite your sources.

### Pseudocode

Here is the pseudocode for implementing PVS:

```py
def pvs(position, depth_so_far, depth_to_go, alpha, beta, pv):
    if depth_to_go = 0:
        return leaf_evaluate(position)
    if position.drawn():
        return 0

    best_score := -infinity
    for i, m in enumerate(position.moves()), sorted in ascending order of candidacy:
        position.make(m)
        score := -infinity

        # Attempt to prove that the principal variation is best
        if not pv or i > 0:
            score := -pvs(
              position,
              depth_so_far + 1,
              depth_to_go - 1,
              -alpha - 1,
              -alpha,
              false)

        # if the proof failed, or if we have no principal variation yet,
        # perform a principal variation search.
        if pv and (i = 0 or alpha < score < beta):
            score := -pvs(
              position,
              depth_so_far + 1,
              depth_to_go - 1,
              -beta,
              -alpha,
              true)
        position.undo()

        # Alpha-beta pruning
        if best_score < score:
            best_score := score

            if alpha < score:
                if beta < score:
                    return score

                alpha := score
    if no moves were played:
        if king is in check:
            return 0
        else:
            return (mated in depth_so_far)

    return best_score
```

### Useful functions for you

This is just a list of things which you might find useful.
In order to read more detail on each item, run `cargo doc --open` in your
terminal to get a full documentation of each item.

- `Eval`
  - `Eval::mate_in` yields a high positive score for mating (i.e. corresponding to mating the
    opponent).
    `-Eval::mate_in(n)` yields a score equivalent to getting mated (this should be notable for being
    equivalent to "mated in n").
  - `Eval::MIN` behaves much like -infinity in pseudocode.
  - `Eval::DRAW` behaves much like 0 in pseudocode.
  - `Eval::centipawns()` converts an integer value to an evaluation.
    Notably, `Eval::centipawns(1)` is the smallest possible `Eval`.
- `leaf_evaluate()` returns a heuristic evaluation of a game.
- `Game` can be used to do most of the heavy lifting on games:
  - `Game::get_moves()` returns a vector of legal moves (but does not check for draws or mates).
  - `Game::drawn()` determines whether a game is drawn.
  - `Game::board()` returns a reference to the current board state of the game.
  - `Game::make_move()` makes a move on the game.
  - `Game::undo()` undoes the most recent move played.
- For some `Board` `b`, `b.checkers` is a bitboard containing the squares containing all
  checkers.
  If `b.checkers` is empty, then the king is not in check.
- `Vec` (the standard type)
  - `Vec::sort_by_cached_key` takes a function which creates a totally-ordered key from each of its
    elements and then sorts in ascending order by those keys.
    Note that this function does _not_ return a new `Vec` and instead mutates the contents of the
    original vector!
- `candidacy()` returns a heuristic candidacy for each move.
  High candidacies correspond to promising moves.

## Grading

I've written a handful of tests for your code.
To verify that your code is correct, I will run all of the tests in `src/engine/search.rs` and base
your grade off of those results.

Do not attempt to hardcode the results for these tests.

## Extra credit: mate distance pruning

For extra credit, implement mate distance pruning in addition to principal variation search.
Mate distance pruning is a form of tree pruning which prevents searching at a depth when you know
that you cannot prevent a mate, or that you already have a faster mate than the current depth.
For more information, refer to the
[page on the Chess Programming Wiki](https://www.chessprogramming.org/Mate_Distance_Pruning).
