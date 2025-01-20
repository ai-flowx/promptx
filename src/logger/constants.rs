#[derive(Clone, Default)]
pub struct Constants {}

impl Constants {
    pub const INPUTS: &'static str = "inputs";
    pub const OUTPUTS: &'static str = "outputs";
    pub const META: &'static str = "meta";
    pub const ID: &'static str = "id";
    pub const TIMESTAMP: &'static str = "timestamp";
    pub const EXEC_SEC: &'static str = "execution_time_sec";
    pub const EVAL_RESULT: &'static str = "eval_result";
    pub const METHOD_NAME: &'static str = "method_name";
    pub const DIR_NAME: &'static str = "io_logs";
}
