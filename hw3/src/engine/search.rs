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
//!
//! All chess engines do some sort of tree searching, and as a classical engine,
//! Tomato uses a variation of Minimax search.
//! In this case, Tomato uses principal-variation search, which runs in
//! Omega(b^{d/2}) time, so long as the move ordering is correct and causes the
//! most critical moves to be searched first at each depth.
//!
//! At each leaf of the principal-variation search, a second, shorter quiescence
//! search is performed to exhaust all captures in the position, preventing the
//! mis-evaluation of positions with hanging pieces.

use crate::{
    base::{game::Game, movegen::has_moves},
    engine::pick::candidacy,
};

use super::evaluate::Eval;

use super::evaluate::leaf_evaluate;

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
/// * `parent_line`: The principal variation line of the parent position.
///     `parent_line` will be overwritten with the best line found by this search, so long as it
///     achieves an alpha cutoff at some point.
pub fn pvs<const PV: bool>(
    game: &mut Game,
    depth_to_go: u8,
    depth_so_far: u8,
    mut alpha: Eval,
    mut beta: Eval,
) -> Eval {
    if depth_to_go == 0 {
        return leaf_evaluate(game);
    }

    // mate distance pruning
    let lower_bound = -Eval::mate_in(depth_so_far);
    if alpha < lower_bound {
        if beta <= lower_bound {
            return lower_bound;
        }
        alpha = lower_bound;
    }

    let upper_bound = Eval::mate_in(1 + depth_so_far);
    if upper_bound < beta {
        if upper_bound <= alpha {
            return upper_bound;
        }
        beta = upper_bound;
    }

    // detect draws.
    if game.drawn() {
        // required so that movepicker only needs to know about current position, and not about
        // history
        return Eval::DRAW;
    }

    let mut moves_iter = game.get_moves();
    let b = game.board();
    moves_iter.sort_by_cached_key(|&m| -candidacy(b, m));
    let mut best_score = Eval::MIN;

    // The number of moves checked. If this is zero after the move search loop, no moves were
    // played.
    let mut move_count = 0;
    for m in moves_iter {
        move_count += 1;
        game.make_move(m);
        let mut score = Eval::MIN;

        if !PV || move_count > 1 {
            // For moves which are not the first move searched at a PV node, or for moves which
            // are not in a PV node, perform a zero-window search of the position.

            score = -pvs::<false>(
                game,
                depth_to_go - 1,
                depth_so_far + 1,
                -alpha - Eval::centipawns(1),
                -alpha,
            );
        }

        if PV && (move_count == 1 || alpha < score && score < beta) {
            // Either this is the first move on a PV node, or the previous search returned a PV
            // candidate.
            score = -pvs::<true>(game, depth_to_go - 1, depth_so_far + 1, -beta, -alpha);
        }

        let undo_result = game.undo();
        debug_assert!(undo_result.is_ok());

        if score > best_score {
            best_score = score;

            if score > alpha {
                if beta <= score {
                    // Beta cutoff: we found a move that was so good that our opponent would
                    // never have let us play it in the first place.
                    // Therefore, we need not consider the other moves, since we wouldn't be
                    // allowed to play them either.
                    break;
                }

                // to keep alpha < beta, only write to alpha if there was not a beta cutoff
                alpha = score;
            }
        }
    }

    debug_assert!((move_count == 0) ^ has_moves(game.board()));

    if move_count == 0 {
        // No moves were played, therefore this position is either a stalemate or a mate.
        best_score = if game.board().checkers.is_empty() {
            // stalemated
            Eval::DRAW
        } else {
            // mated
            lower_bound
        };
    }

    debug_assert!(Eval::MIN < best_score && best_score < Eval::MAX);

    best_score
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
