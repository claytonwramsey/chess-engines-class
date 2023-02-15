# Tree search

## Minimax

$$
\text{Minimax}(b, d) = \begin{cases}
  \text{LeafEvaluate}(b) & d = 0 \\
  \max_{m \in \text{Moves(b)}}(-\text{Minimax}(\text{Make}(b, m), d - 1)) & d > 0
\end{cases}
$$

(note: take special care in case of mates/draws; this description assumes `LeafEvaluate` handles
this)

## Alpha-beta

```py
def alpha_beta(board, depth, alpha, beta):
    if depth = 0:
        return leaf_evaluate(board)

    best_score := -infinity

    for m in board.moves(), sorted in descending order of quality:
        board.make(m)
        score := -alpha_beta(board, depth - 1, -beta, -alpha)

        best_score = max(score, best_score)
        if alpha < score:
            if beta <= score:
                break

            alpha := score

        board.undo()

    if best_score = -infinity:
        best_score := 0 if board.drawn() else mated

    return best_score
```

- Moves must be sorted!

- yields $O(B^{d/2})$ performance

- this is pretty good

## PVS

```py
def pvs(board, depth, alpha, beta, pv):
    if depth = 0:
        return leaf_evaluate(board)

    best_score := -infinity

    for (idx, m) in enumerate(board.moves(), sorted in descending order of quality):
        board.make(m)

        score := -infinity

        if not pv or i > 0:
            score := -pvs(board, depth - 1, -alpha - 1, -alpha, false)

        if pv and (i = 0 or alpha < score < beta):
            score := -pvs(board, depth - 1, -beta, -alpha, true)

        best_score = max(score, best_score)
        if alpha < score:
            if beta <= score:
                break

            alpha := score

        board.undo()

    if best_score = -infinity:
        best_score := 0 if board.drawn() else mated

    return best_score
```
