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

use coll110_hw1::{popcnt_kernighan, popcnt_lookup, popcnt_naive};

fn main() {
    // products of a prime are probably roughly randomly distributed in bits
    let things_to_count: Vec<u64> = (0..100_000_000).map(|n| n * 87178291199).collect();

    bench("builtin", |x| x.count_ones() as u8, &things_to_count);
    bench("naive", popcnt_naive, &things_to_count);
    bench("kernighan", popcnt_kernighan, &things_to_count);
    bench("lookup", popcnt_lookup, &things_to_count);
}

/// Benchmark a one-counting function.
fn bench(name: &str, count_fn: impl Fn(u64) -> u8, numbers: &[u64]) {
    let start = Instant::now();
    for &num in numbers {
        count_fn(num);
    }
    let end = Instant::now();
    let dur = end.duration_since(start).as_secs_f64();
    println!(
        "{name}: {dur:.4}s ({} counts/sec)",
        (numbers.len() as f64 / dur).round()
    );
}
