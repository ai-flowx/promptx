use super::constants::*;
use super::utils::*;
use log::debug;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use tempfile::TempDir;
use tokio;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_download_model_success() {
    let mock_server = MockServer::start().await;

    let temp_dir = tempdir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    let model_dir = temp_dir
        .path()
        .join(DirNames::PACKAGE_BASE_DIR)
        .join(DirNames::MODEL_DIR);
    fs::create_dir_all(&model_dir).unwrap();

    let test_file_content = b"mock model data";
    let test_filename = "test_model.bin";
    let test_url = format!("{}/{}", mock_server.uri(), test_filename);

    Mock::given(method("GET"))
        .and(path(format!("/{}", test_filename)))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(test_file_content))
        .mount(&mock_server)
        .await;

    let result = Download::download_model(&test_url).await;

    assert!(result.is_ok());
    let downloaded_path = result.unwrap();
    assert!(downloaded_path.exists());
    assert_eq!(
        fs::read(downloaded_path.clone()).unwrap(),
        test_file_content
    );

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

#[tokio::test]
async fn test_download_model_file_exists() {
    let temp_dir = tempdir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    let model_dir = temp_dir
        .path()
        .join(DirNames::PACKAGE_BASE_DIR)
        .join(DirNames::MODEL_DIR);
    fs::create_dir_all(&model_dir).unwrap();

    let test_filename = "existing_model.bin";
    let existing_content = b"existing model data";
    let existing_file_path = model_dir.join(test_filename);
    fs::write(&existing_file_path, existing_content).unwrap();

    let test_url = format!("http://example.com/{}", test_filename);

    let result = Download::download_model(&test_url).await;

    assert!(result.is_ok());
    let downloaded_path = result.unwrap();
    assert!(downloaded_path.exists());
    assert_eq!(fs::read(downloaded_path.clone()).unwrap(), existing_content);

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

#[tokio::test]
async fn test_download_model_failure() {
    let mock_server = MockServer::start().await;

    let temp_dir = tempdir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    let model_dir = temp_dir
        .path()
        .join(DirNames::PACKAGE_BASE_DIR)
        .join(DirNames::MODEL_DIR);
    fs::create_dir_all(&model_dir).unwrap();

    let test_filename = "failing_model.bin";
    let test_url = format!("{}/{}", mock_server.uri(), test_filename);

    Mock::given(method("GET"))
        .and(path(format!("/{}", test_filename)))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let result = Download::download_model(&test_url).await;

    assert!(result.is_err());

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

#[tokio::test]
async fn test_download_model_invalid_url() {
    let result = Download::download_model("invalid-url").await;
    assert!(result.is_err());
}

#[test]
fn test_yaml_to_dict() {
    let temp_dir = tempdir().unwrap();
    let yaml_path = temp_dir.path().join("test.yaml");

    fs::write(&yaml_path, "key: value\nlist:\n  - item1\n  - item2").unwrap();

    let result = FileUtils::yaml_to_dict(&yaml_path).unwrap();
    assert_eq!(result["key"], "value");
    assert_eq!(result["list"][0], "item1");
    assert_eq!(result["list"][1], "item2");

    temp_dir
        .close()
        .expect("failed to close temporary directory");
}

#[test]
fn test_yaml_to_class() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        value: i32,
    }

    let temp_dir = tempdir().unwrap();
    let custom_path = temp_dir.path().join("custom.yaml");
    let default_path = temp_dir.path().join("default.yaml");

    fs::write(&custom_path, "name: custom\nvalue: 42").unwrap();
    fs::write(&default_path, "name: default\nvalue: 0\nextra: field").unwrap();

    let result: TestConfig =
        FileUtils::yaml_to_class(Some(&custom_path), Some(&default_path)).unwrap();

    assert_eq!(
        result,
        TestConfig {
            name: "custom".to_string(),
            value: 42,
        }
    );

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

#[test]
fn test_read_jsonl() {
    let temp_dir = tempdir().unwrap();
    let jsonl_path = temp_dir.path().join("test.jsonl");

    let content = r#"{"id": 1, "name": "test1"}
{"id": 2, "name": "test2"}
"#;
    fs::write(&jsonl_path, content).unwrap();

    let result = FileUtils::read_jsonl(&jsonl_path).unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0]["id"], 1);
    assert_eq!(result[1]["name"], "test2");

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

