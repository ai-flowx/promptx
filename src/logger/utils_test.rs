use super::utils::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[test]
fn test_run_method_get_io_dict() {
    let test_fn = |args: &MethodArgs<String>| -> String {
        thread::sleep(Duration::from_millis(10));
        format!("Processed {} args", args.args.len())
    };

    let mut kwargs = HashMap::new();
    kwargs.insert("test_key".to_string(), json!("test_value"));

    let args = MethodArgs {
        args: vec!["test1".to_string(), "test2".to_string()],
        kwargs,
    };

    let result = Utils::run_method_get_io_dict(test_fn, "test_method", args).unwrap();

    assert!(result.meta.execution_time > 0.0);
    assert!(result.meta.execution_time < 1.0);

    assert_eq!(result.meta.method_name, "test_method");

    assert!(chrono::DateTime::parse_from_rfc3339(&result.meta.timestamp).is_ok());

    if let Value::Object(inputs) = result.inputs {
        assert_eq!(inputs.get("arg_0").unwrap(), "test1");
        assert_eq!(inputs.get("arg_1").unwrap(), "test2");
        assert_eq!(inputs.get("test_key").unwrap(), "test_value");
    } else {
        panic!("Expected Object value for inputs");
    }

    assert_eq!(result.outputs, "Processed 2 args");
}

#[test]
fn test_method_args_to_inputs() {
    let mut kwargs = HashMap::new();
    kwargs.insert("key1".to_string(), json!("value1"));
    kwargs.insert("key2".to_string(), json!(42));

    let args = MethodArgs {
        args: vec!["arg1", "arg2"],
        kwargs,
    };

    let inputs = args.to_inputs();

    if let Value::Object(map) = inputs {
        assert_eq!(map.get("arg_0").unwrap(), "arg1");
        assert_eq!(map.get("arg_1").unwrap(), "arg2");
        assert_eq!(map.get("key1").unwrap(), "value1");
        assert_eq!(map.get("key2").unwrap(), 42);
    } else {
        panic!("Expected Object value");
    }
}

#[test]
fn test_method_args_empty() {
    let args: MethodArgs<String> = MethodArgs {
        args: vec![],
        kwargs: HashMap::new(),
    };

    let inputs = args.to_inputs();

    if let Value::Object(map) = inputs {
        assert!(map.is_empty());
    } else {
        panic!("Expected empty Object value");
    }
}

#[test]
fn test_method_args_null_values() {
    let mut kwargs = HashMap::new();
    kwargs.insert("null_key".to_string(), Value::Null);

    let args: MethodArgs<String> = MethodArgs {
        args: vec![],
        kwargs,
    };

    let inputs = args.to_inputs();

    if let Value::Object(map) = inputs {
        assert_eq!(map.get("null_key").unwrap(), &Value::Null);
    } else {
        panic!("Expected Object value with null");
    }
}
