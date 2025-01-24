use super::constants::*;
use anyhow::{Context, Result};
use libloading::Library;
use log::{LevelFilter, SetLoggerError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml::Value;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;

use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};

#[derive(Clone, Default)]
pub struct Download {}

impl Download {
    pub async fn download_model(url: &str) -> Result<PathBuf> {
        let cwd = std::env::current_dir().context("Failed to get current working directory")?;

        let parts: Vec<_> = cwd.components().collect();
        let idx = parts
            .iter()
            .position(|comp| comp.as_os_str() == DirNames::PACKAGE_BASE_DIR)
            .unwrap_or(0);

        let download_path = parts[..=idx]
            .iter()
            .collect::<PathBuf>()
            .join(DirNames::MODEL_DIR);

        fs::create_dir_all(&download_path).context("Failed to create download directory")?;

        let parsed_url = Url::parse(url).context("Failed to parse URL")?;
        let model_filename = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .context("Failed to get filename from URL")?;

        let model_path = download_path.join(model_filename);

        if !model_path.exists() {
            let client = Client::new();
            let mut response = client
                .get(url)
                .send()
                .await
                .context("Failed to send request")?;

            if response.status().is_success() {
                let mut file = File::create(&model_path).context("Failed to create file")?;

                while let Ok(Some(chunk)) = response.chunk().await {
                    file.write_all(&chunk)
                        .context("Failed to write chunk to file")?;
                    file.flush().context("Failed to flush file")?;
                }
            } else {
                anyhow::bail!("Failed to download file: {}", response.status());
            }
        }

        Ok(model_path)
    }
}

#[derive(Clone, Default)]
pub struct FileUtils {}

impl FileUtils {
    pub fn yaml_to_dict<P: AsRef<Path>>(file_path: P) -> Result<Value> {
        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open YAML file at {:?}", file_path.as_ref()))?;

        serde_yaml::from_reader(file)
            .with_context(|| format!("Failed to parse YAML file at {:?}", file_path.as_ref()))
    }

    pub fn yaml_to_class<T, P>(
        yaml_file_path: Option<P>,
        default_yaml_file_path: Option<P>,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        P: AsRef<Path> + Clone,
    {
        let yaml_file_path = match (yaml_file_path, default_yaml_file_path.as_ref()) {
            (Some(path), _) => path,
            (None, Some(path)) => path.clone(),
            (None, None) => return Err(anyhow::anyhow!("No YAML file path provided")),
        };

        let mut custom_args = FileUtils::yaml_to_dict(&yaml_file_path)?;

        if let Some(default_path) = default_yaml_file_path {
            let default_args = FileUtils::yaml_to_dict(default_path)?;

            if let (Value::Mapping(mut custom), Value::Mapping(default)) =
                (custom_args.clone(), default_args)
            {
                for (key, value) in default {
                    if !custom.contains_key(&key) {
                        custom.insert(key, value);
                    }
                }
                custom_args = Value::Mapping(custom);
            }
        }

        serde_yaml::from_value(custom_args).with_context(|| {
            format!(
                "Failed to convert YAML to class from file {:?}",
                yaml_file_path.as_ref()
            )
        })
    }

    pub fn read_jsonl<P: AsRef<Path>>(file_path: P) -> Result<Vec<Value>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut jsonl_list = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                let json_object: Value = serde_json::from_str(&line)?;
                jsonl_list.push(json_object);
            }
        }

        Ok(jsonl_list)
    }

    pub fn read_jsonl_row<P: AsRef<Path>>(
        file_path: P,
    ) -> io::Result<impl Iterator<Item = Result<Value>>> {
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);

        Ok(reader
            .lines()
            .filter_map(move |line_result| match line_result {
                Ok(line) => {
                    if line.trim().is_empty() {
                        return None;
                    }
                    match serde_json::from_str(&line) {
                        Ok(json_object) => Some(Ok(json_object)),
                        Err(e) => {
                            eprintln!(
                                "Error while reading JSONL file at {:?}: {}",
                                file_path.as_ref(),
                                e
                            );
                            None
                        }
                    }
                }
                Err(e) => Some(Err(e.into())),
            }))
    }

    pub fn append_as_jsonl<T, P>(file_path: P, args_to_log: &T) -> Result<()>
    where
        T: Serialize,
        P: AsRef<Path>,
    {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        let json_str = serde_json::to_string(args_to_log)?;
        writeln!(file, "{}", json_str)?;
        Ok(())
    }

    pub fn save_jsonlist<T, P>(file_path: P, json_list: &[T], append: bool) -> Result<()>
    where
        T: Serialize,
        P: AsRef<Path>,
    {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .truncate(!append)
            .open(file_path)?;

        for json_obj in json_list {
            let json_str = serde_json::to_string(json_obj)?;
            writeln!(file, "{}", json_str)?;
        }
        Ok(())
    }

    pub fn str_list_to_dir_path<S: AsRef<str>>(str_list: &[S]) -> PathBuf {
        str_list.iter().fold(PathBuf::new(), |path, dir_name| {
            path.join(dir_name.as_ref())
        })
    }
}

