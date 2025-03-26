use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ModelInfo {
    completion: Option<Vec<String>>,
    chat: Option<Vec<String>>,
}

impl From<tabby_common::config::Config> for ModelInfo {
    fn from(value: tabby_common::config::Config) -> Self {
        let models = value.model;
        let mut http_model_configs: ModelInfo = ModelInfo {
            completion: None,
            chat: None,
        };

        if let Some(tabby_common::config::ModelConfig::Http(completion_http_config)) =
            models.completion
        {
            if let Some(models) = completion_http_config.supported_models {
                http_model_configs.completion = Some(models.clone());
            }
        }

        let mut aggregated_chat_models = Vec::new();
        for config in models.chat {
            if let tabby_common::config::ModelConfig::Http(chat_http_config) = config {
                if let Some(supported) = chat_http_config.supported_models {
                    aggregated_chat_models.extend(supported);
                }
            }
        }
        http_model_configs.chat = if aggregated_chat_models.is_empty() {
            None
        } else {
            Some(aggregated_chat_models)
        };

        http_model_configs
    }
}

#[utoipa::path(
    get,
    path = "/v1beta/models",
    tag = "v1beta",
    operation_id = "config",
    responses(
        (status = 200, description = "Success", body = ServerSetting, content_type = "application/json"),
    ),
    security(
        ("token" = [])
    )
)]
pub async fn models(State(state): State<Arc<ModelInfo>>) -> Json<ModelInfo> {
    Json(state.as_ref().clone())
}
