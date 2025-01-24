use super::base::*;
use serde_json;

#[test]
fn test_llm_model_serialization() {
    let model = LLMModel {
        unique_model_id: "test-model".to_string(),
        model_type: "gpt".to_string(),
        track_tokens: "enabled".to_string(),
        req_per_min: 60,
        tokens_per_min: 40000,
        error_backoff_in_seconds: 5,
    };

    let serialized = serde_json::to_string(&model).unwrap();
    let deserialized: LLMModel = serde_json::from_str(&serialized).unwrap();

    assert_eq!(model.unique_model_id, deserialized.unique_model_id);
    assert_eq!(model.model_type, deserialized.model_type);
}

#[test]
fn test_user_limits() {
    let limits = UserLimits {
        max_num_requests_in_time_window: 100,
        time_window_length_in_seconds: 3600,
    };

    let serialized = serde_json::to_string(&limits).unwrap();
    let deserialized: UserLimits = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        limits.max_num_requests_in_time_window,
        deserialized.max_num_requests_in_time_window
    );
    assert_eq!(
        limits.time_window_length_in_seconds,
        deserialized.time_window_length_in_seconds
    );
}

#[test]
fn test_azure_aoi_models() {
    let base = LLMModel {
        unique_model_id: "azure-gpt".to_string(),
        model_type: "gpt4".to_string(),
        track_tokens: "enabled".to_string(),
        req_per_min: 30,
        tokens_per_min: 20000,
        error_backoff_in_seconds: 10,
    };

    let azure_model = AzureAOIModels {
        base,
        model_name_in_azure: "gpt-4".to_string(),
        deployment_name_in_azure: "prod-deployment".to_string(),
    };

    let serialized = serde_json::to_string(&azure_model).unwrap();
    let deserialized: AzureAOIModels = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        azure_model.model_name_in_azure,
        deserialized.model_name_in_azure
    );
    assert_eq!(
        azure_model.deployment_name_in_azure,
        deserialized.deployment_name_in_azure
    );
}

#[test]
fn test_operation_mode_serialization() {
    let online = OperationMode::Online;
    let offline = OperationMode::Offline;

    let serialized_online = serde_json::to_string(&online).unwrap();
    let serialized_offline = serde_json::to_string(&offline).unwrap();

    assert_eq!(serialized_online, "\"online\"");
    assert_eq!(serialized_offline, "\"offline\"");
}

#[test]
fn test_task_config_defaults() {
    let json = r#"{
        "name": "test-task",
        "prompt_template": "template",
        "llm_request_type": "chat"
    }"#;

    let task: TaskConfig = serde_json::from_str(json).unwrap();

    assert!(task.prepend_system_prompts);
    assert!(task.prepend_system_guidelines);
    assert_eq!(task.name, "test-task");
}

#[test]
fn test_prompt_library_config() {
    let config = PromptLibraryConfig {
        mode: Mode {
            chat: vec![TaskConfig {
                name: "chat-task".to_string(),
                prompt_template: "template".to_string(),
                llm_request_type: "chat".to_string(),
                prepend_system_prompts: true,
                prepend_system_guidelines: true,
                emb_model_id: None,
                llm_model_id: None,
            }],
            generation: vec![],
        },
        system_prompts: Some("system prompt".to_string()),
        system_guidelines: Some("guidelines".to_string()),
    };

    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: PromptLibraryConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.system_prompts, deserialized.system_prompts);
    assert_eq!(config.system_guidelines, deserialized.system_guidelines);
}
