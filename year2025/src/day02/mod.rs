#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day02;

impl Solver for Day02 {
    fn solve(&self, input: &str) -> Vec<String> {
        // Parse input like "a-b,c-d,..." possibly wrapped across lines/spaces
        fn parse_ranges(input: &str) -> impl Iterator<Item = (u128, u128)> + '_ {
            input.split(',').filter_map(|tok: &str| {
                let s: &str = tok.trim();

                if s.is_empty() {
                    return None;
                }

                let (a, b): (&str, &str) = s.split_once('-')?;
                let lo: u128 = a.trim().parse().ok()?;
                let hi: u128 = b.trim().parse().ok()?;

                Some((lo.min(hi), lo.max(hi)))
            })
        }

        // --- Small building blocks shared by Part 1 and Part 2 ---
        // Sum of x * multiplier for x in [min_x, max_x] such that x * multiplier in [lo, hi]
        fn sum_ap_in_range(min_x: u128, max_x: u128, multiplier: u128, lo: u128, hi: u128) -> u128 {
            // We need x * multiplier >= lo  =>  x >= ceil(lo / multiplier)
            let start_limit: u128 = (lo + multiplier - 1) / multiplier;
            let start: u128 = min_x.max(start_limit);

            // We need x * multiplier <= hi  =>  x <= floor(hi / multiplier)
            let end_limit: u128 = hi / multiplier;
            let end: u128 = max_x.min(end_limit);

            if start > end {
                return 0;
            }

            let count: u128 = end - start + 1;
            // Sum of arithmetic progression: sum(x) * multiplier
            // sum(x) from start to end = (start + end) * count / 2
            let sum_x: u128 = if (start + end) % 2 == 0 {
                ((start + end) / 2) * count
            } else {
                ((start + end) * count) / 2
            };

            sum_x.wrapping_mul(multiplier)
        }

        // Compute the decimal-repeat multiplier for a block of length `period_len`
        // repeated exactly `repeats` times: 1 + 10^L + 10^(2L) + ...
        fn repeat_multiplier(period_len: u32, repeats: u32) -> u128 {
            if period_len == 0 || repeats < 2 {
                return 0;
            }

            let step: u128 = 10u128.pow(period_len);
            let mut mult: u128 = 0;

            for _ in 0..repeats {
                mult = mult
                    .checked_mul(step)
                    .and_then(|x| x.checked_add(1))
                    .unwrap_or(u128::MAX);
            }

            mult
        }

        // Sum of all numbers with total length `total_len` that are formed by repeating
        // a base block (length = total_len / repeats) exactly `repeats` times, intersected with [lo, hi].
        fn sum_exact_repeats_for_length(lo: u128, hi: u128, total_len: u32, repeats: u32) -> u128 {
            if repeats < 2 || total_len < repeats || total_len % repeats != 0 {
                return 0;
            }

            let period_len: u32 = total_len / repeats;

            if period_len == 0 {
                return 0;
            }

            let multiplier: u128 = repeat_multiplier(period_len, repeats);
            let min_x: u128 = 10u128.pow(period_len - 1);
            let max_x: u128 = 10u128.pow(period_len) - 1;

            sum_ap_in_range(min_x, max_x, multiplier, lo, hi)
        }

        // Sum of all numbers in [lo, hi] that are made by exactly `repeats` repeats of a block
        // of digits (no restriction on total length beyond divisibility by repeats).
        fn sum_exact_repeats_any_length(lo: u128, hi: u128, repeats: u32) -> u128 {
            if repeats < 2 {
                return 0;
            }

            let min_len: u32 = lo.to_string().len() as u32;
            let max_len: u32 = hi.to_string().len() as u32;
            let mut total: u128 = 0;

            for d in min_len..=max_len {
                if d % repeats == 0 {
                    total = total.wrapping_add(sum_exact_repeats_for_length(lo, hi, d, repeats));
                }
            }

            total
        }

        fn part1(input: &str) -> u128 {
            let mut total_sum: u128 = 0;

            for (lo, hi) in parse_ranges(input) {
                // Part 1: numbers that are exactly two repeats of a base block (XYXY, etc.).
                total_sum = total_sum.wrapping_add(sum_exact_repeats_any_length(lo, hi, 2));
            }

            total_sum
        }

        // For Part 2, return the sum of invalid IDs.
        // An ID is invalid if it is made only of some sequence of digits repeated at least twice.
        fn part2(input: &str) -> u128 {
            let mut total_sum: u128 = 0;
            let ranges: Vec<(u128, u128)> = parse_ranges(input).collect();

            for (lo, hi) in ranges {
                total_sum = total_sum.wrapping_add(sum_invalid_ids_in_range(lo, hi));
            }

            total_sum
        }

        fn gcd(mut a: u32, mut b: u32) -> u32 {
            while b != 0 {
                let t: u32 = b;
                b = a % b;
                a = t;
            }

            a
        }

        fn lcm(a: u32, b: u32) -> u32 {
            if a == 0 || b == 0 {
                0
            } else {
                (a / gcd(a, b)) * b
            }
        }

        fn prime_factors(mut n: u32) -> Vec<u32> {
            let mut factors: Vec<u32> = Vec::new();
            let mut d: u32 = 2;

            while d * d <= n {
                if n % d == 0 {
                    factors.push(d);
                    while n % d == 0 {
                        n /= d;
                    }
                }

                d += 1;
            }

            if n > 1 {
                factors.push(n);
            }

            factors
        }

        fn sum_invalid_ids_in_range(lo: u128, hi: u128) -> u128 {
            let mut total: u128 = 0;
            let lo_s: String = lo.to_string();
            let hi_s: String = hi.to_string();
            let min_len: usize = lo_s.len();
            let max_len: usize = hi_s.len();

            // Iterate over total length D
            for d in min_len..=max_len {
                // Check if this length overlaps with [lo, hi]
                // Smallest number with length d: 10^(d-1)
                // Largest number with length d: 10^d - 1

                // Optimization: Primes of d
                let primes: Vec<u32> = prime_factors(d as u32);

                if primes.is_empty() {
                    // Length 1 has no prime factors, no period <= 1/2 (since 1/2 < 1).
                    continue;
                }

                // Inclusion-Exclusion Principle
                // Union of properties P_p: periodic with base length d/p
                // We iterate over non-empty subsets of primes.
                let n_primes: usize = primes.len();
                let n_subsets: i32 = 1 << n_primes;

                for i in 1..n_subsets {
                    let mut subset_lcm: u32 = 1;
                    let mut subset_size: i32 = 0;

                    for bit in 0..n_primes {
                        if (i >> bit) & 1 == 1 {
                            subset_lcm = lcm(subset_lcm, primes[bit]);
                            subset_size += 1;
                        }
                    }

                    // This subset corresponds to numbers that can be represented as repeating
                    // a block of size D/subset_lcm exactly `subset_lcm` times.
                    let term: u128 = sum_exact_repeats_for_length(lo, hi, d as u32, subset_lcm);

                    if subset_size % 2 == 1 {
                        // Add
                        total = total.wrapping_add(term);
                    } else {
                        // Subtract
                        total = total.wrapping_sub(term);
                    }
                }
            }

            total
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
