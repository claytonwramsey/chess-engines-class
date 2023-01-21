# Homework 1 Errata

This is a list of errors made on the part of me (and on GitHub, mostly).
Hopefully this will clear up some confusion - sorry!

I've published a PDF with the amended README in Files.

## GitHub sucks at math rendering

GitHub ostensibly supports LaTeX rendering via mathjax.
This is a lie.
GitHub actually supports a small subset of LaTeX expressions, and it seems as though we've run up
against a wall.

So far, we've uncovered two bugs:

- Equations in tables aren't rendered as equations.
  Instead, it just displays the original LaTeX source.

- GitHub thinks that curly braces (`{` or `}`) are a suggestion - even when they've been escaped
  (via `\{` and `\}`).
  As a result, it just doesn't render those curly braces online.

You can just look at the PDF from now on.

## My bad description of bitboards

I originally wrote the following in the README:

> Let $S \in \{0, 1\}^{64}$.

I then immediately did not use that definition correctly later in the document.
Instead, you should read a set of squares as the following:

> Let $S \subseteq \{0, 1, \cdots 63\}$.

## Off-by-one in the naive method

I had an off-by-one error in the mathematical description of the naive implementation of population
count.

I originally wrote:

$$
\text{Popcount}(BB(S)) = \sum_{s = 0}^{64} I(s \in S)
$$

This is incorrect because $\sum$-notation is upper-bound-inclusive.
I should have written:

$$
\text{Popcount}(BB(S)) = \sum_{s = 0}^{63} I(s \in S)
$$

## Incorrect class number in license notification

At the top of the source files, I included a license notification for the GNU GPLv3.

In that notification, I wrote the following:

```text
The materials for COLL 100 are distributed in the hope that it will be useful,
```

This class is COLL 110, not COLL 100!
