use super::exceptions::*;
use std::error::Error;

#[test]
fn test_glue_error() {
    let msg = "test error";
    let error = GlueError::new(msg);

    assert_eq!(error.to_string(), msg);
    assert!(error.source().is_none());
}

#[test]
fn test_glue_llm_error() {
    let msg = "llm error";
    let exception = "stack trace";
    let error = GlueLLMError::new(msg, exception);

    let expected = format!(
        "LLM exception\nException: {}\nException logs: {}",
        msg, exception
    );
    assert_eq!(error.to_string(), expected);
    assert!(error.source().is_none());
}

#[test]
fn test_glue_validation_error() {
    let msg = "validation error";
    let exception = "invalid input";
    let error = GlueValidationError::new(msg, exception);

    let expected = format!(
        "[Invalid user input detected]\nException: {}\nException logs: {}",
        msg, exception
    );
    assert_eq!(error.to_string(), expected);
    assert!(error.source().is_none());
}

#[test]
fn test_error_conversions() {
    let llm_error = GlueLLMError::new("llm error", "stack trace");
    let glue_error: GlueError = llm_error.into();
    assert!(glue_error.to_string().contains("LLM exception"));

    let validation_error = GlueValidationError::new("validation error", "invalid input");
    let glue_error: GlueError = validation_error.into();
    assert!(glue_error
        .to_string()
        .contains("Invalid user input detected"));
}

#[test]
fn test_error_debug_format() {
    let error = GlueError::new("test error");
    assert!(format!("{:?}", error).contains("GlueError"));

    let llm_error = GlueLLMError::new("llm error", "stack trace");
    assert!(format!("{:?}", llm_error).contains("GlueLLMError"));

    let validation_error = GlueValidationError::new("validation error", "invalid input");
    assert!(format!("{:?}", validation_error).contains("GlueValidationError"));
}
