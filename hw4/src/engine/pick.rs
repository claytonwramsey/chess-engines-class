/*
  Tomato, a UCI-compatible chess engine.
  Copyright (C) 2022 Clayton Ramsey.

  Tomato is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  Tomato is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

//! Move selection and phased generation.
//!
//! In order to search effectively, all alpha-beta searches require an effective move ordering which
//! puts the best moves first.
//! This move ordering is the move picker's job.

use crate::base::{Board, Move};

use super::evaluate::{material, Eval, Score};

/// Create an estimate for how good a move is.
/// `delta` is the PST difference created by this move.
/// Requires that `m` must be a legal move in `b`.
///
/// # Panics
///
/// This function may panic if the given move is illegal.
pub fn candidacy(b: &Board, m: Move, delta: Score, phase: f32) -> Eval {
    let mover_type = b.type_at_square(m.from_square()).unwrap();

    // Worst case, we don't keep the piece we captured
    let mut worst_case_delta = delta;
    let mover_value = material::value(mover_type);
    worst_case_delta -= mover_value;
    worst_case_delta.blend(phase)
}
