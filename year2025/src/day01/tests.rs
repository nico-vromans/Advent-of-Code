use super::*;
use aoc_core::Solver;

#[test]
fn test_solve_example() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let solver = Day01;
    let result = solver.solve(input);

    // Part 1 example answer is 3
    assert_eq!(result[0], "3");
    // Part 2 example answer is 6
    assert_eq!(result[1], "6");
}

#[test]
fn test_empty_input() {
    let input: &str = "\n\n  \n"; // empty/whitespace-only lines
    let solver = Day01;
    let result: Vec<String> = solver.solve(input);

    assert_eq!(result[0], "0");
    assert_eq!(result[1], "0");
}

#[test]
fn test_lowercase_and_trim() {
    let input: &str = "  l1\n r2 \n l1  ";
    let solver = Day01;
    let result: Vec<String> = solver.solve(input);

    // Start at 50: L1 -> 49, R2 -> 51, L1 -> 50. Part1 counts zeros after each instruction: none.
    assert_eq!(result[0], "0");
    // Part2 steps: L1 hits no zero, R2 no zero, L1 no zero => 0
    assert_eq!(result[1], "0");
}

#[test]
fn test_wrapping_and_counts() {
    let solver = Day01;

    // Part 1: landing exactly on zero after a single large move should count once
    let result: Vec<String> = solver.solve("L50\n");
    assert_eq!(result[0], "1");
    assert_eq!(result[1], "1"); // stepping 50 times from 50 down to 0 hits zero once

    // Part 2: two full wraps should count hitting zero twice
    let result: Vec<String> = solver.solve("L200\n");
    assert_eq!(result[0], "0"); // the final position is 50, not zero
    assert_eq!(result[1], "2"); // crosses zero twice at steps 50 and 150
}

#[test]
#[should_panic]
fn test_invalid_direction_panics() {
    let solver = Day01;
    // 'U' should panic in the match arms
    let _ = solver.solve("U10\n");
}
