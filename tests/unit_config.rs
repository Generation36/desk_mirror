#[cfg(test)]
mod tests {
    use desk_mirror::config::{ConfigError, read_config};
    use std::{io::Write, path::PathBuf};
    use tempfile::NamedTempFile;

    fn write_temp_config(content: &str) -> (NamedTempFile, PathBuf) {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        write!(file, "{}", content).expect("Failed to write to temp file");
        let path = file.path().to_path_buf();
        (file, path)
    }

    #[test]
    fn test_valid_config() {
        let (_file, path) = write_temp_config(
            r#"
        api_key = "12345"
        location = "London"
    "#,
        );

        let config = read_config(&path).expect("Expected config to load");
        assert_eq!(
            config.get("api_key").unwrap().as_str(),
            Some("12345"),
            "Expected api_key to be '12345'"
        );
        assert_eq!(
            config.get("location").unwrap().as_str(),
            Some("London"),
            "Expected location to be 'London'"
        );
    }

    #[test]
    fn test_file_not_found() {
        let path = PathBuf::from("nonexistent.toml");
        match read_config(path) {
            Err(ConfigError::ReadError { .. }) => {} // Expected
            other => panic!("Unexpected result: {:?}", other),
        }
    }

    #[test]
    fn test_parse_error() {
        let (_file, path) = write_temp_config("api_key = "); // Invalid TOML

        match read_config(&path) {
            Err(ConfigError::ParseError { .. }) => {} // Expected
            other => panic!("{}", format!("Unexpected result: {:?}", other)),
        }
    }

    // #[test]
    // fn test_missing_value_error_empty_string() {
    //     let path = write_temp_config(
    //         r#"
    //         api_key = ""
    //         "#,
    //     );

    //     match read_config(&path) {
    //         Err(ConfigError::MissingValueError { key }) => {
    //             assert_eq!(key, "api_key");
    //         }
    //         other => panic!("Unexpected result: {:?}", other),
    //     }
    // }
}
