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

//! Shared data types and useful basic definitions found across the entire Tomato engine.

// Many module elements are re-exported to make names more ergonomic to access.

mod bitboard;
pub use bitboard::Bitboard;

mod board;
pub use board::Board;

mod castling;
use castling::CastleRights;

mod color;
pub use color::Color;

mod direction;
pub use direction::Direction;

pub mod game;

mod magic;
pub use magic::MAGIC;

pub mod movegen;

mod moves;
pub use moves::Move;

mod piece;
pub use piece::Piece;

mod square;
pub use square::Square;

mod zobrist;
