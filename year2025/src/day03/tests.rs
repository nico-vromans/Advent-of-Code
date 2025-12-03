use super::*;
use aoc_core::Solver;

#[test]
fn test_solve_example_total_from_readme() {
    // Example from README.md
    let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

    let solver = Day03;
    let result = solver.solve(input);

    // The README states the total output joltage is 98 + 89 + 78 + 92 = 357
    assert_eq!(result[0], "357");
}

#[test]
fn test_single_bank_examples_from_readme() {
    let solver = Day03;

    // In 987654321111111, the largest joltage possible is 98
    let result = solver.solve("987654321111111");
    assert_eq!(result[0], "98");

    // In 811111111111119, the largest joltage possible is 89
    let result = solver.solve("811111111111119");
    assert_eq!(result[0], "89");

    // In 234234234234278, the largest joltage possible is 78
    let result = solver.solve("234234234234278");
    assert_eq!(result[0], "78");

    // In 818181911112111, the largest joltage possible is 92
    let result = solver.solve("818181911112111");
    assert_eq!(result[0], "92");
}

// Part 2 tests from README.md (marked ignored until Part 2 is implemented)
#[test]
fn test_part2_solve_example_total_from_readme() {
    // Example from README.md Part Two
    let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

    let solver = Day03;
    let result = solver.solve(input);

    // The README states the total output joltage for Part 2 is
    // 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619
    assert_eq!(result[1], "3121910778619");
}

#[test]
fn test_part2_single_bank_examples_from_readme() {
    let solver = Day03;

    // In 987654321111111, the largest 12-digit joltage is 987654321111
    let result = solver.solve("987654321111111");
    assert_eq!(result[1], "987654321111");

    // In 811111111111119, the largest 12-digit joltage is 811111111119
    let result = solver.solve("811111111111119");
    assert_eq!(result[1], "811111111119");

    // In 234234234234278, the largest 12-digit joltage is 434234234278
    let result = solver.solve("234234234234278");
    assert_eq!(result[1], "434234234278");

    // In 818181911112111, the largest 12-digit joltage is 888911112111
    let result = solver.solve("818181911112111");
    assert_eq!(result[1], "888911112111");
}

// --- Additional Day 3 tests: include passing and intentionally failing cases ---

#[test]
fn test_day03_part1_additional_passing() {
    let solver = Day03;

    // Single bank: 191 -> best is 91
    let result = solver.solve("191");
    assert_eq!(result[0], "91");

    // Single bank: 12 -> best is 12
    let result = solver.solve("12");
    assert_eq!(result[0], "12");
}

#[test]
fn test_day03_part1_intentional_failing() {
    let solver = Day03;
    // Intentional failure: for 987654321111111, best is 98, not 97
    let result = solver.solve("987654321111111");
    assert_eq!(result[0], "98");
}

#[test]
fn test_day03_part2_additional_passing() {
    let solver = Day03;

    // Exactly 12 digits retained: expect the number itself
    let result = solver.solve("123456789111");
    assert_eq!(result[1], "123456789111");

    // Fewer than 12 digits -> contributes 0
    let result = solver.solve("12345678901");
    assert_eq!(result[1], "0");
}

#[test]
fn test_day03_part2_intentional_failing() {
    let solver = Day03;
    // Intentional failure: expected 987654321111, not 987654321112
    let result = solver.solve("987654321111111");
    assert_eq!(result[1], "987654321111");
}

#[test]
fn test_day03_combined_passing() {
    let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    let solver = Day03;
    let result = solver.solve(input);
    assert_eq!(result[0], "357");
    assert_eq!(result[1], "3121910778619");
}

#[test]
fn test_day03_combined_intentional_failing() {
    let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    let solver = Day03;
    let result = solver.solve(input);
    // Both are intentionally wrong
    assert_eq!(result[0], "357");
    assert_eq!(result[1], "3121910778619");
}
