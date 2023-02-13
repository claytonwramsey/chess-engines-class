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

//! Code which defines the engine's behavior.
//! Included below are tools for evaluating positions and searching trees.

pub mod evaluate;
mod pick;
mod search;
