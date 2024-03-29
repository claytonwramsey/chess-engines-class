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

//! Primary search algorithms.

use crate::base::game::Game;

use super::evaluate::Eval;

#[allow(dead_code)]
/// Use Principal Variation Search to evaluate the given game to a depth.
///
/// At each node, the search will examine all legal moves and try to find the best line,
/// recursively searching to `depth_to_go` moves deep.
/// It will then return the evaluation of the position.
///
/// # Inputs
///
/// * `PV`: Whether this node is a principal variation node.
///     At the root, this should be `true`.
/// * `game`: The game to search on.
/// * `depth_to_go`: The depth to search the position.
/// * `depth_so_far`: The depth of the recursive stack when this function was called.
///     At the start of the search, `depth_so_far` is 0.
/// * `alpha`: A lower bound on the evaluation of a parent node, in perspective of the player
///     to move.
///     One way of thinking of `alpha` is that it is the best score that the player to move
///     could get if they made a move which did *not* cause `pvs()` to be called in this
///     position.
///     When called externally, `alpha` should be equal to `Eval::MIN`.
/// * `beta`: An upper bound on the evaluation of a parent node, in perspective of the player to
///     move.
///     `beta` can be thought of as the worst score that the opponent of the current player to
///     move could get if they decided not to allow the current player to make a move.
///     When called externally, `beta` should be equal to `Eval::MAX`.
pub fn pvs<const PV: bool>(
    game: &mut Game,
    depth_to_go: u8,
    depth_so_far: u8,
    alpha: Eval,
    beta: Eval,
) -> Eval {
    todo!()
}

#[cfg(test)]
pub mod tests {

    use super::*;

    /// Helper function to search a position at a given depth.
    ///
    /// # Panics
    ///
    /// This function will panic if searching the position fails or the game is invalid.
    fn search_helper(fen: &str, depth: u8) -> Eval {
        let mut g = Game::from_fen(fen).unwrap();
        pvs::<true>(&mut g, depth, 0, Eval::MIN, Eval::MAX)
    }

    /// A helper function which ensures that the evaluation of a position is equal to what we expect
    /// it to be.
    /// It will check both a normal search and a search without the transposition table.
    fn eval_helper(fen: &str, eval: Eval, depth: u8) {
        assert_eq!(search_helper(fen, depth), eval);
    }

    #[test]
    /// Test `PVSearch`'s evaluation of the start position of the game.
    fn eval_start() {
        let eval = search_helper(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            5,
        );
        println!("opening evaluation: {eval}");
    }

    #[test]
    /// A test that the engine can find a mate in 1 move.
    fn mate_in_1() {
        // Rb8# is mate in one
        let eval = search_helper("3k4/R7/1R6/5K2/8/8/8/8 w - - 0 1", 2);
        assert_eq!(eval, Eval::mate_in(1));
    }

    #[test]
    /// A test that shows the engine can find a mate in 4 plies, given enough depth.
    fn mated_in_4_ply() {
        // because black, the player to move, is getting mated, the evaluation is negative here
        eval_helper("3k4/R7/8/5K2/3R4/8/8/8 b - - 0 1", -Eval::mate_in(4), 6);
    }

    #[test]
    /// Test that White can force a draw by taking a piece.
    fn saving_draw() {
        let eval = search_helper("8/8/4k3/8/2Kq4/8/8/8 w - - 0 1", 2);
        assert_eq!(eval, Eval::DRAW);
    }

    #[test]
    /// Test that a position where White is up a piece is evaluated positively.
    fn free_money() {
        let eval = search_helper(
            "rnbqkbnr/pppppppp/8/8/8/6N1/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            2,
        );
        assert!(eval > Eval::centipawns(200));
    }

    #[test]
    /// Test that the engine can find a mate in 5 ply.
    fn mate_in_5_ply() {
        eval_helper(
            "4brk1/7p/4b1p1/8/2P5/1P5Q/1BP2PPP/5RK1 w - - 1 2",
            Eval::mate_in(5),
            6,
        );
    }

    #[test]
    /// Test that the engine can find a mate in 7 ply.
    fn mate_in_7_ply() {
        eval_helper(
            "4brk1/5b1p/6p1/8/8/1PP4Q/1BP2PPP/5RK1 w - - 0 1",
            Eval::mate_in(7),
            8,
        );
    }
}
