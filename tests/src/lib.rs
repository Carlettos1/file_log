#[allow(unused_imports)]
use std::fs;

#[test]
fn test_log_macro() {
    file_log::log!("test_log", "Test log message");

    // Verify that the log file was created
    let file_path = format!("test_log_{}.log", file_log::index());
    assert!(fs::metadata(&file_path).is_ok());

    // Verify that the log file contains the expected data
    let data = fs::read_to_string(file_path).unwrap();
    assert_eq!(data, "Test log message\n");
}

// test the log macro with custom file extension
#[test]
fn test_log_macro_with_custom_extension() {
    file_log::log!("test_log" "log2", "Test log message");

    // Verify that the log file was created
    let file_path = format!("test_log_{}.log2", file_log::index());
    assert!(fs::metadata(&file_path).is_ok());

    // Verify that the log file contains the expected data
    let data = fs::read_to_string(file_path).unwrap();
    assert_eq!(data, "Test log message\n");
}
