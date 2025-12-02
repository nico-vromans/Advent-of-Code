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
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }

        fn lcm(a: u32, b: u32) -> u32 {
            if a == 0 || b == 0 { 0 } else { (a / gcd(a, b)) * b }
        }

        fn prime_factors(mut n: u32) -> Vec<u32> {
            let mut factors = Vec::new();
            let mut d = 2;
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

        // Sum of x * multiplier for x in [min_x, max_x] such that x * multiplier in [lo, hi]
        fn sum_ap_in_range(min_x: u128, max_x: u128, multiplier: u128, lo: u128, hi: u128) -> u128 {
            // We need x * multiplier >= lo  =>  x >= ceil(lo / multiplier)
            let start_limit = (lo + multiplier - 1) / multiplier;
            let start = min_x.max(start_limit);

            // We need x * multiplier <= hi  =>  x <= floor(hi / multiplier)
            let end_limit = hi / multiplier;
            let end = max_x.min(end_limit);

            if start > end {
                return 0;
            }

            let count = end - start + 1;
            // Sum of arithmetic progression: sum(x) * multiplier
            // sum(x) from start to end = (start + end) * count / 2
            // We must be careful with overflow here.
            // Use u128. Since max x has ~20 digits (half of 39), sum(x) can be ~40 digits -> overflow?
            // No, x has length `period_len`. `period_len` <= D/2.
            // If D=38, `period_len`=19. 10^19 fits in u64.
            // sum(x) ~ (10^19 + 10^19) * 10^19 / 2 ~ 10^38. Fits in u128.
            // But if multiplier is large?
            // multiplier ~ 10^(D - period_len).
            // Total sum ~ sum(x) * multiplier ~ 10^38.
            // It should be close to the limit but fit.
            
            // To be safe with intermediate calculation (start + end) * count:
            // (start + end) is 2*10^19. count is 10^19. Product 10^38.
            // Fits in u128 (max 3.4e38).
            
            let sum_x = if (start + end) % 2 == 0 {
                 ((start + end) / 2) * count
            } else {
                 // One of them must be even, and u128 is large enough.
                 ((start + end) * count) / 2
            };
            
            sum_x.wrapping_mul(multiplier)
        }

        fn sum_invalid_ids_in_range(lo: u128, hi: u128) -> u128 {
            let mut total: u128 = 0;

            let lo_s = lo.to_string();
            let hi_s = hi.to_string();
            let min_len = lo_s.len();
            let max_len = hi_s.len();

            // Iterate over total length D
            for d in min_len..=max_len {
                // Check if this length overlaps with [lo, hi]
                // Smallest number with length d: 10^(d-1)
                // Largest number with length d: 10^d - 1
                
                // Optimization: Primes of d
                let primes = prime_factors(d as u32);
                if primes.is_empty() {
                     // Length 1 has no prime factors, no period <= 1/2 (since 1/2 < 1).
                     continue;
                }

                // Inclusion-Exclusion Principle
                // Union of properties P_p: periodic with base length d/p
                // We iterate over non-empty subsets of primes.
                let n_primes = primes.len();
                let n_subsets = 1 << n_primes;

                for i in 1..n_subsets {
                    let mut subset_lcm = 1;
                    let mut subset_size = 0;
                    for bit in 0..n_primes {
                        if (i >> bit) & 1 == 1 {
                            subset_lcm = lcm(subset_lcm, primes[bit]);
                            subset_size += 1;
                        }
                    }

                    // The property is "periodic with block size D / subset_lcm"
                    // Repetition count m = subset_lcm.
                    // Actually, if we take LCM of prime factors, say {p1, p2}.
                    // We consider numbers formed by repeating a block of size D/lcm(p1, p2).
                    // The block repeats lcm(p1, p2) times.
                    // Wait. If we intersect "repeat p1 times" and "repeat p2 times".
                    // "Repeat p1 times" means block size D/p1.
                    // "Repeat p2 times" means block size D/p2.
                    // Intersection is numbers that have period D/p1 AND D/p2.
                    // This means period gcd(D/p1, D/p2) = D / lcm(p1, p2).
                    // So block size is `period_len` = D / subset_lcm.
                    // The number of repeats is `subset_lcm`.
                    
                    let period_len = (d as u32 / subset_lcm) as u32;
                    let repeats = subset_lcm; // This is >= 2 since subset is non-empty and primes >= 2.
                    
                    // Calculate multiplier: (10^D - 1) / (10^period_len - 1)
                    // We can compute powers of 10.
                    // Note: 10^D fits in u128 only if D < 39.
                    // If D=39, 10^39 overflows u128.
                    // But max u128 is ~3.4e38. So D can be at most 38.
                    // 10^38 fits. 10^39 does not.
                    // If input ranges go up to u128::MAX, we might have issues with 10^D.
                    // However, parse_ranges uses u128, so max input is u128::MAX.
                    // If hi is u128::MAX, len is 39.
                    // We need 10^39.
                    // Workaround: Calculate multiplier without full 10^D?
                    // multiplier = 1 + 10^L + 10^2L + ...
                    // Iterative calculation.
                    
                    let mut multiplier: u128 = 0;
                    // We need to sum 10^(k * period_len) for k in 0..repeats
                    // Be careful with overflow if D=39.
                    // But we only need multiplier. multiplier < 10^D.
                    // If 10^D > u128::MAX, multiplier might still fit if we are lucky?
                    // No, multiplier is roughly 10^(D - period_len).
                    // If D=39, period_len=1, multiplier ~ 10^38. Fits.
                    // If D=39, period_len=13, multiplier ~ 10^26. Fits.
                    
                    let ten_pow_period = 10u128.pow(period_len);
                    for _ in 0..repeats {
                        // multiplier = multiplier * 10^period + 1
                        // But standard geometric series order: 1 + 10^L + ...
                        // = 1 + 10^L ( 1 + ... )
                        // Let's accumulate.
                        multiplier = multiplier.checked_mul(ten_pow_period).and_then(|x| x.checked_add(1)).unwrap_or(u128::MAX);
                        // Note: the sequence is 1, 101, 10101...
                        // If we do `mult * 10^L + 1`, we get `1` -> `1*100+1=101` -> `101*100+1=10101`. Correct.
                        // BUT for 123123 (period 3, repeats 2), L=3.
                        // mult = 1.
                        // k=0: mult=1.
                        // k=1: mult = 1 * 1000 + 1 = 1001.
                        // Result 123 * 1001 = 123123. Correct.
                    }
                    
                    // Range for base number x (length period_len):
                    // [10^(period_len-1), 10^period_len - 1]
                    let min_x = 10u128.pow(period_len - 1);
                    let max_x = 10u128.pow(period_len) - 1;

                    let term = sum_ap_in_range(min_x, max_x, multiplier, lo, hi);

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
