use super::constants::*;

#[test]
fn test_commonlogsstr() {
    assert_eq!(
        CommonLogsStr::INSTALL_MISSING_LIB,
        "{lib_name} is not installed. Installing {lib_name}."
    );
    assert_eq!(
        CommonLogsStr::LOG_SEPERATOR,
        "\n".to_owned() + &*"=".repeat(10).to_owned() + &*"\n".to_owned()
    );
}

#[test]
fn test_dirnames() {
    assert_eq!(DirNames::MODEL_DIR, "model");
    assert_eq!(DirNames::PACKAGE_BASE_DIR, "/path/to/packages");
}

#[test]
fn test_fileconstants() {
    assert_eq!(FileConstants::LOGFILE_NAME, "glue_logs.log");
    assert_eq!(FileConstants::LOGFILE_PREFIX, "glue_logs_");
}

#[test]
fn test_installlibs() {
    assert_eq!(
        InstallLibs::LLAMA_LLM_AZ_OAI,
        "llama-index-llms-azure-openai==0.1.5"
    );
    assert_eq!(
        InstallLibs::LLAMA_EMB_AZ_OAI,
        "llama-index-embeddings-azure-openai==0.1.6"
    );
    assert_eq!(
        InstallLibs::LLAMA_MM_LLM_AZ_OAI,
        "llama-index-multi-modal-llms-azure-openai==0.1.4"
    );
    assert_eq!(InstallLibs::AZURE_CORE, "azure-core==1.30.1");
    assert_eq!(InstallLibs::TIKTOKEN, "tiktoken");
}

#[test]
fn test_llmliterals() {
    assert_eq!(LLMLiterals::EMBEDDING_TOKEN_COUNT, "embedding_token_count");
    assert_eq!(
        LLMLiterals::PROMPT_LLM_TOKEN_COUNT,
        "prompt_llm_token_count"
    );
    assert_eq!(
        LLMLiterals::COMPLETION_LLM_TOKEN_COUNT,
        "completion_llm_token_count"
    );
    assert_eq!(LLMLiterals::TOTAL_LLM_TOKEN_COUNT, "total_llm_token_count");
}

#[test]
fn test_llmoutputtypes() {
    assert_eq!(LLMOutputTypes::COMPLETION, "completion");
    assert_eq!(LLMOutputTypes::CHAT, "chat");
    assert_eq!(LLMOutputTypes::EMBEDDINGS, "embeddings");
    assert_eq!(LLMOutputTypes::MULTI_MODAL, "multimodal");
}

#[test]
fn test_oailiterals() {
    assert_eq!(OAILiterals::OPENAI_API_KEY, "OPENAI_API_KEY");
    assert_eq!(OAILiterals::OPENAI_API_BASE, "OPENAI_API_BASE");
    assert_eq!(OAILiterals::OPENAI_API_TYPE, "OPENAI_API_TYPE");
    assert_eq!(OAILiterals::OPENAI_API_VERSION, "OPENAI_API_VERSION");
    assert_eq!(OAILiterals::AZ_OPEN_AI_OBJECT, "AZ_OPEN_AI_OBJECT");
}

#[test]
fn test_vellmerrorstrings() {
    assert_eq!(
        VellmErrorStrings::PATH_DOESNT_EXIST,
        "{path} path doesn't exist. Please create path {path}"
    );
}
