#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day06;

impl Solver for Day06 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

            if lines.is_empty() {
                return 0;
            }

            // The last line contains the operators
            let op_line = lines.last().unwrap();
            let operators: Vec<&str> = op_line.split_whitespace().collect();

            // All previous lines contain numbers
            let num_lines = &lines[..lines.len() - 1];

            // Parse numbers into a grid (Vec of rows)
            let mut parsed_rows: Vec<Vec<u128>> = Vec::new();
            for line in num_lines {
                let nums: Vec<u128> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u128>().expect("invalid number"))
                    .collect();
                parsed_rows.push(nums);
            }

            let mut total: u128 = 0;

            // Iterate over each column (defined by the operator line)
            for (col_idx, op) in operators.iter().enumerate() {
                let mut column_nums: Vec<u128> = Vec::new();

                // Collect numbers for this column from all rows
                for row in &parsed_rows {
                    if col_idx < row.len() {
                        column_nums.push(row[col_idx]);
                    } else {
                        // If a row is shorter than the operator line, something is wrong with assumption
                        // or the input format is irregular.
                        panic!(
                            "Row has fewer columns than operators line at index {}",
                            col_idx
                        );
                    }
                }

                // Apply the operator
                let col_res: u128 = match *op {
                    "+" => column_nums.iter().sum(),
                    "*" => column_nums.iter().product(),
                    _ => panic!("Unknown operator: {}", op),
                };

                total += col_res;
            }

            total
        }

        fn part2(input: &str) -> u128 {
            let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

            if lines.is_empty() {
                return 0;
            }

            let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

            // Pad lines to form a grid
            let grid: Vec<Vec<char>> = lines
                .iter()
                .map(|l| {
                    let mut chars: Vec<char> = l.chars().collect();
                    if chars.len() < max_len {
                        chars.resize(max_len, ' ');
                    }
                    chars
                })
                .collect();

            let height = grid.len();
            let width = max_len;

            // Find separator columns (all spaces)
            let mut separators = vec![];
            for c in 0..width {
                let mut is_sep = true;
                for r in 0..height {
                    if grid[r][c] != ' ' {
                        is_sep = false;
                        break;
                    }
                }
                if is_sep {
                    separators.push(c);
                }
            }

            // Identify blocks
            let mut blocks = vec![];
            let mut start = 0;
            let mut all_seps = separators.clone();
            all_seps.push(width);

            for sep in all_seps {
                if sep > start {
                    blocks.push(start..sep);
                }
                start = sep + 1;
            }

            let mut total: u128 = 0;

            for block_cols in blocks {
                // Extract operator from the last row
                let last_row = height - 1;
                let mut op_char = None;
                for c in block_cols.clone() {
                    let char_at = grid[last_row][c];
                    if char_at != ' ' {
                        op_char = Some(char_at);
                        break;
                    }
                }

                if op_char.is_none() {
                    continue;
                }

                let op = op_char.unwrap();
                let mut numbers: Vec<u128> = Vec::new();

                // Process columns to form numbers
                for c in block_cols {
                    let mut s = String::new();
                    // Read column top-to-bottom, excluding the operator row
                    for r in 0..height - 1 {
                        if grid[r][c] != ' ' {
                            s.push(grid[r][c]);
                        }
                    }
                    if !s.is_empty() {
                        if let Ok(n) = s.parse::<u128>() {
                            numbers.push(n);
                        }
                    }
                }

                if numbers.is_empty() {
                    continue;
                }

                let val: u128 = match op {
                    '+' => numbers.iter().sum(),
                    '*' => numbers.iter().product(),
                    _ => panic!("Unknown operator: {}", op),
                };

                total += val;
            }

            total
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
