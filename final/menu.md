# COLL 110 Final Project Menu

This is a list of suggestions for final projects for COLL 110.
You do not need to select something from this list, instead, it is meant to be a jumping-off point
to give you a feel for what can be done.

## Easy tasks (<50 lines of code)

These tasks are relatively easy and can usually be implemented in under an hour.
You will likely need to reserve some time for testing the Elo gain of your changes, though.

- **Killer move heuristic.**
  Consider the following position:
  `r1bqkbnr/pppp1ppp/2n5/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 3 3`.
  If Black makes a bad move, White can immediately reply with Qxf3# and win the game on the spot.
  We would like to encode that knowledge: that somewhere down the line, there is a move threatened
  which we have to defend against.
  One way to achieve that is by the _killer move heuristic_: we store a list of "killer" moves for
  each depth that we searched.
  For each depth, the killer move is the most recent move to have caused a beta cutoff.
  Typically, killer moves are ordered after all good captures and before all quiet moves.

- **Pondering**.
  As any chess coach might tell you, you should still think on your opponent's time.
  Chess engines can do the same.
  The UCI standard supports an option on `go` commands called `ponder` which is explicitly intended
  to support thinking on the opponent's time.
  Make an engine which is compliant to the `ponder` option in UCI and will spend extra time
  thinking.

- **PEXT Bitboards**.
  _Note: this will not be possible on non-Intel computers (such as ARM processors or M1 Macs)._
  Magic bitboards are not the only way of generating rook or bishop moves.
  On Intel's x86 instruction architectures, the instruction `pext` (parallel bits extract) does
  exactly what we need a magic multiply to do: extract the bits from a masked int to create an
  index.
  I strongly recommend that you have your software fall back to using magic bitboards if the
  target architecture does not support the `pext` instruction.

- **Transposition table prefetching.**
  A chess engine can spend as much as 20% of its time waiting on transposition table lookups.
  The main reason why this is so slow is because of cache misses - if the transposition data is not
  already in the cache, it can take an enormous amount of time to load from RAM.
  One way of reducing this slowdown is by prefetching:
  As soon as the hash-key of a position is known (i.e. immediately after making a move),
  we can perform a prefetch to start loading the associated transposition table entry into the cache
  so that it will be ready by the time it's needed.
  I suggest starting by taking a look at the intrinsic function `prefetch_read_data`.

- **Quick draw detection.**
  The 3-move repetition rule is rather silly.
  As far as an engine is concerned, if you repeat moves once you might as well repeat moves forever.
  We can more effectively detect draws by reducing the number of required repetitions to 2 during a
  search.
  In other words, add a rule so that if a move is ever repeated in a search, we can immediately call
  that line a draw.

## Medium tasks (50 - 500 lines of code)

- **Phased move generation.**
  At most steps in a search, we only need to look at one or two moves to prove that the current line
  will not be reached due to a beta cutoff.
  It's therefore very wasteful to spend a lot of time generating moves that are never actually used.
  One solution to this is to use a technique called phased move generation: instead of creating a
  list of all the moves and sorting it, we only generate a handful of moves at a time.
  Typically, we start by generating captures and split out the "good" (capturee more valuable than
  capturer) captures from the "bad" (captureee less valuable than capturer) captures.

- **Endgame solutions**.
  Some endgames have known solutions.
  For instance, a king-and-pawn endgame is always a forced win or draw for a player with the pawn,
  and there are rules for determining the outcome of the game based on the location of the pieces.
  Common solved endgames include:

  - KRvK
  - KQvK
  - KPvK
  - KBNvK
  - KBvKN
  - KQvKR
  - KQvKP

  You can implement some of these endgame solutions and see if that improves performance of the
  engine.

- **Mobility-based evaluation**.
  Tomato uses a piece-square table for the entirety of its evaluation.
  This is okay for most applications, but in practice it's not perfect.
  Many engines instead use mobility to get a finer sense of piece activity on their board.

  This approach is simple: for every piece on the board, get the set of all squares "visible" to the
  piece.
  You can decide for yourself whether allies count as "visible" or not (or use empirical data to
  decide!).
  Then, check a lookup table for a score associated with that level of mobility.
  For instance, having a bishop which can see 10 squares could get a bonus of +20 centipawns,
  while having a knight which can see 0 squares could get a malus of -30 centipawns.
  You will have to additionally alter the tuner to create tuned values for your mobility scores.

- **Incrememntal evaluation.**
  Heuristic evaluation of a position is very slow.
  You have to iterate through every piece type and run up a big total - and for what?
  A single move will usually change the evaluation of the position by very little.
  Implement an incremental (aka cumulative) evaluation for boards, where each board stores its
  evaluation and updates that evaluation as moves are made.
  Then, the heuristic evaluation step just reads off that total from the board.

- **Generating your own training data.**
  We normally use the ZuriChess traning set for training our evaluation function, mostly because
  it's a high-quality dataset.
  However, the big kids like to make their own training data.
  Follow the procedure outlined
  [here](https://bitbucket.org/zurichess/zurichess/wiki/Choosing%20positions%20for%20Texel%27s%20Tuning%20Method)
  (or make your own procedure) to develop your own training set for tuning the engine.

## Hard tasks (500+ lines of code)

These suggestions will require either total rewrites of the engine part of the code or an enormous
effort on the part of the implementer.
However, they will probably be the most interesting and fulfilling options.
You will have to make a lot of decisions for yourself about how to implement them and it will likely
take a lot of tuning to get it right.

- **Tablebase support**.
  Why bother making an endgame solver when every endgame has been solved?
  A tablebase is a kind of database for looking up the guaranteed result of a game in a given
  position.
  Tablebases have been generated for positions of up to seven pieces on a board at a time.
  I suggest implemententing probing support for Syzygy tablebases - you may find
  [this post](https://talkchess.com/forum3/viewtopic.php?p=936095#p936095) by the author of Syzygy
  tablebases helpful.

- **Deep learning**.
  Most chess engines use a linear combination of rules to produce their heuristic evaluation.
  This is equivalent to a single-layer neural-network.
  Why stop there?
  Implement a feedforward neural network which takes in a position and returns an evaluation of it.
  I suggest doing some validation testing to choose your network architecture.

- **Monte-Carlo search**.
  Most chess engines use a conventional alpha-beta search, and Tomato is no exception.
  Instead, implement a Monte-Carlo stochastic search for Tomato and see if it yields better results.