#[test]
fn test_read_jsonl_row() {
    let temp_dir = tempdir().unwrap();
    let jsonl_path = temp_dir.path().join("test.jsonl");

    let content = r#"{"id": 1}
{"id": 2}
"#;
    fs::write(&jsonl_path, content).unwrap();

    match FileUtils::read_jsonl_row(&jsonl_path.clone()) {
        Ok(iterator) => {
            let mut iter = iterator.enumerate();
            while let Some((i, json_result)) = iter.next() {
                match i {
                    0 => assert_eq!(json_result.unwrap()["id"], 1),
                    1 => assert_eq!(json_result.unwrap()["id"], 2),
                    _ => assert!(false),
                }
            }
        }
        Err(_) => assert!(false),
    }

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

#[test]
fn test_append_as_jsonl() {
    let temp_dir = tempdir().unwrap();
    let jsonl_path = temp_dir.path().join("test.jsonl");

    let data = json!({"test": "value"});
    FileUtils::append_as_jsonl(&jsonl_path, &data).unwrap();

    let content = fs::read_to_string(&jsonl_path).unwrap();
    assert_eq!(content.trim(), r#"{"test":"value"}"#);

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

#[test]
fn test_save_jsonlist() {
    let temp_dir = tempdir().unwrap();
    let jsonl_path = temp_dir.path().join("test.jsonl");

    let data = vec![json!({"id": 1}), json!({"id": 2})];

    FileUtils::save_jsonlist(&jsonl_path, &data, false).unwrap();
    let content = fs::read_to_string(&jsonl_path).unwrap();
    assert_eq!(content.lines().count(), 2);

    FileUtils::save_jsonlist(&jsonl_path, &data, true).unwrap();
    let content = fs::read_to_string(&jsonl_path).unwrap();
    assert_eq!(content.lines().count(), 4);

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

#[test]
fn test_str_list_to_dir_path() {
    let str_list = vec!["path", "to", "dir"];
    let result = FileUtils::str_list_to_dir_path(&str_list);
    assert_eq!(result, PathBuf::from("path").join("to").join("dir"));
}

#[test]
fn test_logger() {
    let temp_dir = tempdir().unwrap();
    let log_dir = temp_dir.path().to_str().unwrap();

    let mut logger = Logger {};
    let result = logger.set_logger("test_module", log_dir);
    assert!(result.is_ok());

    assert!(Path::new(log_dir).exists());

    let log_file = Path::new(log_dir).join(FileConstants::LOGFILE_NAME);
    assert!(log_file.exists());

    debug!("This is a test debug message.");

    let log_contents = fs::read_to_string(log_file).unwrap();
    assert!(log_contents.contains("This is a test debug message."));

    temp_dir
        .close()
        .expect("failed to delete temporary directory");
}

fn setup_cargo_toml(content: &str) -> (TempDir, PathBuf) {
    let dir = tempdir().unwrap();
    let cargo_path = dir.path().join("Cargo.toml");

    fs::write(&cargo_path, content).unwrap();
    (dir, cargo_path)
}

#[test]
fn test_install_lib_existing_package() {
    let content = r#"
[package]
name = "test_package"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#;
    let (_dir, _cargo_path) = setup_cargo_toml(content);

    let result = RuntimeTasks::install_lib_if_missing("serde", None);
    assert!(result.is_ok());
    assert!(result.unwrap());

    _dir.close().expect("failed to close cargo dir");
}

#[test]
fn test_install_lib_with_version() {
    let content = r#"
[package]
name = "test_package"
version = "0.1.0"

[dependencies]
"#;
    let (_dir, _cargo_path) = setup_cargo_toml(content);

    let result = RuntimeTasks::install_lib_if_missing("tokio==1.0", None);
    assert!(result.is_ok());
    assert!(!result.unwrap());

    _dir.close().expect("failed to close cargo dir");
}

#[test]
fn test_install_lib_with_registry() {
    let content = r#"
[package]
name = "test_package"
version = "0.1.0"

[dependencies]
"#;
    let (_dir, _cargo_path) = setup_cargo_toml(content);

    let result =
        RuntimeTasks::install_lib_if_missing("custom_package", Some("https://custom.registry.com"));
    assert!(result.is_ok());
    assert!(!result.unwrap());

    _dir.close().expect("failed to close cargo dir");
}

#[test]
fn test_str_to_class_import_path() {
    let lib_path = "dummy_lib";
    let result = RuntimeTasks::str_to_class(Some(lib_path), None);
    assert!(result.is_err());
    matches!(result.unwrap_err(), RuntimeError::ModuleLoadError(_));
}

#[test]
fn test_str_to_class_file_path() {
    let path = Path::new("dummy_lib");
    let result = RuntimeTasks::str_to_class(None, Some(path));
    assert!(result.is_err());
    matches!(result.unwrap_err(), RuntimeError::ModuleLoadError(_));
}

#[test]
fn test_str_to_class_no_path() {
    let result = RuntimeTasks::str_to_class(None, None);
    assert!(result.is_err());
    matches!(result.unwrap_err(), RuntimeError::ModuleLoadError(_));
}

#[test]
fn test_runtime_error_display() {
    let errors = vec![
        RuntimeError::PackageNotFound("test".to_string()),
        RuntimeError::InstallationError("test".to_string()),
        RuntimeError::ValidationError("test".to_string()),
        RuntimeError::ModuleLoadError("test".to_string()),
    ];

    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}
