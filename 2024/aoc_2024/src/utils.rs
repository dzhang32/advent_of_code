use std::env;
use std::path::PathBuf;

pub fn data_path(day: i32, part: i32) -> PathBuf {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    dir.push("data");
    dir.push(format!("day_{}_part_{}.txt", day, part));

    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_dir() {
        let input_file_path = data_path(1, 1);
        assert!(input_file_path.is_file())
    }
}
