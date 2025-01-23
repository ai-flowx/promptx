#[derive(Clone, Default)]
pub struct CommonLogsStr {}

impl CommonLogsStr {
    pub const INSTALL_MISSING_LIB: &'static str =
        "{lib_name} is not installed. Installing {lib_name}.";
    pub const LOG_SEPERATOR: &'static str = "\n==========\n";
}

#[derive(Clone, Default)]
pub struct DirNames {}

impl DirNames {
    pub const MODEL_DIR: &'static str = "model";
    pub const PACKAGE_BASE_DIR: &'static str = "/path/to/packages";
}

#[derive(Clone, Default)]
pub struct FileConstants {}

impl FileConstants {
    pub const LOGFILE_NAME: &'static str = "glue_logs.log";
    pub const LOGFILE_PREFIX: &'static str = "glue_logs_";
}

#[derive(Clone, Default)]
pub struct InstallLibs {}

impl InstallLibs {
    pub const LLAMA_LLM_AZ_OAI: &'static str = "llama-index-llms-azure-openai==0.1.5";
    pub const LLAMA_EMB_AZ_OAI: &'static str = "llama-index-embeddings-azure-openai==0.1.6";
    pub const LLAMA_MM_LLM_AZ_OAI: &'static str =
        "llama-index-multi-modal-llms-azure-openai==0.1.4";
    pub const AZURE_CORE: &'static str = "azure-core==1.30.1";
    pub const TIKTOKEN: &'static str = "tiktoken";
}

#[derive(Clone, Default)]
pub struct LLMLiterals {}

impl LLMLiterals {
    pub const EMBEDDING_TOKEN_COUNT: &'static str = "embedding_token_count";
    pub const PROMPT_LLM_TOKEN_COUNT: &'static str = "prompt_llm_token_count";
    pub const COMPLETION_LLM_TOKEN_COUNT: &'static str = "completion_llm_token_count";
    pub const TOTAL_LLM_TOKEN_COUNT: &'static str = "total_llm_token_count";
}

#[derive(Clone, Default)]
pub struct LLMOutputTypes {}

impl LLMOutputTypes {
    pub const COMPLETION: &'static str = "completion";
    pub const CHAT: &'static str = "chat";
    pub const EMBEDDINGS: &'static str = "embeddings";
    pub const MULTI_MODAL: &'static str = "multimodal";
}

#[derive(Clone, Default)]
pub struct OAILiterals {}

impl OAILiterals {
    pub const OPENAI_API_KEY: &'static str = "OPENAI_API_KEY";
    pub const OPENAI_API_BASE: &'static str = "OPENAI_API_BASE";
    pub const OPENAI_API_TYPE: &'static str = "OPENAI_API_TYPE";
    pub const OPENAI_API_VERSION: &'static str = "OPENAI_API_VERSION";
    pub const AZ_OPEN_AI_OBJECT: &'static str = "AZ_OPEN_AI_OBJECT";
}

#[derive(Clone, Default)]
pub struct VellmErrorStrings {}

impl VellmErrorStrings {
    pub const PATH_DOESNT_EXIST: &'static str =
        "{path} path doesn't exist. Please create path {path}";
}
