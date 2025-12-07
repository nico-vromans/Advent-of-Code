#[cfg(test)]
mod tests;

use std::collections::HashSet;
use aoc_core::Solver;

pub struct Day07;

impl Solver for Day07 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
            if lines.is_empty() {
                return 0;
            }

            let height = lines.len();
            let width = lines[0].len();
            let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

            // Find start
            let mut start_pos = None;
            for r in 0..height {
                for c in 0..width {
                    if grid[r][c] == 'S' {
                        start_pos = Some((r, c));
                        break;
                    }
                }
                if start_pos.is_some() {
                    break;
                }
            }

            let (start_r, start_c) = match start_pos {
                Some(pos) => pos,
                None => return 0,
            };

            let mut active_cols: HashSet<isize> = HashSet::new();
            active_cols.insert(start_c as isize);
            
            let mut splits: u128 = 0;

            for r in (start_r + 1)..height {
                let mut next_cols: HashSet<isize> = HashSet::new();
                
                for &c in &active_cols {
                    // Check bounds
                    if c < 0 || c >= width as isize {
                        continue;
                    }
                    
                    let char_at = grid[r][c as usize];
                    if char_at == '^' {
                        splits += 1;
                        next_cols.insert(c - 1);
                        next_cols.insert(c + 1);
                    } else {
                        next_cols.insert(c);
                    }
                }
                
                active_cols = next_cols;
                
                if active_cols.is_empty() {
                    break;
                }
            }

            splits
        }

        fn part2(_input: &str) -> u128 {
            0
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
