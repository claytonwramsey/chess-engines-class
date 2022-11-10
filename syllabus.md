# COLL 110: Artificial Intelligence for Chess

_Clayton Ramsey, Spring 2023_

*cwr3@rice.edu*

Chess was one of the definining problems in artificial intelligence research
throughout the 20th century.
All attempts to solve the game encountered a combinatorial explosion: despite
having only thirty-two pieces and sixty-four squares to put them, there are
more possible games of chess than there are atoms in the universe.
In spite of all this, modern chess engines dramatically outperform human
players.
To this day, chess remains technically unsolved, but these modern engines take
a practical approach to give approximate solutions in a reasonable time.

**_How can we approach computationally intractable problems whose answers
aren't even objectively verifiable?_**
To answer this, we will explore the history of chess engines, from early
alpha-beta search all the way to modern deep learning approaches.
Along the way, we'll examine key algorithms developed in the 20th and 21st
centuries for search, concurrency, and machine learning.
Additionally, we will write parts of our own chess engine in Rust to demonstrate
the effects of particular optimizations and heuristics on engine performance.
For the final project, students will implement a feature of their choosing in
a chess engine and present a summary of their changes and results to the class.

It is recommended that students have some programming experience equivalent to
COMP 140 or CAAM 210 before taking this class.

## Course Objectives and Outcomes

At the end of this class, students should be able to:

1. Develop their own approaches for solving intractable problems using
   heuristics, stochastic search, and statistical methods.
1. Reason about the correctness of programs objectively, both through
   theoretical analysis and empirical testing.
1. Apply their understanding of computer architecture to create high-performance
   software.

I will assess your completion of these objectives through the homework
assignments, quizzes, and the final project.
Quizzes and homeworks are meant to be formative assessments, i.e. ones which
assist you to build up your understanding.
The final project is meant to be a summative assessment which I use to verify
that I've correctly taught the class material.
For more details, refer to the _Grade Policy_ section of this syllabus.

## Office Hours

Office hours are available by appointment.
I will also schedule a regular office hours session once per week; date, time,
and location TBD.

## Grade Policy

Grades will be computed as a percentage.
Any percentage below 65% will receive an Unsatisfactory (U) grade; any grade
equal to or above 65% will receive a Satisfactory (S) grade.
I may lower this requirement over the course of the class, but I will not raise
it.

- **Attendance: 10% of final grade.**
  I will take attendance at every class session.
  The proportion of unexcused absences to the total number of class sections
  will be deducted from the total attendance score.
  A perfect attendance will result in a full 10%; unexcused absence from all
  classes will result in a 0% in this category.
  For further details, refer to the _Absence Policy_ section of this syllabus.

- **Homeworks: 40% of final grade.**
  There will be 4 total homework assignments, each worth 10% of your grade.
  Each homework assignment will be a piece of software development in Rust,
  likely no more than a hundred lines of code each.
  I will provide a set of unit tests for each assignment, and any
  implementation which nontrivially (i.e. without hardcoding results) passes
  all tests will receive a perfect score.
  The score on each homework is the proportion of tests which your software
  passes.
  You will be able to check against these tests prior to your submission.

- **Quizzes: 30% of final grade.**
  We will have weekly quizzes hosted on Canvas.
  The purpose of each quiz is to ensure that you fully understand the content
  of the lectures, i.e. as a formative assessment.
  Accordingly, I will allow an unlimited number of tries on the quizzes.
  I expect that each of the quizzes will take around 5-10 minutes to complete.

  Each quiz will also have a feedback section for me.
  Please be honest in your feedback as I will use to adjust the course as
  needed.

- **Final project: 20% of final grade.**
  For your final project, you must research a feature of chess engines (such
  as tablebase probing, null-move pruning, or similar).
  You may also choose to develop your own original idea.
  You must then implement your feature on an existing engine (one will be
  provided) and demonstrate how your feature improves the performance of the
  engine.

  The goal of this project is for you to demonstrate mastery of all course
  objectives.

## Absence Policy

You will have an attendance score which makes up 10% of your final grade in the
class.
This score is the proportion of class sessions which you either attended or
received an excused absence for.

The possible reasons I may issue an excused absence are for sickness, family or
medical emergency, exigent circumstances (e.g. natural disasters), religious or
secular holidays, school closures, and general "acts of God," in insurance
parlance.
This list of reasons is not exhaustive, and I will try to be understanding if
you need to be excused.
If you will miss or have missed class, please email me as soon as you can and I
will try to issue an excused absence if possible.

## Recommended Texts

