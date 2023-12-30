/// Return timestamp based on current date, time and zone,
/// for use in logs and filenames.
pub fn timestamp() -> String {
    let dt = chrono::Utc::now();
    let file_name = format!("{}", dt.format("%Y%m%d_%H%M%S_%f_%Z"));
    file_name
}

/// Write given data to file under given dir, and make sure the dir exists.
pub fn write_file(dir: &String, file_name: &String, data: &Vec<u8>) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let path = std::path::Path::new(&dir).join(file_name);
    std::fs::write(path, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn timestamp_format() {
        // Check if the timestamp format is as expected
        let timestamp = timestamp();
        assert_eq!(timestamp.len(), 29);
    }

    #[test]
    fn write_file_successful() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

        let dir_path = temp_dir.path().to_str().unwrap().to_string();
        let file_name = "test_file.txt";
        let data = b"Test data";

        // Perform the file write operation
        let result = write_file(&dir_path, &file_name.to_string(), &data.to_vec());

        // Check if the file was created successfully
        assert!(result.is_ok());

        // Check if the file content is as expected
        let file_path = std::path::Path::new(&dir_path).join(&file_name);
        let file_content = fs::read(&file_path).expect("Failed to read file");
        assert_eq!(file_content, data);
    }

    #[test]
    fn write_file_with_invalid_directory() {
        // Attempt to write to an invalid directory
        let invalid_dir = "/nonexistent/directory";
        let file_name = "test_file.txt";
        let data = b"Test data";

        // Perform the file write operation
        let result = write_file(
            &invalid_dir.to_string(),
            &file_name.to_string(),
            &data.to_vec(),
        );

        // Check if the write operation failed as expected
        assert!(result.is_err());
    }
}
