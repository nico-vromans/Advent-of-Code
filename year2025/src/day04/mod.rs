#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day04;

impl Solver for Day04 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            // Count how many rolls ('@') are accessible: a roll is accessible
            // if it has fewer than four adjacent rolls among the 8 neighbors.
            let mut grid: Vec<Vec<u8>> = Vec::new();

            for line in input.lines() {
                let s: &str = line.trim();
                if s.is_empty() {
                    continue;
                }

                let row: Vec<u8> = s
                    .chars()
                    .map(|c: char| if c == '@' { 1u8 } else { 0u8 })
                    .collect();
                grid.push(row);
            }

            let h: usize = grid.len();
            if h == 0 {
                return 0u128;
            }
            let w: usize = grid[0].len();

            let dirs: [(isize, isize); 8] = [
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1),
            ];

            let mut total: u128 = 0u128;

            for y in 0..h {
                for x in 0..w {
                    if grid[y][x] == 0u8 {
                        continue;
                    }

                    let mut adj: u8 = 0u8;
                    for &(dx, dy) in &dirs {
                        let nx: isize = x as isize + dx;
                        let ny: isize = y as isize + dy;
                        if nx >= 0 && ny >= 0 {
                            let ux: usize = nx as usize;
                            let uy: usize = ny as usize;
                            if uy < h && ux < w {
                                adj = adj.saturating_add(grid[uy][ux]);
                            }
                        }
                    }

                    if adj < 4u8 {
                        total = total + 1u128;
                    }
                }
            }

            total
        }

        fn part2(input: &str) -> u128 {
            // Not yet specified for this day.
            0u128
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