No texts are required for this class.
However, I do recommend the following resources to aid you in your work:

- **The Chess Programming Wiki: [https://www.chessprogramming.org]()**.
  An exhaustive wiki covering an array of topics in the field of writing chess
  engines.
  However, some information on the wiki may be outdated or incorrect; I
  recommend that you verify your findings on the wiki with your own research
  whenever possible.
- **Dominik Klein, _Neural Networks for Chess_.**.
  A thorough book covering the broad strokes of the design of chess engines.
  The book takes an emphasis on machine learning (hence the title) but is also
  a good general reference.
- **Talkchess.com: [https://www.talkchess.com]()**.
  The largest forum discussing chess engine programming.
  New developments in the chess programming world can often be found here.

## Special Materials Required

You will need access to a computer which is supported as a target by the Rust
compiler.
Nearly all consumer computers are supported; however, if you happen to use a
20-year-old laptop or a cutting-edge RISC-V supercomputer this may be an issue.

Additionally, this computer will need to have enough compute power for some
relatively intensive computational tasks.

Fortunately, Rice provides both in the form of CLEAR, which is a server cluster
that all Rice students can use.
This means that any student will be able to achieve all the requirements of
the class, even if they do not have the requisite hardware.
Students are not required to use CLEAR.

We will also use the following free software:

- **Cute Chess**: [https://cutechess.com]().
  A GUI for testing chess engines and running tournaments.
- **Rustup**: [https://rustup.rs]().
  A toolchain manager for the Rust programming language.

## Academic Integrity Policy

Collaboration is allowed in all parts of the class.
You may discuss the class material, share code, and work together as needed.
However, I expect you to fully understand everything you submit as an
assignment.

If you directly use another someone else's work, I expect you to credit them as
required.
This credit may be as simple as a comment in your code.
Additionally, code derived from elsewhere (such as StackOverflow or the Chess
Programming wiki) must be cited.
If you are unsure if whether you code you write is plagiarism, go ahead and
cite!
There is no penalty for accidentally citing something you didn't need to.

If you prefer a more precise definition, I allow everything up to and including
[Type 6 collaboration](http://honor.rice.edu/sample-honor-code-language-for-syllabi/)
as defined by the Honor Council:

> Students are allowed to adapt and use the work of other students in writing
> their assignments.
> At this level, the Honor Council requires collaboration credit that indicates
> where adaptations occurred, and where they were sourced from, to prevent
> plagiarism.

You may use any tools for any assignment, including your classmates, your
notes, the course materials, and the Internet.

If you are ever unsure if something is a violation of my academic integrity
policy, do not hesitate to contact me and ask.

## Title IX Notification

I am not a mandatory reporter for Title IX violations, but many faculty and
staff at Rice University are.
Mandatory reporters are required to notify the Title IX office of all
non-consensual interpersonal behavior.
If you experience harassment, discrimination, or violence related to your sex or
gender, seek support through the SAFE office ([safe.rice.edu]()) or email
[titleixsupport@rice.edu]().

## Disability

If you have a disability which will affect your ability to take this course,
please reach out both to me and to the Disability Resource Center:
[https://drc.rice.edu/]() as soon as possible.

I will work with you to make any and all reasonable accomodations for your
disability.

## Course Schedule

This is a tentative lecture list and schedule for the class.
I may change the ordering of lectures, and add or remove some as needed.
I may also push back the due dates for assignments, but I will not move them
earlier.

1. Introduction to chess and syllabus review.

1. Introduction to the Rust programming language.

1. Bitboards and board representation.
   _Homework 1 assigned._

1. Zobrist hashing; move generation, part 1.

1. Move generation, part 2.
   _Homework 1 due.
   Homework 2 assigned._

1. Conventional search algorithms and evaluation.

1. Tuning an engine.
   _Homework 2 due.
   Homework 3 assigned._

1. Pruning.

1. Transposition tables, killer moves, and lazy move generation.
   _Homework 3 due.
   Homework 4 assigned._

1. Monte-Carlo tree search.

1. Deep learning and NNUE.
   _Homework 4 due.
   Final project assigned._

1. "Flex" day 1 (optional extra lectures, which may be consumed by scheduling
   or administrative meddling).

1. "Flex" day 2.

1. Final presentations.
   _Final project due._

## Homework List

The 4 homeworks assigned in the class will serve to help you show your
understanding of the content covered in class (and hopefully give you something
interesting to do!).

1. Bit-hackery: bitboards and hashing.

1. Movegen performance testing (perft) and optimizing move generation.

1. Principal-variation search (and playing against your engine!).

1. Optimizing a search.
