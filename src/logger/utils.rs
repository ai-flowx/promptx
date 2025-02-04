use chrono::Utc;
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::io;
use std::time::Instant;

#[derive(Clone, Default)]
pub struct Utils {}

#[derive(Debug, Serialize)]
pub struct MethodArgs<T> {
    pub args: Vec<T>,
    pub kwargs: HashMap<String, Value>,
}

#[derive(Debug, Serialize)]
pub struct MethodResult<T> {
    pub inputs: Value,
    pub outputs: T,
    pub meta: MethodMeta,
}

#[derive(Debug, Serialize)]
pub struct MethodMeta {
    pub execution_time: f64,
    pub timestamp: String,
    pub method_name: String,
}

impl Utils {
    pub fn run_method_get_io_dict<F, T, A>(
        method: F,
        method_name: &str,
        args: A,
    ) -> io::Result<MethodResult<T>>
    where
        F: FnOnce(&A) -> T,
        T: Serialize,
        A: ToInputs + Serialize,
    {
        // TBD: FIXME
        let start = Instant::now();
        let output = method(&args);
        let execution_time = start.elapsed().as_secs_f64();
        let meta = MethodMeta {
            execution_time,
            timestamp: Utc::now().to_rfc3339(),
            method_name: method_name.to_string(),
        };
        let inputs = args.to_inputs();
        Ok(MethodResult {
            inputs,
            outputs: output,
            meta,
        })
    }
}

pub trait ToInputs {
    fn to_inputs(&self) -> Value;
}

impl<T: Serialize> ToInputs for MethodArgs<T> {
    fn to_inputs(&self) -> Value {
        let mut inputs = Map::new();
        for (i, arg) in self.args.iter().enumerate() {
            inputs.insert(
                format!("arg_{}", i),
                serde_json::to_value(arg).unwrap_or(Value::Null),
            );
        }
        for (key, value) in &self.kwargs {
            inputs.insert(key.clone(), value.clone());
        }
        Value::Object(inputs)
    }
}
