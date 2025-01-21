use super::file_utils::*;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn setup_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

#[test]
fn test_read_jsonl() {
    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.path().join("test.jsonl");

    let test_data = vec![
        json!({"name": "Alice", "age": 30}),
        json!({"name": "Bob", "age": 25}),
    ];

    FileUtils::save_jsonlist(&file_path, &test_data, "w").unwrap();

    let result = FileUtils::read_jsonl(&file_path).unwrap();
    assert_eq!(result, test_data);
}

#[test]
fn test_read_jsonl_row() {
    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.path().join("test.jsonl");

    let test_data = vec![
        json!({"name": "Alice", "age": 30}),
        json!({"name": "Bob", "age": 25}),
    ];

    FileUtils::save_jsonlist(&file_path, &test_data, "w").unwrap();

    let result: Vec<Value> = FileUtils::read_jsonl_row(&file_path)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(result, test_data);
}

#[test]
fn test_append_as_jsonl() {
    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.path().join("test.jsonl");

    let initial_data = json!({"name": "Alice", "age": 30});
    FileUtils::append_as_jsonl(&file_path, &initial_data).unwrap();

    let append_data = json!({"name": "Bob", "age": 25});
    FileUtils::append_as_jsonl(&file_path, &append_data).unwrap();

    let result = FileUtils::read_jsonl(&file_path).unwrap();
    assert_eq!(result, vec![initial_data, append_data]);
}

#[test]
fn test_save_jsonlist() {
    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.path().join("test.jsonl");

    let initial_data = vec![
        json!({"name": "Alice", "age": 30}),
        json!({"name": "Bob", "age": 25}),
    ];
    FileUtils::save_jsonlist(&file_path, &initial_data, "w").unwrap();

    let append_data = vec![json!({"name": "Charlie", "age": 35})];
    FileUtils::save_jsonlist(&file_path, &append_data, "a").unwrap();

    let mut expected = initial_data.clone();
    expected.extend(append_data);
    let result = FileUtils::read_jsonl(&file_path).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_str_list_to_dir_path() {
    let input = vec!["path", "to", "directory"];
    let result = FileUtils::str_list_to_dir_path(&input);

    let expected = PathBuf::from("path").join("to").join("directory");
    assert_eq!(result, expected);
}

#[test]
fn test_error_handling() {
    let result = FileUtils::read_jsonl("nonexistent.jsonl");
    assert!(result.is_err());

    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.path().join("invalid.jsonl");
    fs::write(&file_path, "invalid json\n").unwrap();

    let result = FileUtils::read_jsonl(&file_path);
    assert!(result.is_err());

    let result: Result<Vec<_>, _> = FileUtils::read_jsonl_row(&file_path).unwrap().collect();
    assert!(result.is_err());
}
