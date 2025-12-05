#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day04;

impl Solver for Day04 {
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
