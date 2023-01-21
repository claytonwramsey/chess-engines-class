/*
  COLL 110, a chess engines class.
  Copyright (C) 2022 Clayton Ramsey.

  The materials for COLL 110 is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The materials for COLL 110 are distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::time::Instant;

use coll110_hw2::movegen::perft;

fn main() {
    let bench_data = [
        (
            // starting position
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            6,
        ),
        (
            // kiwipete
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            5,
        ),
        (
            // endgame
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            5,
        ),
        (
            // unbalanced
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            5,
        ),
        (
            // edwards
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            5,
        ),
        (
            // edwards2
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            5,
        ),
    ];

    let start = Instant::now();
    let nodes = bench_data
        .into_iter()
        .map(|(fen, depth)| perft(fen, depth))
        .sum::<u64>();
    let time_taken = Instant::now() - start;

    println!("Total time taken: {time_taken:?}");
    println!(
        "Final NPS: {}",
        1_000_000 * nodes / time_taken.as_micros() as u64
    );
}
