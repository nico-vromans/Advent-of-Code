use aoc_core::Solver;
use std::env;
use std::io::Error;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: runner <year-day>");
        eprintln!("Example: runner 2025-01");
        process::exit(1);
    }

    let target: &String = &args[1];
    let parts: Vec<&str> = target.split('-').collect();

    if parts.len() != 2 {
        eprintln!("Invalid format. Use <year-day>, e.g., 2025-01");
        process::exit(1);
    }

    let year: u16 = parts[0].parse().expect("Invalid year");
    let day: u8 = parts[1].parse().expect("Invalid day");

    let solver: Option<Box<dyn Solver>> = match year {
        2025 => year2025::get_solver(day),
        _ => {
            eprintln!("Year {} not implemented", year);
            process::exit(1);
        }
    };

    if let Some(solver) = solver {
        // In a real scenario, we would read the input file here.
        // For now, we'll pass a dummy string or try to read it if it exists.
        // The input file path convention could be `year{}/src/day{:02}/input.txt` but that's inside src.
        // Better: `year{}/input/day{:02}.txt` or similar.
        // For this basic setup, let's assume the input is passed or hardcoded for now,
        // or we try to read from `year2025/src/day01/input.txt` relative to CWD.

        // Use the generic input reader
        let input: String = aoc_core::read_input(year, day).unwrap_or_else(|e: Error| {
            eprintln!("Error reading input: {}", e);
            String::new()
        });

        println!("--- Year {} Day {} ---", year, day);

        let results: Vec<String> = solver.solve(&input);

        if results.is_empty() {
            println!("No parts implemented.");
        } else {
            for (i, result) in results.iter().enumerate() {
                println!("Part {}: {}", i + 1, result);
            }
        }
    } else {
        eprintln!("Day {} not implemented for year {}", day, year);
        process::exit(1);
    }
}
