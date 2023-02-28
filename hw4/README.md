# COLL 110 HW4: Tuning your engine

This is the last of 4 homework assignments for COLL 110, and probably the most useful.
We'll be implementing (part of) a tuner for Tomato, a pedagogical chess engine.
My hope is that this homework will put you in a pretty good place for tuning your engine.

**This assignment is relatively light on coding, but it will require a lot of time to run.
Start early!**

## Important problems

Inside of [`quiet-labeled.epd`](quiet-labeled.epd), there is a list of 725,000 chess positions each
tagged with the outcome of the game they were played in.
Meanwhile, inside of [`src/engine/evaluate`](src/engine/evaluate), we have some very nice leaf
evaluation functions with absolutely no data!

Perhaps we can take these two problems and solve each other.
The leaf evaluation function for our chess engine is the weighted sum of some number of rules.
For example, the material value for a knight is (300, 300), meaning that a knight is worth 300
centipawns in the midgame and also in the endgame.

### The math

Let's try to formulate this model mathematically.
First, we'll assume that every board $b$ has some feature vector $\bold {x}(b)$.
Each element of $\bold {x}(b)$ corresponds to one "fact" about the board - for example,
$\bold x(b)_0$ can be the net number of knights on the board, weighted by phase of the game.
In a game where Black has 2 knights, White has 1 knight, and the game is 75% midgame and 25%
endgame (i.e. a phase of 0.25), $\bold x(b)_0$ is $0.75 \cdot (1 - 2) = -0.75$.

Next, let's imagine that the values for each rule can be expressed as some weight vector $\bold w$.
For example, $\bold w_0$ is the value of a knight in the middle-game, so right now it's $3$.
Now, to evaluate the leaf evaluation $l(b)$, we can construct it as an inner product of these two
vectors:

$$
l(\bold w, b) = \bold w^T \bold x(b)
$$

What we'd like to do is find some weight vector $\bold w$ such that $l(\bold w, b)$ is high for
winning positions and low for losing positions.
Our annotated dataset can help us figure this out.
Each position in `quiet-labeled.epd` is annotated with some result $y(b)$, which is defined as
followed:

$$
y(b) = \begin{cases}
1 & b \text{ is a win for White} \\
\frac{1}{2} & b \text{ is a draw for White} \\
0 & b \text{ is a loss for White} \\
\end{cases}
$$

What we might try to do to find the optimal $\bold w$ is to find one which minimizes the difference
between $l(\bold w, b)$ and $y(b)$ over every board $b$ in the set of all training boards $B$:

$$
\bold w^* = \argmin_{\bold w} \frac{1}{2} \sum_{b \in B} |l(\bold w, b) - y(b)|^2
$$

_However, this has a problem!_
If we doubled the features on $\bold x$, the leaf evaluation $l(b)$ would double - and our error
would increase, even if we were confident about the fact that we were winning!

**Idea: use some function to squeeze l(b) to look more like y(b)**.
We want a function $\sigma(l)$ which is roughly 1 when $l$ is big and roughly 0 when $l$ is small,
varying smoothly in between.

Here's a handy choice of our squeezing function: the logistic function.

$$
\sigma(l) = \frac{1}{1 - e^{-l}}
$$

Now we can find an actually useful formulation:

$$
\bold w^* = \argmin_{\bold w} \sum_{b \in B} |\sigma(l(\bold w, b)) - y(b)|^2
$$

This is a nice problem, but we have a problem - there's no analytic solution!
Luckily, this problem is convex, smooth, and differentiable - so gradient descent works.
Let's calculate the gradient of our loss function with respect to $\bold w$:

$$
\begin{aligned}
\nabla_{\bold w} \left( \frac{1}{2} \sum_{b \in B} |\sigma(l(\bold w, b)) - y(b)|^2 \right)
    &= \sum_{b \in B} (\sigma(l(\bold w, b)) - y(b)) \nabla_{\bold w}(\sigma(l(\bold w, b))) \\
    &= \sum_{b \in B}
        (\sigma(\bold w^T \bold x(b)) - y(b))
        \cdot \sigma(\bold w^T \bold x(b))
        \cdot (1 - \sigma(\bold w^T \bold x(b)))
        \cdot \bold x(b)
\end{aligned}
$$

(a useful fact about our sigmoid function is that
$\frac{d}{dx}\sigma(x) = \sigma(x) (1 - \sigma(x))$).

We can now use gradient descent to find our ideal value of $\bold w$.

### Sparse vectors

Computing $\bold w^T \bold x(b)$ is by far the most computationally expensive step of gradient
descent.
Additionally, most of the elements of $\bold x(b)$ are going to be zero.
This means we waste a lot of time computing the product of zero and some other number.

We can speed it up by using a sparse representation of our feature vector.
Instead of just a pure array of numbers, we can instead represent $\bold x(b)$ as a sparse vector:
it is instead a list of `(index, value)` pairs.
In Rust, we represent $\bold x(b)$ as a `Vec<(usize, f32)>`.

To save you some grief: here's a quick Rust one-liner for evaluating the inner product:

```rust
fn sparse_inner_product(x: &[(usize, f32)], w: &[f32]) -> f32 {
    x.iter().map(|&(i, value)| w[i] * value).sum()
}
```

## Your assignment

Your assignment is to implement the gradient-calculating algorithm for a chess engine tuner in
Tomato.

1. Compile the engine as-is right now (with un-tuned values) using
   `cargo build --release --bin tomato`.
   Save a copy of the binary in [`target/release/tomato`](target/release/tomato) somewhere else for
   later (we'll call this the untuned binary).

1. Implement the function `compute_gradient()` in [`src/bin/tune.rs`](src/bin/tune.rs).
   This function computes the gradient
   $\nabla_{\bold w} \left( \frac{1}{2} \sum_{b \in B} |\sigma(l(\bold w, b)) - y(b)|^2 \right)$,
   as well as the value of the loss function
   $\sum \_ {b \in B} |\sigma(l(\bold w, b)) - y(b)|^2 $.
   For more details, you can refer to the documentation in the function.

1. Run the tuner by running `cargo run --release --bin tune quiet-labeled.epd`.
   If you completed the previous step right, you should find that the mean squared error constantly
   decreases and that the epoch runs in around 25 milliseconds (if you get it to run faster, let me
   know!)

1. When the tuner is done running, it'll print out some new values for the material evaluation.
   Update the material values by copy-pasting those values in
   [`src/engine/evaluate/material.rs`](src/engine/evaluate/material.rs) and
   [`src/engine/evaluate/pst.rs`](src/engine/evaluate/pst.rs), then recompile the engine using
   `cargo build --release --bin tomato`.

1. Use Cute Chess to run a tournament between the untuned and tuned binaries for Tomato.
   You can use whatever parameters for the tournament that you want, so long as you run at least 100
   matches.
   This will be the longest step of the process.

1. Report the results of the tournament in a file named `report.txt`.
   The contents "Results" popup here is sufficient.

This process should take roughly half an hour of coding, but will require a few hours of just
letting stuff run in the background while you wait for things to happen.
You might get stuck - if so, send me an email at shrimpfried@rice.edu and I'll try to help.

## Grading

This assignment will be graded just a little differently from the others since there's a part that
isn't required from tests.
Here's my rubric:

- 6 points: all tests passing

- 2 points: updated weights in `src/engine/evaluate`

- 2 points: tournament results

If it turns out that I way underestimated the time required for this assignment, I might relax this
rubric a bit so that students don't get totally demolished.
