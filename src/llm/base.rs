use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLMModel {
    pub unique_model_id: String,
    pub model_type: String,
    pub track_tokens: String,
    pub req_per_min: i32,
    pub tokens_per_min: i32,
    pub error_backoff_in_seconds: i32,
}

impl Display for LLMModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLimits {
    pub max_num_requests_in_time_window: i32,
    pub time_window_length_in_seconds: i32,
}

impl Display for UserLimits {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMQueueSchedulerLimits {
    pub ttl_in_seconds: i32,
    pub max_queue_size: i32,
}

impl Display for LLMQueueSchedulerLimits {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureAOIModels {
    #[serde(flatten)]
    pub base: LLMModel,
    pub model_name_in_azure: String,
    pub deployment_name_in_azure: String,
}

impl Display for AzureAOIModels {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureAOILM {
    pub api_key: String,
    pub api_version: String,
    pub api_type: String,
    pub azure_endpoint: String,
    pub azure_oai_models: Vec<AzureAOIModels>,
}

impl Display for AzureAOILM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomLLM {
    #[serde(flatten)]
    pub base: LLMModel,
    pub path_to_py_file: String,
    pub class_name: String,
}

impl Display for CustomLLM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMConfig {
    pub azure_open_ai: AzureAOILM,
    pub user_limits: UserLimits,
    pub scheduler_limits: LLMQueueSchedulerLimits,
    pub custom_models: Vec<CustomLLM>,
}

impl Display for LLMConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantLLM {
    pub prompt_opt: String,
}

impl Display for AssistantLLM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dir {
    pub base_dir: String,
    pub log_dir_name: String,
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationMode {
    Online,
    Offline,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupConfig {
    pub assistant_llm: AssistantLLM,
    pub dir_info: Dir,
    pub experiment_name: String,
    pub mode: OperationMode,
    pub description: String,
}

impl Display for SetupConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub prompt_template: String,
    pub llm_request_type: String,
    #[serde(default = "default_true")]
    pub prepend_system_prompts: bool,
    #[serde(default = "default_true")]
    pub prepend_system_guidelines: bool,
    pub emb_model_id: Option<String>,
    pub llm_model_id: Option<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mode {
    pub chat: Vec<TaskConfig>,
    pub generation: Vec<TaskConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptLibraryConfig {
    pub mode: Mode,
    pub system_prompts: Option<String>,
    pub system_guidelines: Option<String>,
}

impl Display for PromptLibraryConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