pub struct Logger {}

impl Logger {
    pub fn set_console_logger(
        &mut self,
        module_name: &str,
    ) -> std::result::Result<Handle, SetLoggerError> {
        let pattern = "{d(%Y-%m-%d,%H:%M:%S)}.{ms:03} | {M} | {f}:\n{m}\n";

        let console_appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build();

        let console = Appender::builder().build(
            &format!("{}_console", module_name),
            Box::new(console_appender),
        );

        let config = Config::builder()
            .appender(console)
            .build(
                Root::builder()
                    .appender(&format!("{}_console", module_name))
                    .build(LevelFilter::Debug),
            )
            .unwrap();

        log4rs::init_config(config)
    }

    pub fn set_file_logger(
        &mut self,
        module_name: &str,
        log_dirpath: &str,
    ) -> std::result::Result<Handle, SetLoggerError> {
        fs::create_dir_all(log_dirpath).expect("Failed to create log dir");

        let pattern = "{d(%Y-%m-%d,%H:%M:%S)}.{ms:03} | {M} | {f}:\n{m}\n";
        let log_path = Path::new(log_dirpath).join(FileConstants::LOGFILE_NAME);

        let roller = FixedWindowRoller::builder()
            .build(
                &format!("{}/{}.{{}}.log", log_dirpath, FileConstants::LOGFILE_PREFIX),
                30,
            )
            .unwrap();

        let trigger = SizeTrigger::new(50 * 1024 * 1024);
        let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

        let file_appender = RollingFileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build(log_path, Box::new(policy))
            .expect("Failed to create log file");

        let file =
            Appender::builder().build(&format!("{}_file", module_name), Box::new(file_appender));

        let config = Config::builder()
            .appender(file)
            .build(
                Root::builder()
                    .appender(&format!("{}_file", module_name))
                    .build(LevelFilter::Debug),
            )
            .unwrap();

        log4rs::init_config(config)
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    PackageNotFound(String),
    InstallationError(String),
    ValidationError(String),
    ModuleLoadError(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::PackageNotFound(msg) => write!(f, "Package not found: {}", msg),
            RuntimeError::InstallationError(msg) => write!(f, "Installation error: {}", msg),
            RuntimeError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            RuntimeError::ModuleLoadError(msg) => write!(f, "Module load error: {}", msg),
        }
    }
}

impl Error for RuntimeError {}

#[derive(Clone, Default)]
pub struct RuntimeTasks {}

impl RuntimeTasks {
    pub fn install_lib_if_missing(
        lib_name: &str,
        find_links: Option<&str>,
    ) -> Result<bool, RuntimeError> {
        let (package_name, _) = if let Some((name, ver)) = lib_name.split_once("==") {
            (name, Some(ver))
        } else {
            (lib_name, None)
        };

        let cargo_toml = Path::new("Cargo.toml");
        if cargo_toml.exists() {
            let content = fs::read_to_string(cargo_toml)
                .map_err(|e| RuntimeError::PackageNotFound(e.to_string()))?;

            if content.contains(package_name) {
                // TODO: Add version checking
                return Ok(true);
            }
        }

        let mut cmd = Command::new("cargo");
        cmd.arg("add").arg(package_name);

        if let Some(links) = find_links {
            cmd.arg("--registry").arg(links);
        }

        match cmd.output() {
            Ok(output) if output.status.success() => Ok(false),
            Ok(output) => Err(RuntimeError::InstallationError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            )),
            Err(e) => Err(RuntimeError::InstallationError(e.to_string())),
        }
    }

    pub fn str_to_class(
        import_path: Option<&str>,
        file_path: Option<&Path>,
    ) -> Result<Library, RuntimeError> {
        if let Some(path) = import_path {
            let lib_path = PathBuf::from(path).with_extension(std::env::consts::DLL_EXTENSION);
            unsafe {
                Library::new(lib_path).map_err(|e| RuntimeError::ModuleLoadError(e.to_string()))
            }
        } else if let Some(path) = file_path {
            unsafe { Library::new(path).map_err(|e| RuntimeError::ModuleLoadError(e.to_string())) }
        } else {
            Err(RuntimeError::ModuleLoadError(
                "Loading from current module not implemented".to_string(),
            ))
        }
    }
}
