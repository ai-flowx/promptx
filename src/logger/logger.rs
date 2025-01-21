use super::constants::*;
use super::file_utils::*;
use chrono::Utc;
use serde::Serialize;
use serde_json::{json, Value};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Instant;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct Logger {
    pub base_path: PathBuf,
    pub sample_unq_id: Option<Uuid>,
    pub chained_log: Vec<Value>,
    pub del_self_arg: bool,
}

impl Logger {
    pub fn new<P: AsRef<Path>>(base_path: P) -> io::Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        if !base_path.as_os_str().is_empty() {
            fs::create_dir_all(&base_path)?;
        }

        Ok(Logger {
            base_path,
            sample_unq_id: None,
            chained_log: Vec::new(),
            del_self_arg: true,
        })
    }

    pub fn reset_eval_glue<P: AsRef<Path>>(&mut self, base_path: P) -> io::Result<()> {
        self.base_path = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&self.base_path)?;
        self.sample_unq_id = None;
        self.chained_log.clear();
        Ok(())
    }

    pub fn clear_chained_log(&mut self) {
        self.chained_log.clear();
    }

    pub fn dump_chained_log_to_file(&mut self, file_name: &str) -> io::Result<()> {
        let file_path = self.base_path.join(format!("{}.jsonl", file_name));
        FileUtils::save_jsonlist(&file_path, &self.chained_log, "a")?;
        self.clear_chained_log();
        Ok(())
    }

    pub fn append_dict_to_chained_logs(&mut self, args_to_log: Value) {
        self.chained_log.push(args_to_log);
    }

    pub fn append_to_chained_log<F, T>(&mut self, method_name: &str, method: F) -> T
    where
        F: FnOnce() -> T,
        T: Serialize,
    {
        let start = Instant::now();
        let result = method();
        let duration = start.elapsed();

        let args_to_log = json!({
            Constants::INPUTS: {},
            Constants::OUTPUTS: &result,
            Constants::META: {
                Constants::METHOD_NAME: method_name,
                Constants::EXEC_SEC: duration.as_secs_f64(),
                Constants::TIMESTAMP: Utc::now().to_rfc3339()
            }
        });

        self.chained_log.push(args_to_log);
        result
    }

    pub fn log_io_params<F, T>(
        &mut self,
        method_name: &str,
        method: F,
        file_name: &str,
    ) -> io::Result<T>
    where
        F: FnOnce() -> T,
        T: Serialize,
    {
        let start = Instant::now();
        let result = method();
        let duration = start.elapsed();

        if self.sample_unq_id.is_none() {
            self.sample_unq_id = Some(Uuid::new_v4());
        }

        let args_to_log = json!({
            Constants::ID: self.sample_unq_id.unwrap().to_string(),
            Constants::INPUTS: {},
            Constants::OUTPUTS: &result,
            Constants::META: {
                Constants::METHOD_NAME: method_name,
                Constants::EXEC_SEC: duration.as_secs_f64(),
                Constants::TIMESTAMP: Utc::now().to_rfc3339()
            }
        });

        let file_path = self.base_path.join(format!("{}.jsonl", file_name));
        FileUtils::append_as_jsonl(&file_path, &args_to_log)?;
        self.sample_unq_id = None;

        Ok(result)
    }

    pub fn log_io_params_for_method<F, T>(&mut self, method_name: &str, method: F) -> io::Result<T>
    where
        F: FnOnce() -> T,
        T: Serialize,
    {
        self.log_io_params(method_name, method, method_name)
    }

    pub fn run_over_logs<F, T>(
        &self,
        method_name: &str,
        eval_method: F,
        file_path: &Path,
    ) -> io::Result<()>
    where
        F: Fn(&str, &Value, &Value, &Value) -> T,
        T: Serialize,
    {
        let eval_file_path = self.base_path.join(format!(
            "{}_{}",
            method_name,
            file_path.file_name().unwrap().to_string_lossy()
        ));

        for json_obj in FileUtils::read_jsonl_row(file_path)? {
            let json_obj = json_obj?;
            let eval_result = eval_method(
                json_obj[Constants::ID].as_str().unwrap_or_default(),
                &json_obj[Constants::INPUTS],
                &json_obj[Constants::OUTPUTS],
                &json_obj[Constants::META],
            );
            let args_to_log = json!({
                Constants::ID: json_obj[Constants::ID],
                Constants::EVAL_RESULT: eval_result,
                Constants::META: {
                    Constants::TIMESTAMP: Utc::now().to_rfc3339()
                }
            });
            FileUtils::append_as_jsonl(&eval_file_path, &args_to_log)?;
        }

        Ok(())
    }
}
