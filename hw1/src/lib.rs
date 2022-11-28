/// Naively perform a population count by scanning every bit forward and keeping a running total of
/// all ones.
///
/// # Examples
///
/// ```
/// use coll110_hw1::popcnt_naive;
///
/// assert_eq!(popcnt_naive(0), 0);
/// assert_eq!(popcnt_naive(0xFF), 8);
/// assert_eq!(popcnt_naive(!0), 64);
/// ```
pub fn popcnt_naive(x: u64) -> u8 {
    todo!()
}

/// Perform a population count by Brian Kernighan's method.
///
/// # Examples
///
/// ```
/// use coll110_hw1::popcnt_kernighan;
///
/// assert_eq!(popcnt_kernighan(0), 0);
/// assert_eq!(popcnt_kernighan(0xFF), 8);
/// assert_eq!(popcnt_kernighan(!0), 64);
/// ```
pub fn popcnt_kernighan(mut x: u64) -> u8 {
    todo!()
}

/// Perform a population count by looking up the population for each byte in the bitboard and
/// summing them.
///
/// The lookup table is already provided for you as `BYTE_COUNT_LOOKUP`.
/// You may find the function `u64::to_le_bytes` to be useful.
///
/// # Examples
///
/// ```
/// use coll110_hw1::popcnt_lookup;
///
/// assert_eq!(popcnt_lookup(0), 0);
/// assert_eq!(popcnt_lookup(0xFF), 8);
/// assert_eq!(popcnt_lookup(!0), 64);
/// ```
pub fn popcnt_lookup(x: u64) -> u8 {
    /// A lookup table for the population count of an 8-bit integer.
    /// For each index `i`, `BYTE_COUT_LOOKUP[i]` is the population count of `i`.
    const BYTE_COUNT_LOOKUP: [u8; 256] = {
        let mut lookup_table = [0; 256];

        // unfortunately, for-loops are not support in constant expressions, so we get to deal with
        // this
        let mut i = 0;
        while i < 256 {
            lookup_table[i] = i.count_ones() as u8;
            i += 1;
        }

        lookup_table
    };
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function for testing the correctness of a popcount function.
    ///
    /// # Panics
    ///
    /// This function will panic if `function` is not correctly implemented as a population count.
    fn correctness_help(function: impl Fn(u64) -> u8) {
        for idx in 0u64..100_000 {
            let num = idx.wrapping_mul(108086391056891903);
            assert_eq!(function(num), num.count_ones() as u8);
        }
    }

    #[test]
    fn naive() {
        correctness_help(popcnt_naive);
    }

    #[test]
    fn kernighan() {
        correctness_help(popcnt_kernighan);
    }

    #[test]
    fn lookup() {
        correctness_help(popcnt_lookup);
    }
}
