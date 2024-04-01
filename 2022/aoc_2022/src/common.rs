#[cfg(test)]
pub mod tests {
    use std::env;
    use std::path::PathBuf;

    pub fn data_path(file: &str) -> PathBuf {
        let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        dir.push("data");
        dir.push(file);

        dir
    }

    #[test]
    fn test_data_dir() {
        let input_file_path = data_path("day_01_part_1.txt");
        assert!(input_file_path.is_file())
    }
}
