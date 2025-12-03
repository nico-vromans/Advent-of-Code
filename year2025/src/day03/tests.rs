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
