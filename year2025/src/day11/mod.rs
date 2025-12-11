#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day11;

impl Solver for Day11 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            123
        }

        fn part2(input: &str) -> u128 {
            123
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
