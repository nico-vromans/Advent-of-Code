use super::Day06;
use aoc_core::Solver;

#[test]
fn test_part1_simple() {
    let input = "1 2\n3 4\n+ *";
    let solver = Day06;
    let result = solver.solve(input);
    assert_eq!(result[0], "12");
}

#[test]
fn test_part1_readme_example_adapted() {
    let input = "123 328 51 64\n45 64 387 23\n6 98 215 314\n* + * +";
    let solver = Day06;
    let result = solver.solve(input);
    assert_eq!(result[0], "4277556");
}

#[test]
fn test_part2_readme_example() {
    // Constructed to match the alignment requirements derived:
    // P1: Right aligned (123, 45, 6)
    // P2: Left aligned (328, 64, 98)
    // P3: Right aligned (51, 387, 215)
    // P4: Left aligned (64, 23, 314)
    // Separated by at least one empty column.
    // Operators placed under the blocks.

    let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
 *   +    *   + ";

    // P1: 356 * 24 * 1 = 8544
    // P2: 8 + 248 + 369 = 625
    // P3: 175 * 581 * 32 = 3253600
    // P4: 4 + 431 + 623 = 1058
    // Total: 8544 + 625 + 3253600 + 1058 = 3263827

    let solver = Day06;
    let result = solver.solve(input);
    assert_eq!(result[1], "3263827");
}
