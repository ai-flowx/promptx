use super::llm::*;
use crate::config::config::{ConfigData, ConfigLLM};

#[tokio::test]
async fn test_chat_completion_success() {
    let config = ConfigData {
        llm: vec![ConfigLLM {
            name: "openai".to_string(),
            api: "https://api.openai.com/v1/engines/davinci-codex/completions".to_string(),
            key: "test_key".to_string(),
            endpoint: "davinci-codex".to_string(),
        }],
    };
    let llm = LLM::new(config);
    let messages = vec![Message {
        role: "user".to_string(),
        content: "Hello, world!".to_string(),
    }];

    let result = llm.chat_completion("openai".to_string(), messages).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_chat_completion_unsupported_llm() {
    let config = ConfigData { llm: vec![] };
    let llm = LLM::new(config);
    let messages = vec![Message {
        role: "user".to_string(),
        content: "Hello, world!".to_string(),
    }];

    let result = llm
        .chat_completion("unsupported".to_string(), messages)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_call_openai_api_success() {
    let config = ConfigLLM {
        name: "openai".to_string(),
        api: "https://api.openai.com/v1/engines/davinci-codex/completions".to_string(),
        key: "test_key".to_string(),
        endpoint: "davinci-codex".to_string(),
    };
    let llm = LLM::new(ConfigData {
        llm: vec![config.clone()],
    });
    let messages = vec![Message {
        role: "user".to_string(),
        content: "Hello, world!".to_string(),
    }];

    let result = llm.call_openai_api(config, messages).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_model_type() {
    let config = ConfigData {
        llm: vec![
            ConfigLLM {
                name: "doubao".to_string(),
                api: "https://ark.cn-beijing.volces.com/api/v3/chat/completions".to_string(),
                key: "test_key".to_string(),
                endpoint: "doubao-1.5-pro-32k".to_string(),
            },
            ConfigLLM {
                name: "openai".to_string(),
                api: "https://api.openai.com/v1/engines/davinci-codex/completions".to_string(),
                key: "test_key".to_string(),
                endpoint: "davinci-codex".to_string(),
            },
        ],
    };
    let llm = LLM::new(config);

    let model_types = llm.list_model_type();
    assert_eq!(
        model_types,
        vec!["doubao".to_string(), "openai".to_string()]
    );
}
