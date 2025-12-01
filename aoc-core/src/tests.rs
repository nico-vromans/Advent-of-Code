#[cfg(test)]
mod tests {
    use crate::read_input;
    use std::env;
    use std::fs;
    use std::io::Error;
    use std::path::PathBuf;

    #[test]
    fn reads_existing_input_file() {
        // Create a temporary directory and populate the expected path
        let tmp_dir: PathBuf = env::temp_dir().join("aoc_core_read_input_test");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(tmp_dir.join("year2025/src/day01")).unwrap();
        let expected: &str = "L1\nR2\nL3\n";
        fs::write(tmp_dir.join("year2025/src/day01/input.txt"), expected).unwrap();

        // Save the current dir and switch to the temp dir
        let original_dir: PathBuf = env::current_dir().unwrap();
        env::set_current_dir(&tmp_dir).unwrap();

        let content: String = read_input(2025, 1).expect("should read existing input file");
        assert_eq!(content, expected);

        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn missing_input_file_returns_error() {
        let err: Error = read_input(2099, 99).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}
