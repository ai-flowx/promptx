use super::constants::*;

#[test]
fn test_constants() {
    assert_eq!(Constants::INPUTS, "inputs");
    assert_eq!(Constants::OUTPUTS, "outputs");
    assert_eq!(Constants::META, "meta");
    assert_eq!(Constants::ID, "id");
    assert_eq!(Constants::TIMESTAMP, "timestamp");
    assert_eq!(Constants::EXEC_SEC, "execution_time_sec");
    assert_eq!(Constants::EVAL_RESULT, "eval_result");
    assert_eq!(Constants::METHOD_NAME, "method_name");
    assert_eq!(Constants::DIR_NAME, "io_logs");
}
