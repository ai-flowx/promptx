use super::constants::*;

#[test]
fn test_dirnames() {
    assert_eq!(DirNames::MODEL_DIR, "/tmp/model");
    assert_eq!(DirNames::PACKAGE_BASE_DIR, "/tmp/packages");
}

#[test]
fn test_fileconstants() {
    assert_eq!(FileConstants::LOGFILE_NAME, "glue_logs.log");
    assert_eq!(FileConstants::LOGFILE_PREFIX, "glue_logs_");
}

#[test]
fn test_llmoutputtypes() {
    assert_eq!(LLMOutputTypes::COMPLETION, "completion");
    assert_eq!(LLMOutputTypes::CHAT, "chat");
    assert_eq!(LLMOutputTypes::EMBEDDINGS, "embeddings");
    assert_eq!(LLMOutputTypes::MULTI_MODAL, "multimodal");
}
