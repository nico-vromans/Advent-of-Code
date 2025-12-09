#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day09;

impl Solver for Day09 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            // Parse coordinates and find the largest axis-aligned rectangle area
            // using any two red tiles as opposite corners. Area is inclusive:
            // (|x1 - x2| + 1) * (|y1 - y2| + 1).
            let mut pts: Vec<(i128, i128)> = Vec::new();

            for line in input.lines() {
                let s: &str = line.trim();
                if s.is_empty() { continue; }
                if let Some((xs, ys)) = s.split_once(',') {
                    let x: i128 = xs.trim().parse::<i128>().expect("invalid x");
                    let y: i128 = ys.trim().parse::<i128>().expect("invalid y");
                    pts.push((x, y));
                }
            }

            let n: usize = pts.len();
            if n < 2usize { return 0u128; }

            let mut best: u128 = 0u128;
            for i in 0usize..n {
                let (x1, y1) = pts[i];
                for j in (i + 1usize)..n {
                    let (x2, y2) = pts[j];
                    let dx: i128 = if x1 >= x2 { x1 - x2 } else { x2 - x1 };
                    let dy: i128 = if y1 >= y2 { y1 - y2 } else { y2 - y1 };
                    let width: u128 = (dx as u128) + 1u128;
                    let height: u128 = (dy as u128) + 1u128;
                    let area: u128 = width * height;
                    if area > best { best = area; }
                }
            }

            best
        }

        fn part2(_input: &str) -> u128 {
            123
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
