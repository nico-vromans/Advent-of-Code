#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day02;

impl Solver for Day02 {
    fn solve(&self, input: &str) -> Vec<String> {
        // Parse input like "a-b,c-d,..." possibly wrapped across lines/spaces
        fn parse_ranges(input: &str) -> impl Iterator<Item = (u128, u128)> + '_ {
            input
                .split(',')
                .filter_map(|tok| {
                    let s: &str = tok.trim();
                    if s.is_empty() {
                        return None;
                    }
                    let (a, b) = s.split_once('-')?;
                    let lo: u128 = a.trim().parse().ok()?;
                    let hi: u128 = b.trim().parse().ok()?;
                    Some((lo.min(hi), lo.max(hi)))
                })
        }

        #[inline]
        fn ceil_div(a: u128, b: u128) -> u128 {
            if a == 0 { 0 } else { (a - 1) / b + 1 }
        }

        // Returns (count, sum) of all numbers in [lo, hi] that are a repeated pair: XYXY
        fn sum_repeated_pairs_in_range(lo: u128, hi: u128) -> (u128, u128) {
            let mut count: u128 = 0;
            let mut sum: u128 = 0;

            // Iterate half-length k (total digits = 2k). For each k, numbers are n = x * (10^k + 1)
            // with x in [10^(k-1), 10^k - 1].
            let mut pow10_km1: u128 = 1; // 10^(k-1)
            let mut pow10_k: u128 = 10;  // 10^k
            loop {
                let s: u128 = pow10_k + 1; // multiplier (10^k + 1)
                let min_x: u128 = pow10_km1;
                let max_x: u128 = pow10_k - 1;
                // Smallest number for this k
                let min_n: u128 = s.saturating_mul(min_x);
                if min_n > hi {
                    break;
                }

                // Determine x range that maps into [lo, hi]
                let x_lo: u128 = min_x.max(ceil_div(lo, s));
                let x_hi: u128 = max_x.min(hi / s);
                if x_lo <= x_hi {
                    let n_terms: u128 = x_hi - x_lo + 1;
                    // Sum of x from x_lo..=x_hi: (x_lo + x_hi) * n_terms / 2
                    let (a, n) = if (x_lo + x_hi) % 2 == 0 {
                        ((x_lo + x_hi) / 2, n_terms)
                    } else {
                        (x_lo + x_hi, n_terms / 2)
                    };
                    let sum_x: u128 = a * n;
                    count += n_terms;
                    sum += s * sum_x;
                }

                // Advance k -> k+1
                pow10_km1 = pow10_km1.saturating_mul(10);
                pow10_k = pow10_k.saturating_mul(10);
            }

            (count, sum)
        }

        fn part1(input: &str) -> u128 {
            let mut total_sum: u128 = 0;
            for (lo, hi) in parse_ranges(input) {
                let (_, s) = sum_repeated_pairs_in_range(lo, hi);
                total_sum += s;
            }
            total_sum
        }

        // For Part 2 (unspecified in README excerpt), return the count of invalid IDs.
        fn part2(input: &str) -> u128 {
            unimplemented!("part 2 not ready yet")
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
