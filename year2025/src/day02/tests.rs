use super::Day02;
use aoc_core::{Solver, read_input};

// Tests for Day 2 — based on the puzzle README and the provided input file.

#[test]
fn test_day02_readme_example_single_both_parts() {
    // The README example (wrapped there for legibility) as a single line of ranges.
    // It should produce a sum of 33 for Part 1.
    let input: &str = "11-22";

    let solver = Day02;
    let result: Vec<String> = solver.solve(input);

    // Part 1 example answer from README
    assert_eq!(result[0], "33");

    // Part 2 answer derived from logic or manual calculation if small enough.
    // The README says for the FULL example it is 4174379265.
    // For the single example "11-22":
    // 11-22. Invalid: 11, 22. Sum = 33.
    // Part 2 rule: "at least twice".
    // 11 is 1 repeated 2 times. Valid.
    // 22 is 2 repeated 2 times. Valid.
    // So sum is 33.
    assert_eq!(result[1], "33");
}

#[test]
fn test_day02_readme_example_full_both_parts() {
    // The README example (wrapped there for legibility) as a single line of ranges.
    // It should produce a sum of 1227775554 for Part 1.
    let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

    let solver = Day02;
    let result: Vec<String> = solver.solve(input);

    // Part 1 example answer from README
    assert_eq!(result[0], "1227775554");

    // Part 2 example answer from README
    assert_eq!(result[1], "4174379265");
}

#[test]
fn test_day02_real_input_known_answers() {
    // Smoke test for the real input file to quickly verify integration using
    // the known correct answers for both parts.
    let input: String = read_input(2025, 2).expect("input file should exist for day 02");
    let solver = Day02;
    let result: Vec<String> = solver.solve(&input);

    // Ensure we always return two parts
    assert_eq!(result.len(), 2, "solver should return two answers");

    assert_eq!(result[0], "53420042388");
    assert_eq!(result[1], "69553832684");
}

// --- Additional Day 2 tests -------------------------------------------------
// These complement the README example with extra passing cases and sanity checks
// for both parts, plus a simple combined scenario.

#[test]
fn test_day02_part1_additional_passing() {
    let solver = Day02;
    // From README: 95-115 has one invalid ID 99 for Part 1 => sum 99
    let result: Vec<String> = solver.solve("95-115");
    assert_eq!(result[0], "99");
}

#[test]
fn test_day02_part1_additional_checks() {
    let solver = Day02;
    // Sanity check: 95-115 has a single invalid ID (99) for Part 1
    let result: Vec<String> = solver.solve("95-115");
    assert_eq!(result[0], "99");
}

#[test]
fn test_day02_part2_additional_passing() {
    let solver = Day02;
    // From README Part 2: 95-115 has invalid IDs 99 and 111 => 99 + 111 = 210
    let result: Vec<String> = solver.solve("95-115");
    assert_eq!(result[1], "210");
}

#[test]
fn test_day02_part2_additional_checks() {
    let solver = Day02;
    // Sanity check: 95-115 has invalid IDs 99 and 111 for Part 2 => 210
    let result: Vec<String> = solver.solve("95-115");
    assert_eq!(result[1], "210");
}

#[test]
fn test_day02_combined_passing() {
    let solver = Day02;
    let result: Vec<String> = solver.solve("95-115");
    assert_eq!(result[0], "99");
    assert_eq!(result[1], "210");
}

#[test]
fn test_day02_combined_additional_checks() {
    let solver = Day02;
    let result: Vec<String> = solver.solve("95-115");
    // Additional combined check: verify both parts match expected values
    assert_eq!(result[0], "99");
    assert_eq!(result[1], "210");
}
