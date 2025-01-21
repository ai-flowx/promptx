use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Default)]
pub struct FileUtils {}

impl FileUtils {
    pub fn read_jsonl<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<Value>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut jsonl_list = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let json_object: Value = serde_json::from_str(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            jsonl_list.push(json_object);
        }

        Ok(jsonl_list)
    }

    pub fn read_jsonl_row<P: AsRef<Path>>(
        file_path: P,
    ) -> io::Result<impl Iterator<Item = io::Result<Value>>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        Ok(reader.lines().map(|line_result| {
            line_result.and_then(|line| {
                serde_json::from_str(&line)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        }))
    }

    pub fn append_as_jsonl<P: AsRef<Path>>(file_path: P, args_to_log: &Value) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        writeln!(file, "{}", serde_json::to_string(args_to_log)?)?;
        Ok(())
    }

    pub fn save_jsonlist<P: AsRef<Path>>(
        file_path: P,
        json_list: &[Value],
        mode: &str,
    ) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(mode == "a")
            .truncate(mode != "a")
            .open(file_path)?;

        for json_obj in json_list {
            writeln!(file, "{}", serde_json::to_string(json_obj)?)?;
        }

        Ok(())
    }

    pub fn str_list_to_dir_path(str_list: &[&str]) -> PathBuf {
        str_list
            .iter()
            .fold(PathBuf::new(), |path, dir| path.join(dir))
    }
}
