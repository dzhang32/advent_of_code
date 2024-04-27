#[cfg(test)]
pub mod tests {
    use std::env;
    use std::path::PathBuf;

    // Places under tests module as fn is only used in tests.
    pub fn data_path(file: &str) -> PathBuf {
        // CARGO_MANIFEST_DIR points to root of the project,
        // where Cargo.toml lives.
        let mut data_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        data_path.push("data");
        data_path.push(file);

        data_path
    }

    #[test]
    fn test_data_dir() {
        let input_file_path = data_path("day_01_part_1.txt");
        assert!(input_file_path.is_file())
    }
}
