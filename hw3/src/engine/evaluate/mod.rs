/*
  COLL 110, a chess engines class.
  Copyright (C) 2022 Clayton Ramsey.

  The course materials for COLL 110 are free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The course materials for COLL 110 are distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

//! Static evaluation of positions.
//!
//! Of all the parts of a chess engine, static evaluation is arguably the most important.
//! Every leaf of the search is statically evaluated, and based on the comparisons of each
//! evaluation, the full minimax search is achieved.
//!
//! Tomato uses a classical approach to static evaluation: the final evaluation is the sum of a
//! number of rules.
//! Each rule contributes a quantity to the evaluation.
//!
//! Also like other engines, Tomato uses a "tapered" evaluation: rules are given different weights
//! at different phases of the game.
//! To prevent sharp changes in evaluation as the phase blends, a "midgame" and "endgame" evaluation
//! is created, and then the final evaluation is a linear combination of those two.

use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::base::{game::Game, Board, Color, Piece};

pub mod material;
pub mod pst;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(C)]
/// A wrapper for the evaluation of a position.
/// The higher an evaluation is, the better the position is for the player to move.
/// An evaluation of 0 is a draw.
/// Internally, an evaluation is a 16-bit signed interger.
/// The integer value is 1/100 of a pawn (so if the internal value is +200, the position is +2
/// pawns for the player).
///
/// Values > 29,000 are reserved for mates.
/// 30,000 is player to mate in 0 (e.g. White has won the game), 29,999 is player to mate in 1
/// (player  will play their move and mate), 29,998 is player to mate in 1, with opponent to move
/// (opponent will play their move, then player will play their move to mate) and so on.
/// Values of < -29,000 are reserved for black mates, likewise.
///
/// # Examples
///
/// ```
/// use coll110_hw3::engine::evaluate::Eval;
/// let mate_eval = Eval::mate_in(3);
/// let draw_eval = Eval::DRAW;
/// assert!(mate_eval > draw_eval);
/// ```
pub struct Eval(i16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
/// A `Score` is a pair of two `Evals` - one for the midgame and one for the endgame.
/// The values inside of a `Score` should never be mate values.
pub struct Score {
    /// The midgame-only evaluation of a position.
    pub mg: Eval,
    /// The endgame-only evaluation of a position.
    pub eg: Eval,
}

/// The cutoff for pure midgame material.
pub const MG_LIMIT: Eval = Eval::centipawns(2408);

/// The cutoff for pure endgame material.
pub const EG_LIMIT: Eval = Eval::centipawns(1348);

#[must_use]
#[allow(clippy::module_name_repetitions)]
/// Heuristically evaluate a leaf position on a game.
/// The returned value is in perspective of the player to move, so it is positive if the player to
/// move is winning and negative if they are losing.
pub fn leaf_evaluate(g: &Game) -> Eval {
    let b = g.board();
    let mg_npm = {
        let mut total = Eval::DRAW;
        for pt in Piece::NON_PAWNS {
            total += material::value(pt).mg * b[pt].len();
        }
        total
    };
    let phase = calculate_phase(mg_npm);
    (material::evaluate(b) + pst::evaluate(b))
        .blend(phase)
        .in_perspective(g.board().player)
}

#[must_use]
/// Get a blending float describing the current phase of the game.
/// Will range from 0 (full endgame) to 1 (full midgame).
///
/// # Examples
///
/// ```
/// use coll110_hw3::base::Board;
/// use coll110_hw3::engine::evaluate::phase_of;
///
/// assert!(phase_of(&Board::new()).eq(&1.0));
/// ```
pub fn phase_of(b: &Board) -> f32 {
    // amount of non-pawn material in the board, under midgame values
    let mg_npm = {
        let mut total = Eval::DRAW;
        for pt in Piece::NON_PAWNS {
            total += material::value(pt).mg * b[pt].len();
        }
        total
    };

    calculate_phase(mg_npm)
}
#[must_use]
/// Get a blending float describing the current phase of the game.
/// Will range from 0 (full endgame) to 1 (full midgame).
/// `mg_npm` is the amount of midgame non-pawn material on the board.
pub fn calculate_phase(mg_npm: Eval) -> f32 {
    let bounded_npm = mg_npm.clamp(EG_LIMIT, MG_LIMIT);

    (EG_LIMIT - bounded_npm).float_val() / (EG_LIMIT - MG_LIMIT).float_val()
}

impl Eval {
    /// An evaluation which is smaller than every other "normal" evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use coll110_hw3::engine::evaluate::Eval;
    ///
    /// assert!(Eval::MIN < Eval::MATED);
    /// assert!(Eval::MIN < Eval::DRAW);
    /// assert!(Eval::MIN < Eval::MATING);
    /// assert!(Eval::MIN < Eval::MAX);
    /// ```
    pub const MIN: Eval = Eval(-Eval::MATE_0_VAL - 1000);

    /// An evaluation which is larger than every other "normal" evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use coll110_hw3::engine::evaluate::Eval;
    ///
    /// assert!(Eval::MIN < Eval::MAX);
    /// assert!(Eval::MATED < Eval::MAX);
    /// assert!(Eval::DRAW < Eval::MAX);
    /// assert!(Eval::MATING < Eval::MAX);
    /// ```
    pub const MAX: Eval = Eval(Eval::MATE_0_VAL + 1000);

    /// An evaluation where Black has won the game by mate.
    ///
    /// # Examples
    ///
    /// ```
    /// use coll110_hw3::engine::evaluate::Eval;
    ///
    /// assert!(Eval::MIN < Eval::MATED);
    /// assert!(Eval::MATED < Eval::DRAW);
    /// assert!(Eval::MATED < Eval::MATING);
    /// assert!(Eval::MATED < Eval::MAX);
    /// ```
    pub const MATED: Eval = Eval(-Eval::MATE_0_VAL);

    /// An evaluation where the player to move has won by mate.
    ///
    /// # Examples
    ///
    /// ```
    /// use coll110_hw3::engine::evaluate::Eval;
    ///
    /// assert!(Eval::MIN < Eval::MATING);
    /// assert!(Eval::MATED < Eval::MATING);
    /// assert!(Eval::DRAW < Eval::MATING);
    /// assert!(Eval::MATING < Eval::MAX);
    /// ```
    pub const MATING: Eval = Eval(Eval::MATE_0_VAL);

    /// The evaluation of a drawn position.
    pub const DRAW: Eval = Eval(0);

    /// The internal evaluation of a mate in 0 for White (i.e. White made the mating move on the
    /// previous ply).
    const MATE_0_VAL: i16 = 30_000;

    /// The highest value of a position which is not a mate.
    const MATE_CUTOFF: i16 = 29_000;

    /// The value of one pawn.
    const PAWN_VALUE: i16 = 100;

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// Get an evaluation equivalent to the given pawn value.
    /// Will round down by the centipawn.
    pub fn pawns(x: f64) -> Eval {
        Eval((x * f64::from(Eval::PAWN_VALUE)) as i16)
    }

    #[must_use]
    /// Construct an `Eval` with the given value in centipawns.
    pub const fn centipawns(x: i16) -> Eval {
        Eval(x)
    }

    #[must_use]
    /// Create an `Eval` based on the number of half-moves required for the player to move to mate.
    /// `-Eval::mate_in(n)` will give the oponent to mate in the number of plies.
    pub const fn mate_in(nplies: u8) -> Eval {
        Eval(Eval::MATE_0_VAL - (nplies as i16))
    }

    #[must_use]
    /// Get the value in floating-point pawns of this evaluation.
    pub fn float_val(self) -> f32 {
        f32::from(self.0) / 100.
    }

    #[must_use]
    /// Put this evaluation into the perspective of the given player.
    /// In essence, if the player is Black, the evaluation will be inverted, but if the player is
    /// White, the evaluation will remain the same.
    /// This function is an involution, meaning that calling it twice with the same player will
    /// yield the original evaluation.
    pub const fn in_perspective(self, player: Color) -> Eval {
        match player {
            Color::White => self,
            Color::Black => Eval(-self.0),
        }
    }
}

impl Score {
    /// The score for a position which is completely drawn.
    pub const DRAW: Score = Score::centipawns(0, 0);

    #[must_use]
    /// Create a new `Score` by composing two evaluations together.
    pub const fn new(mg: Eval, eg: Eval) -> Score {
        Score { mg, eg }
    }

    #[must_use]
    /// Create a `Score` directly as a pair of centipawn values.
    pub const fn centipawns(mg: i16, eg: i16) -> Score {
        Score::new(Eval::centipawns(mg), Eval::centipawns(eg))
    }

    #[must_use]
    /// Blend the midgame and endgame
    pub fn blend(self, phase: f32) -> Eval {
        // in test mode, require that the phase is between 0 and 1
        debug_assert!(0. <= phase);
        debug_assert!(phase <= 1.);

        self.mg * phase + self.eg * (1. - phase)
    }
}

impl Display for Eval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 > Eval::MATE_CUTOFF {
            // white to mate
            write!(f, "+M{:.0}", (Eval::MATE_0_VAL - self.0 + 1) / 2)?;
        } else if self.0 < -Eval::MATE_CUTOFF {
            // black to mate
            write!(f, "-M{:.0}", (Eval::MATE_0_VAL + self.0 + 1) / 2)?;
        } else if self.0 == 0 {
            // draw
            write!(f, "00.00")?;
        } else {
            // normal eval
            write!(
                f,
                "{:+2.2}",
                f32::from(self.0) / f32::from(Eval::PAWN_VALUE)
            )?;
        }
        Ok(())
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.mg, self.eg)
    }
}

impl Mul<u8> for Eval {
    type Output = Self;
    fn mul(self, rhs: u8) -> Self::Output {
        Eval(self.0 * i16::from(rhs))
    }
}

impl Mul<i16> for Eval {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self::Output {
        Eval(self.0 * rhs)
    }
}

impl Mul<i8> for Eval {
    type Output = Self;
    fn mul(self, rhs: i8) -> Self::Output {
        Eval(self.0 * i16::from(rhs))
    }
}

impl Mul<f32> for Eval {
    type Output = Self;
    #[allow(clippy::cast_possible_truncation)]
    fn mul(self, rhs: f32) -> Self::Output {
        Eval((f32::from(self.0) * rhs) as i16)
    }
}

impl MulAssign<i16> for Eval {
    fn mul_assign(&mut self, rhs: i16) {
        self.0 *= rhs;
    }
}

impl AddAssign<Eval> for Eval {
    fn add_assign(&mut self, rhs: Eval) {
        self.0 += rhs.0;
    }
}

impl SubAssign<Eval> for Eval {
    fn sub_assign(&mut self, rhs: Eval) {
        self.0 -= rhs.0;
    }
}

impl Add<Eval> for Eval {
    type Output = Self;
    fn add(self, rhs: Eval) -> Eval {
        Eval(self.0 + rhs.0)
    }
}

impl Sub<Eval> for Eval {
    type Output = Self;
    fn sub(self, rhs: Eval) -> Eval {
        Eval(self.0 - rhs.0)
    }
}

impl Neg for Eval {
    type Output = Self;
    fn neg(self) -> Eval {
        Eval(-self.0)
    }
}

impl AddAssign<Score> for Score {
    fn add_assign(&mut self, rhs: Score) {
        self.mg += rhs.mg;
        self.eg += rhs.eg;
    }
}

impl SubAssign<Score> for Score {
    fn sub_assign(&mut self, rhs: Score) {
        self.mg -= rhs.mg;
        self.eg -= rhs.eg;
    }
}

impl Add<Score> for Score {
    type Output = Self;

    fn add(self, rhs: Score) -> Self::Output {
        Score::new(self.mg + rhs.mg, self.eg + rhs.eg)
    }
}

impl Sub<Score> for Score {
    type Output = Self;

    fn sub(self, rhs: Score) -> Self::Output {
        Score::new(self.mg - rhs.mg, self.eg - rhs.eg)
    }
}

impl Mul<i8> for Score {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        Score::new(self.mg * rhs, self.eg * rhs)
    }
}

impl Mul<u8> for Score {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Score::new(self.mg * rhs, self.eg * rhs)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn certainly_endgame() {
        assert_eq!(
            phase_of(&Board::from_fen("8/5k2/6p1/8/5PPP/8/pb3P2/6K1 w - - 0 37").unwrap()),
            0.0
        );
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn certainly_midgame() {
        assert_eq!(phase_of(&Board::default()), 1.0);
    }

    #[test]
    /// Test that multiplying scores doesn't screw up and cause weird overflows.
    fn score_multiply() {
        let s1 = Score::centipawns(-289, 0);
        let s2 = Score::centipawns(-289, -200);
        assert_eq!(s1 * 2i8, Score::centipawns(-578, 0));
        assert_eq!(s2 * 2i8, Score::centipawns(-578, -400));

        assert_eq!(s1 * -2i8, Score::centipawns(578, 0));
        assert_eq!(s2 * -2i8, Score::centipawns(578, 400));
    }
}
