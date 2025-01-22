use super::constants::*;
use super::file_utils::*;
use super::logger::*;
use serde_json::json;
use std::fs;
use std::io;
use tempfile::tempdir;
use uuid::Uuid;

fn setup() -> (Logger, tempfile::TempDir) {
    let temp_dir = tempdir().unwrap();
    let logger = Logger::new(temp_dir.path()).unwrap();
    (logger, temp_dir)
}

#[test]
fn test_new_logger() {
    let (logger, _temp_dir) = setup();
    assert!(logger.sample_unq_id.is_none());
    assert!(logger.chained_log.is_empty());
    assert!(logger.del_self_arg);
}

#[test]
fn test_reset_eval_glue() {
    let (mut logger, temp_dir) = setup();
    logger.sample_unq_id = Some(Uuid::new_v4());
    logger.chained_log.push(json!({"test": "data"}));

    let new_path = temp_dir.path().join("new_dir");
    logger.reset_eval_glue(&new_path).unwrap();

    assert!(logger.sample_unq_id.is_none());
    assert!(logger.chained_log.is_empty());
    assert!(new_path.exists());
}

#[test]
fn test_clear_chained_log() {
    let (mut logger, _temp_dir) = setup();
    logger.chained_log.push(json!({"test": "data"}));
    assert!(!logger.chained_log.is_empty());

    logger.clear_chained_log();
    assert!(logger.chained_log.is_empty());
}

#[test]
fn test_dump_chained_log_to_file() -> io::Result<()> {
    let (mut logger, temp_dir) = setup();
    let test_data = json!({"test": "data"});
    logger.chained_log.push(test_data.clone());

    let file_name = "test_dump";
    logger.dump_chained_log_to_file(file_name)?;

    let file_path = temp_dir.path().join(format!("{}.jsonl", file_name));
    assert!(file_path.exists());

    let content = fs::read_to_string(file_path)?;
    assert!(content.contains("test"));
    assert!(content.contains("data"));
    assert!(logger.chained_log.is_empty());

    Ok(())
}

#[test]
fn test_append_dict_to_chained_logs() {
    let (mut logger, _temp_dir) = setup();
    let test_data = json!({"test": "data"});
    logger.append_dict_to_chained_logs(test_data.clone());

    assert_eq!(logger.chained_log.len(), 1);
    assert_eq!(logger.chained_log[0], test_data);
}

#[test]
fn test_append_to_chained_log() {
    let (mut logger, _temp_dir) = setup();
    let result = logger.append_to_chained_log("test_method", || 42);

    assert_eq!(result, 42);
    assert_eq!(logger.chained_log.len(), 1);

    let log_entry = &logger.chained_log[0];
    assert_eq!(log_entry[Constants::OUTPUTS], json!(42));
    assert_eq!(
        log_entry[Constants::META][Constants::METHOD_NAME],
        "test_method"
    );
    assert!(log_entry[Constants::META][Constants::EXEC_SEC].is_number());
    assert!(log_entry[Constants::META][Constants::TIMESTAMP].is_string());
}

#[test]
fn test_log_io_params() -> io::Result<()> {
    let (mut logger, temp_dir) = setup();
    let result = logger.log_io_params("test_method", || "test_result", "test_file")?;

    assert_eq!(result, "test_result");
    let file_path = temp_dir.path().join("test_file.jsonl");
    assert!(file_path.exists());

    let content = fs::read_to_string(file_path)?;
    assert!(content.contains("test_result"));
    assert!(content.contains("test_method"));

    Ok(())
}

#[test]
fn test_log_io_params_for_method() -> io::Result<()> {
    let (mut logger, temp_dir) = setup();
    let method_name = "test_method";
    let result = logger.log_io_params_for_method(method_name, || "test_result")?;

    assert_eq!(result, "test_result");
    let file_path = temp_dir.path().join(format!("{}.jsonl", method_name));
    assert!(file_path.exists());

    let content = fs::read_to_string(file_path)?;
    assert!(content.contains("test_result"));
    assert!(content.contains(method_name));

    Ok(())
}

#[test]
fn test_run_over_logs() -> io::Result<()> {
    let (logger, temp_dir) = setup();

    let input_path = temp_dir.path().join("input.jsonl");
    let test_input = json!({
        Constants::ID: "test_id",
        Constants::INPUTS: {"input": "test"},
        Constants::OUTPUTS: {"output": "test"},
        Constants::META: {"meta": "test"}
    });
    FileUtils::append_as_jsonl(&input_path, &test_input)?;

    logger.run_over_logs(
        "test_eval",
        |id, inputs, outputs, meta| {
            assert_eq!(id, "test_id");
            assert_eq!(inputs["input"], "test");
            assert_eq!(outputs["output"], "test");
            assert_eq!(meta["meta"], "test");
            Constants::EVAL_RESULT
        },
        &input_path,
    )?;

    let eval_path = temp_dir.path().join("test_eval_input.jsonl");
    assert!(eval_path.exists());

    let content = fs::read_to_string(eval_path)?;
    assert!(content.contains("eval_result"));
    assert!(content.contains("test_id"));

    Ok(())
}
