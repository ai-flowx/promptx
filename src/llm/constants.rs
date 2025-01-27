#[derive(Clone, Default)]
pub struct DirNames {}

impl DirNames {
    pub const MODEL_DIR: &'static str = "/tmp/model";
    pub const PACKAGE_BASE_DIR: &'static str = "/tmp/packages";
}

#[derive(Clone, Default)]
pub struct FileConstants {}

impl FileConstants {
    pub const LOGFILE_NAME: &'static str = "glue_logs.log";
    pub const LOGFILE_PREFIX: &'static str = "glue_logs_";
}

#[derive(Clone, Default)]
pub struct LLMOutputTypes {}

impl LLMOutputTypes {
    pub const COMPLETION: &'static str = "completion";
    pub const CHAT: &'static str = "chat";
    pub const EMBEDDINGS: &'static str = "embeddings";
    pub const MULTI_MODAL: &'static str = "multimodal";
}
