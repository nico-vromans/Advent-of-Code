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
            // Repeatedly remove accessible rolls ('@'): a roll is accessible
            // if it has fewer than four adjacent rolls among the 8 neighbors.
            // After removing a roll, neighbors' adjacency counts decrease,
            // potentially making more rolls accessible. Return total removed.
            use std::collections::VecDeque;

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

            // Precompute adjacency counts for all cells.
            let mut adj: Vec<Vec<u8>> = vec![vec![0u8; w]; h];
            for y in 0..h {
                for x in 0..w {
                    if grid[y][x] == 0u8 {
                        continue;
                    }
                    let mut cnt: u8 = 0u8;
                    for &(dx, dy) in &dirs {
                        let nx: isize = x as isize + dx;
                        let ny: isize = y as isize + dy;
                        if nx >= 0 && ny >= 0 {
                            let ux: usize = nx as usize;
                            let uy: usize = ny as usize;
                            if uy < h && ux < w {
                                cnt = cnt.saturating_add(grid[uy][ux]);
                            }
                        }
                    }
                    adj[y][x] = cnt;
                }
            }

            // Initialize queue with currently-accessible rolls (adj < 4).
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            for y in 0..h {
                for x in 0..w {
                    if grid[y][x] == 1u8 && adj[y][x] < 4u8 {
                        queue.push_back((x, y));
                    }
                }
            }

            let mut removed_total: u128 = 0u128;

            while let Some((x, y)) = queue.pop_front() {
                // Skip if already removed or no longer accessible.
                if grid[y][x] == 0u8 {
                    continue;
                }
                if adj[y][x] >= 4u8 {
                    continue;
                }

                // Remove this roll.
                grid[y][x] = 0u8;
                removed_total = removed_total + 1u128;

                // Update neighbors' adjacency counts and enqueue if they become accessible.
                for &(dx, dy) in &dirs {
                    let nx: isize = x as isize + dx;
                    let ny: isize = y as isize + dy;
                    if nx >= 0 && ny >= 0 {
                        let ux: usize = nx as usize;
                        let uy: usize = ny as usize;
                        if uy < h && ux < w {
                            if grid[uy][ux] == 1u8 {
                                // This neighbor loses one adjacent roll.
                                if adj[uy][ux] > 0u8 {
                                    adj[uy][ux] -= 1u8;
                                }
                                if adj[uy][ux] < 4u8 {
                                    queue.push_back((ux, uy));
                                }
                            }
                        }
                    }
                }
            }

            removed_total
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
