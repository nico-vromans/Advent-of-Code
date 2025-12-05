#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day05;

impl Solver for Day05 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            // Count how many available ingredient IDs are fresh.
            // Input format: list of inclusive ranges "a-b", then a blank line, then one ID per line.
            let mut ranges: Vec<(u128, u128)> = Vec::new();
            let mut ids: Vec<u128> = Vec::new();

            let mut in_ranges: bool = true;

            for line in input.lines() {
                let s: &str = line.trim();
                if s.is_empty() {
                    in_ranges = false;
                    continue;
                }

                if in_ranges {
                    // Parse a range a-b
                    if let Some((a_str, b_str)) = s.split_once('-') {
                        let a: u128 = a_str.trim().parse::<u128>().expect("invalid range start");
                        let b: u128 = b_str.trim().parse::<u128>().expect("invalid range end");
                        let lo: u128 = a.min(b);
                        let hi: u128 = a.max(b);
                        ranges.push((lo, hi));
                    }
                } else {
                    // Parse an ID line
                    let id: u128 = s.parse::<u128>().expect("invalid id");
                    ids.push(id);
                }
            }

            if ranges.is_empty() || ids.is_empty() {
                return 0u128;
            }

            // Merge overlapping ranges for efficient membership checks.
            ranges.sort_unstable_by(|&(a1, b1), &(a2, b2)| if a1 == a2 { b1.cmp(&b2) } else { a1.cmp(&a2) });
            let mut merged: Vec<(u128, u128)> = Vec::with_capacity(ranges.len());
            for (start, end) in ranges {
                if let Some((_m_start, m_end)) = merged.last_mut() {
                    if start <= *m_end + 1 { // touch or overlap
                        if end > *m_end { *m_end = end; }
                        continue;
                    }
                }
                merged.push((start, end));
            }

            // For each id, binary search merged intervals.
            let mut fresh_count: u128 = 0u128;
            for id in ids {
                let mut lo: usize = 0;
                let mut hi: usize = merged.len();
                let mut found: bool = false;
                while lo < hi {
                    let mid: usize = lo + (hi - lo) / 2;
                    let (s0, e0) = merged[mid];
                    if id < s0 {
                        hi = mid;
                    } else if id > e0 {
                        lo = mid + 1;
                    } else {
                        found = true; // s0 <= id <= e0
                        break;
                    }
                }
                if found {
                    fresh_count = fresh_count + 1u128;
                }
            }

            fresh_count
        }

        fn part2(input: &str) -> u128 {
            // Count how many ingredient IDs are considered fresh by the ranges alone.
            // Ignore the available IDs section. Sum the total size of the union of ranges.
            let mut ranges: Vec<(u128, u128)> = Vec::new();

            let mut in_ranges: bool = true;
            for line in input.lines() {
                let s: &str = line.trim();
                if s.is_empty() {
                    // Blank line separates sections; stop collecting ranges.
                    in_ranges = false;
                    continue;
                }
                if in_ranges {
                    if let Some((a_str, b_str)) = s.split_once('-') {
                        let a: u128 = a_str.trim().parse::<u128>().expect("invalid range start");
                        let b: u128 = b_str.trim().parse::<u128>().expect("invalid range end");
                        let lo: u128 = a.min(b);
                        let hi: u128 = a.max(b);
                        ranges.push((lo, hi));
                    }
                } else {
                    // Past the blank line; ignore available IDs for Part 2.
                    // We still iterate to the end to be robust to formatting.
                    continue;
                }
            }

            if ranges.is_empty() {
                return 0u128;
            }

            // Merge overlapping/touching intervals.
            ranges.sort_unstable_by(|&(a1, b1), &(a2, b2)| if a1 == a2 { b1.cmp(&b2) } else { a1.cmp(&a2) });
            let mut merged: Vec<(u128, u128)> = Vec::with_capacity(ranges.len());
            for (start, end) in ranges {
                if let Some((_m_start, m_end)) = merged.last_mut() {
                    if start <= *m_end + 1u128 {
                        if end > *m_end { *m_end = end; }
                        continue;
                    }
                }
                merged.push((start, end));
            }

            // Sum sizes (inclusive ranges)
            let mut total: u128 = 0u128;
            for (s0, e0) in merged {
                let len: u128 = (e0 - s0) + 1u128;
                total = total + len;
            }

            total
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
