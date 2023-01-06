/*
  COLL 110, a chess engines class.
  Copyright (C) 2022 Clayton Ramsey.

  The materials for COLL 110 is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The materials for COLL 100 are distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

//! Generation and verification of legal moves in a position.

#[cfg(test)]
mod tests;

use std::mem::transmute;

use super::{bitboard::Bitboard, Board, Color, Direction, Move, Piece, Square, MAGIC};

#[must_use]
/// Perform a performance test on the move generator.
/// Returns the number of independent paths to a leaf reachable in `depth` plies from a board with
/// starting position `fen`.
///
/// # Inputs
///
/// - `fen`: A string containing the Forsyth-Edwards notation (FEN) representation of a board.
/// - `depth`: The depth to search.
///
/// # Examples
///
/// ```
/// use coll110_hw2::movegen::perft;
///
/// // Use the starting position FEN and calculate the number of legal moves.
///
/// // There is only ever one position reachable in zero moves.
/// assert_eq!(perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 0), 1);
///
/// // There are 20 legal moves in the starting board position.
/// assert_eq!(perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 1), 20);
///
/// // There are 400 legal opening lines at depth 2.
/// assert_eq!(perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 2), 400);
/// ```
///
/// # Panics
///
/// This function may panic if `fen` is not a valid FEN string.
pub fn perft(fen: &str, depth: u8) -> u64 {
    todo!()
}

/// A lookup table for the legal squares a knight to move to from a given square.
///
/// # Examples
///
/// ```
/// use coll110_hw2::{movegen::KNIGHT_MOVES, Square, Bitboard};
///
/// let mut knight_attacks_a1 = Bitboard::EMPTY
///     .with_square(Square::C2)
///     .with_square(Square::B3);
///
/// assert_eq!(KNIGHT_MOVES[Square::A1 as usize], knight_attacks_a1);
/// ```
// bob seger
pub const KNIGHT_MOVES: [Bitboard; 64] = create_step_attacks(&Direction::KNIGHT_STEPS, 2);

/// A lookup table for the legal squares a king can move to from a given square.
///
/// # Examples
///
/// ```
/// use coll110_hw2::{movegen::KING_MOVES, Square, Bitboard};
///
/// let mut king_attacks_a1 = Bitboard::EMPTY
///     .with_square(Square::A2)
///     .with_square(Square::B1)
///     .with_square(Square::B2);
///
/// assert_eq!(KING_MOVES[Square::A1 as usize], king_attacks_a1);
/// ```
pub const KING_MOVES: [Bitboard; 64] = create_step_attacks(&Direction::KING_STEPS, 1);

/// A lookup table for the legal squares a pawn can attack if it is on a given square and of a given
/// color.
///
/// `PAWN_ATTACKS[0]` is a lookup table for the squares that a white pawn can attack, while
/// `PAWN_ATTACKS[1]` is a lookup table for the squares that a black pawn can attack.
///
/// This table does not include squares that pawns can move to by pushing forward.
///
/// # Examples
///
/// ```
/// use coll110_hw2::{movegen::PAWN_ATTACKS, Color, Square, Bitboard};
///
/// let mut attacked_squares = Bitboard::EMPTY
///     .with_square(Square::A4)
///     .with_square(Square::C4);
///
/// // A white pawn on B3 can attack squares A4 and C4.
/// assert_eq!(PAWN_ATTACKS[Color::White as usize][Square::B3 as usize], attacked_squares);
/// // A black pawn on B5 can attack squares A4 and C4.
/// assert_eq!(PAWN_ATTACKS[Color::Black as usize][Square::B5 as usize], attacked_squares);
/// ```
pub const PAWN_ATTACKS: [[Bitboard; 64]; 2] = [
    create_step_attacks(&[Direction::NORTHEAST, Direction::NORTHWEST], 1),
    create_step_attacks(&[Direction::SOUTHEAST, Direction::SOUTHWEST], 1),
];

/// Get the step attacks that could be made by moving in `dirs` from each point in the square.
///
/// Exclude the steps that travel more than `max_dist` (this prevents overflow around the edges of
/// the board).
const fn create_step_attacks(dirs: &[Direction], max_dist: u8) -> [Bitboard; 64] {
    let mut attacks = [Bitboard::EMPTY; 64];
    let mut i = 0;
    #[allow(clippy::cast_possible_truncation)]
    while i < attacks.len() {
        // SAFETY: we know that `attacks` is 64 elements long, which is the number of
        // `Square`s, so we will not create an illegal variant.
        let sq: Square = unsafe { transmute(i as u8) };
        let mut j = 0;
        #[allow(clippy::cast_sign_loss)]
        while j < dirs.len() {
            let dir = dirs[j];
            let target_sq_disc = sq as i8 + dir.0;
            if target_sq_disc < 0 || 64 <= target_sq_disc {
                // square is out of bounds
                j += 1;
                continue;
            }
            let target_sq: Square = unsafe { transmute((sq as i8 + dir.0) as u8) };
            if target_sq.chebyshev_to(sq) <= max_dist {
                attacks[i] = attacks[i].with_square(target_sq);
            }
            j += 1;
        }
        // sanity check that we added only two attacks
        debug_assert!(attacks[i].len() as usize <= dirs.len());
        i += 1;
    }

    attacks
}

#[inline(always)]
#[must_use]
/// Get the legal moves in a board.
///
/// `get_moves()` will make no regard to whether the position is drawn by
/// repetition, 50-move-rule, or by insufficient material.
///
/// # Examples
///
/// ```
/// use coll110_hw2::{Board, movegen::get_moves};
///
/// let b = Board::new();
/// for m in get_moves(&b) {
///     let mut bcopy = b.clone();
///     bcopy.make_move(m);
/// }
/// ```
pub fn get_moves(b: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let in_check = !b.checkers.is_empty();

    if in_check {
        evasions(b, &mut moves);
    } else {
        non_evasions(b, &mut moves);
    };

    moves
}

#[inline(always)]
#[must_use]
/// Determine whether a square is attacked by the pieces of a given color in a position.
/// Squares which are threatened by only non-capture moves (i.e. pawn-pushes) will not qualify as
/// attacked.
///
/// # Examples
///
/// ```
/// use coll110_hw2::{Board, Square, Color, movegen::is_square_attacked_by};
///
/// let b = Board::new();
/// assert!(is_square_attacked_by(&b, Square::E2, Color::White));
/// assert!(!is_square_attacked_by(&b, Square::E4, Color::White));
/// ```
pub fn is_square_attacked_by(board: &Board, sq: Square, color: Color) -> bool {
    !square_attackers(board, sq, color).is_empty()
}

#[inline(always)]
/// Enumerate the legal moves a player of the given color would be able to make if it were their
/// turn to move, assuming the player's king is not in check.
///
/// Requires that the player to move's king is not in check.
fn non_evasions(b: &Board, moves: &mut Vec<Move>) {
    let target_sqs = !b[b.player];

    let mut pawn_targets = target_sqs;
    if let Some(ep_sq) = b.en_passant_square {
        pawn_targets.insert(ep_sq);
    }
    pawn_assistant(b, moves, pawn_targets);

    normal_piece_assistant(b, moves, target_sqs);

    // generate king moves
    castles(b, moves);
    king_move_non_castle(b, moves, target_sqs);
}

/// Enumerate the legal moves a player of the given color would be able to make if it were their
/// turn to move, assuming the player's king is in check.
///
/// Requires that the player to move's king is in check.
fn evasions(b: &Board, moves: &mut Vec<Move>) {
    let player = b.player;
    let king_sq = b.king_sqs[player as usize];

    // only look at non-king moves if we are not in double check
    if b.checkers.has_single_bit() {
        // SAFETY: We checked that the square is nonzero.
        let checker_sq = unsafe { Square::unsafe_from(b.checkers) };
        // Look for blocks or captures
        let target_sqs = !b[b.player] & Bitboard::between(king_sq, checker_sq) | b.checkers;

        let mut pawn_targets = target_sqs;
        if let Some(ep_sq) = b.en_passant_square {
            // can en passant save us from check?
            let ep_attacker_sq = ep_sq - player.pawn_direction();
            if b.checkers.contains(ep_attacker_sq) {
                pawn_targets.insert(ep_sq);
            }
        }

        pawn_assistant(b, moves, pawn_targets);
        normal_piece_assistant(b, moves, target_sqs);
    }

    let king_targets = !b[b.player];
    king_move_non_castle(b, moves, king_targets);
}

#[inline(always)]
#[must_use]
/// Get the attackers of a given color on a square as a `Bitboard` representing the squares of the
/// attackers.
///
/// # Examples
///
/// ```
/// use coll110_hw2::{Bitboard, Board, Square, Color, movegen::square_attackers};
///
/// let b = Board::new();
/// let attackers = Bitboard::EMPTY
///     .with_square(Square::E1)
///     .with_square(Square::D1)
///     .with_square(Square::F1)
///     .with_square(Square::G1);
///
/// assert_eq!(square_attackers(&b, Square::E2, Color::White), attackers);
/// ```
pub fn square_attackers(board: &Board, sq: Square, color: Color) -> Bitboard {
    square_attackers_occupancy(board, sq, color, board.occupancy())
}

/// Same functionality as `square_attackers`, but uses the provided `occupancy` bitboard (as
/// opposed to the board's occupancy.)
fn square_attackers_occupancy(
    board: &Board,
    sq: Square,
    color: Color,
    occupancy: Bitboard,
) -> Bitboard {
    let mut attackers = Bitboard::EMPTY;
    let color_bb = board[color];
    // Check for pawn attacks
    let pawn_vision = PAWN_ATTACKS[!color as usize][sq as usize];
    attackers |= pawn_vision & board[Piece::Pawn];

    // Check for knight attacks
    let knight_vision = KNIGHT_MOVES[sq as usize];
    attackers |= knight_vision & board[Piece::Knight];

    let queens_bb = board[Piece::Queen];

    // Check for rook/horizontal queen attacks
    let rook_vision = MAGIC.rook_attacks(occupancy, sq);
    attackers |= rook_vision & (queens_bb | board[Piece::Rook]);

    // Check for bishop/diagonal queen attacks
    let bishop_vision = MAGIC.bishop_attacks(occupancy, sq);
    attackers |= bishop_vision & (queens_bb | board[Piece::Bishop]);

    // Check for king attacks
    let king_vision = KING_MOVES[sq as usize];
    attackers |= king_vision & board[Piece::King];

    attackers & color_bb
}

/// Generate the moves all pawns can make and populate `moves` with those moves.
/// Only moves which result in a pawn landing on `target` will be generated.
///
/// Moves which capture allies will also be generated.
/// To prevent this, ensure all squares containing allies are excluded from `target`.
fn pawn_assistant(b: &Board, moves: &mut Vec<Move>, target: Bitboard) {
    const NOT_WESTMOST: Bitboard = Bitboard::new(0xFEFE_FEFE_FEFE_FEFE);
    const NOT_EASTMOST: Bitboard = Bitboard::new(0x7F7F_7F7F_7F7F_7F7F);

    let board = &b;
    let player = b.player;
    let allies = board[player];
    let opponents = board[!player];
    let occupancy = allies | opponents;
    let unoccupied = !occupancy;
    let pawns = board[Piece::Pawn] & allies;
    let rank8 = player.pawn_promote_rank();
    let not_rank8 = !rank8;
    let rank3 = match player {
        Color::White => Bitboard::new(0x0000_0000_00FF_0000),
        Color::Black => Bitboard::new(0x0000_FF00_0000_0000),
    };
    let direction = player.pawn_direction();
    let doubledir = 2 * direction;
    let unpinned = !board.pinned;
    let king_sq = board.king_sqs[player as usize];
    let king_file_mask = Bitboard::vertical(king_sq);
    // pawn captures

    // Pin masks for capture movement
    let (west_pin_diag, east_pin_diag) = match b.player {
        Color::White => (
            Bitboard::anti_diagonal(king_sq),
            Bitboard::diagonal(king_sq),
        ),
        Color::Black => (
            Bitboard::diagonal(king_sq),
            Bitboard::anti_diagonal(king_sq),
        ),
    };

    let capture_mask = opponents & target;

    // prevent pawns from capturing by wraparound
    let west_capturers = pawns & NOT_WESTMOST & (unpinned | west_pin_diag);
    let east_capturers = pawns & NOT_EASTMOST & (unpinned | east_pin_diag);
    // hack because negative bitshift is UB
    let (west_targets, west_direction, east_targets, east_direction) = match player {
        Color::White => (
            west_capturers << 7 & capture_mask,
            Direction::NORTHWEST,
            east_capturers << 9 & capture_mask,
            Direction::NORTHEAST,
        ),
        Color::Black => (
            west_capturers >> 9 & capture_mask,
            Direction::SOUTHWEST,
            east_capturers >> 7 & capture_mask,
            Direction::SOUTHEAST,
        ),
    };

    // promotion captures
    for to_sq in east_targets & rank8 {
        let from_sq = to_sq - east_direction;
        for pt in Piece::PROMOTING {
            let m = Move::promoting(from_sq, to_sq, pt);
            moves.push(m);
        }
    }

    for to_sq in west_targets & rank8 {
        let from_sq = to_sq - west_direction;
        for pt in Piece::PROMOTING {
            let m = Move::promoting(from_sq, to_sq, pt);
            moves.push(m);
        }
    }

    // normal captures
    for to_sq in east_targets & not_rank8 {
        let from_sq = to_sq - east_direction;
        let m = Move::normal(from_sq, to_sq);
        moves.push(m);
    }
    for to_sq in west_targets & not_rank8 {
        let from_sq = to_sq - west_direction;
        let m = Move::normal(from_sq, to_sq);
        moves.push(m);
    }

    // en passant
    if let Some(ep_square) = board.en_passant_square {
        if target.contains(ep_square) {
            let king_sq = b.king_sqs[b.player as usize];
            let enemy = b[!b.player];
            let to_bb = Bitboard::from(ep_square);
            let capture_bb = match player {
                Color::White => to_bb >> 8,
                Color::Black => to_bb << 8,
            };
            let from_sqs = PAWN_ATTACKS[!player as usize][ep_square as usize] & pawns;
            for from_sq in from_sqs {
                let new_occupancy = b.occupancy() ^ Bitboard::from(from_sq) ^ capture_bb ^ to_bb;
                if (MAGIC.rook_attacks(new_occupancy, king_sq)
                    & (b[Piece::Rook] | b[Piece::Queen])
                    & enemy)
                    .is_empty()
                    && (MAGIC.bishop_attacks(new_occupancy, king_sq)
                        & (b[Piece::Bishop] | b[Piece::Queen])
                        & enemy)
                        .is_empty()
                {
                    let m = Move::en_passant(from_sq, ep_square);
                    moves.push(m);
                }
            }
        }
    }

    // pawn forward moves

    // pawns which are not pinned or on the same file as the king can move
    let pushers = pawns & (unpinned | king_file_mask);
    let mut singles = match b.player {
        Color::White => pushers << 8,
        Color::Black => pushers >> 8,
    } & unoccupied;
    let double_candidates = singles & rank3;
    let doubles = match b.player {
        Color::White => double_candidates << 8,
        Color::Black => double_candidates >> 8,
    } & target
        & unoccupied;
    singles &= target;

    // promotion single-moves
    for to_sq in singles & rank8 {
        let from_sq = to_sq - direction;
        for pt in Piece::PROMOTING {
            let m = Move::promoting(from_sq, to_sq, pt);
            moves.push(m);
        }
    }

    // doublemoves
    for to_sq in doubles {
        let m = Move::normal(to_sq - doubledir, to_sq);
        moves.push(m);
    }

    // normal single-moves
    for to_sq in singles & not_rank8 {
        let m = Move::normal(to_sq - direction, to_sq);
        moves.push(m);
    }
}

/// Generate all the moves for a knight, bishop, rook, or queen which end up on the target.
///
/// Moves which capture allies will also be generated.
/// To prevent this, ensure all squares containing allies are excluded from `target`.
fn normal_piece_assistant(b: &Board, moves: &mut Vec<Move>, target: Bitboard) {
    let board = &b;
    let player = b.player;
    let allies = board[player];
    let occupancy = allies | board[!player];
    let queens = board[Piece::Queen];
    let rook_movers = (board[Piece::Rook] | queens) & allies;
    let bishop_movers = (board[Piece::Bishop] | queens) & allies;
    let king_sq = board.king_sqs[player as usize];
    let unpinned = !board.pinned;
    let king_hv = Bitboard::hv(king_sq);
    let king_diags = Bitboard::diags(king_sq);

    // only unpinned knights can move
    for from_sq in board[Piece::Knight] & allies & unpinned {
        for to_sq in KNIGHT_MOVES[from_sq as usize] & target {
            let m = Move::normal(from_sq, to_sq);
            moves.push(m);
        }
    }

    // pinned bishops and queens
    for from_sq in bishop_movers & board.pinned & king_diags {
        for to_sq in MAGIC.bishop_attacks(occupancy, from_sq) & target & king_diags {
            let m = Move::normal(from_sq, to_sq);
            moves.push(m);
        }
    }

    // unpinned bishops and queens
    for from_sq in bishop_movers & unpinned {
        for to_sq in MAGIC.bishop_attacks(occupancy, from_sq) & target {
            let m = Move::normal(from_sq, to_sq);
            moves.push(m);
        }
    }

    // pinned rooks and queens
    for from_sq in rook_movers & board.pinned & king_hv {
        for to_sq in MAGIC.rook_attacks(occupancy, from_sq) & target & king_hv {
            let m = Move::normal(from_sq, to_sq);
            moves.push(m);
        }
    }

    // unpinned rooks and queens
    for from_sq in rook_movers & unpinned {
        for to_sq in MAGIC.rook_attacks(occupancy, from_sq) & target {
            let m = Move::normal(from_sq, to_sq);
            moves.push(m);
        }
    }
}

#[inline(always)]
/// Get the moves that a king could make in a position that are not castles, and append them onto
/// `moves`.
///
/// Only moves which result in a king landing on a square contained by `target` will be generated.
/// If `target` contains a square occupied by an ally, it can generate a move with the ally as the
/// target square.
fn king_move_non_castle(b: &Board, moves: &mut Vec<Move>, target: Bitboard) {
    let king_sq = b.king_sqs[b.player as usize];
    let allies = b[b.player];
    let to_bb = KING_MOVES[king_sq as usize] & !allies & target;
    let king_bb = b[Piece::King] & b[b.player];
    let old_occupancy = b.occupancy();
    for to_sq in to_bb {
        let new_occupancy = (old_occupancy ^ king_bb) | Bitboard::from(to_sq);
        if square_attackers_occupancy(b, to_sq, !b.player, new_occupancy).is_empty() {
            let m = Move::normal(king_sq, to_sq);
            moves.push(m);
        }
    }
}

#[inline(always)]
/// Get the castling moves that the king could make in this position, and append them onto `moves`.
///
/// Will not generate valid moves if the king is in check.
fn castles(b: &Board, moves: &mut Vec<Move>) {
    let player = b.player;
    let occ = b.occupancy();
    let king_sq = b.king_sqs[player as usize];

    // the squares the king must pass through to reach the castled position
    let kingside_castle_passthrough_sqs = match player {
        Color::White => Bitboard::new(0x0000_0000_0000_0060),
        Color::Black => Bitboard::new(0x6000_0000_0000_0000),
    };

    let can_kingside_castle =
        b.castle_rights.kingside(player) && (occ & kingside_castle_passthrough_sqs).is_empty();

    if can_kingside_castle {
        // ignore start sq since we assume the king is not in check
        let passthrough_squares = match player {
            Color::White => [Square::F1, Square::G1],
            Color::Black => [Square::F8, Square::G8],
        };
        if !passthrough_squares
            .iter()
            .any(|&sq| is_square_attacked_by(b, sq, !player))
        {
            let m = Move::castling(king_sq, passthrough_squares[1]);
            moves.push(m);
        }
    }

    // now, repeat the same process for queenside castling

    let queenside_castle_passthrough_sqs = match player {
        Color::White => Bitboard::new(0x0000_0000_0000_000E),
        Color::Black => Bitboard::new(0x0E00_0000_0000_0000),
    };

    let can_queenside_castle =
        b.castle_rights.queenside(player) && (occ & queenside_castle_passthrough_sqs).is_empty();

    if can_queenside_castle {
        // ignore start sq since we assume the king is not in check
        let passthrough_squares = match player {
            Color::White => [Square::D1, Square::C1],
            Color::Black => [Square::D8, Square::C8],
        };
        if !passthrough_squares
            .iter()
            .any(|&sq| is_square_attacked_by(b, sq, !player))
        {
            let m = Move::castling(king_sq, passthrough_squares[1]);
            moves.push(m);
        }
    }
}
