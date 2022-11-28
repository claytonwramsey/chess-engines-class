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
