#[cfg(test)]
mod my_tests;

use aoc_core::Solver;

pub struct Day06;

impl Solver for Day06 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            let lines: Vec<&str> = input
                .lines()
                .filter(|l| !l.trim().is_empty())
                .collect();

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
                         panic!("Row has fewer columns than operators line at index {}", col_idx);
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

        fn part2(_input: &str) -> u128 {
            0
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
