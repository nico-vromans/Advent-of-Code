#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day03;

impl Solver for Day03 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            // For each line (bank), pick two digits in order to form the largest possible
            // two-digit number, then sum across banks.
            let mut total: u128 = 0;

            for line in input.lines() {
                let s = line.trim();
                if s.is_empty() {
                    continue;
                }

                // Collect digits; ignore any non-digit just in case.
                let digits: Vec<u8> = s
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                    .collect();

                if digits.len() < 2 {
                    // Can't form a two-digit number from this bank.
                    continue;
                }

                // Build suffix maximums of digits to quickly get the best ones place after index i
                let n = digits.len();
                let mut suffix_max: Vec<u8> = vec![0; n + 1];
                // suffix_max[n] = 0 (no digits after the last index)
                for i in (0..n).rev() {
                    suffix_max[i] = suffix_max[i + 1].max(digits[i]);
                }

                let mut best: u16 = 0;
                for i in 0..(n - 1) {
                    let tens = digits[i];
                    let ones = suffix_max[i + 1];
                    let cand: u16 = (tens as u16) * 10 + (ones as u16);
                    if cand > best {
                        best = cand;
                    }
                }

                total += best as u128;
            }

            total
        }

        // For Part 2 (unspecified in README excerpt), return the count of invalid IDs.
        fn part2(input: &str) -> u128 {
            // For each line (bank), pick exactly 12 digits in order to form the
            // lexicographically largest possible 12-digit number, then sum across banks.
            const K: usize = 12;
            let mut total: u128 = 0;

            for line in input.lines() {
                let s = line.trim();
                if s.is_empty() {
                    continue;
                }

                // Collect digits; ignore any non-digit just in case.
                let digits: Vec<u8> = s
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                    .collect();

                let n = digits.len();
                if n < K {
                    // Cannot select exactly K digits; contribute 0.
                    continue;
                }

                // Greedy monotonic stack to keep the largest subsequence of length K.
                // We can remove exactly r = n - K digits.
                let mut remove = n - K;
                let mut stack: Vec<u8> = Vec::with_capacity(n);
                for &d in &digits {
                    while remove > 0 && !stack.is_empty() && stack[stack.len() - 1] < d {
                        stack.pop();
                        remove -= 1;
                    }
                    stack.push(d);
                }
                // If we still have removals left, drop from the end.
                let mut chosen = if stack.len() > K {
                    stack[..K].to_vec()
                } else {
                    stack
                };
                if chosen.len() > K {
                    chosen.truncate(K);
                }

                // Convert the 12-digit sequence to a u128 value and add to total.
                let mut val: u128 = 0;
                for &d in &chosen {
                    val = val * 10 + (d as u128);
                }
                total += val;
            }

            total
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
