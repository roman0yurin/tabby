use std::sync::Arc;
use tabby_common::config::HttpModelConfig;
use tabby_inference::{ChatCompletionStream, ExtendedOpenAIConfig};
use super::rate_limit;
use crate::{create_reqwest_client, AZURE_API_VERSION};
use async_openai_alt::config::AzureConfig;
use async_openai_alt::Client;
use async_openai_alt::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionResponse,
    },
};
use async_trait::async_trait;

pub async fn create(model: &HttpModelConfig) -> Arc<dyn ChatCompletionStream> {
    let api_endpoint = model
        .api_endpoint
        .as_deref()
        .expect("api_endpoint is required");

    let engine: Box<dyn ChatCompletionStream> = match model.kind.as_str() {
        "azure/chat" => {
            let config = async_openai_alt::config::AzureConfig::new()
                .with_api_base(api_endpoint)
                .with_api_key(model.api_key.clone().unwrap_or_default())
                .with_api_version(AZURE_API_VERSION.to_string())
                .with_deployment_id(model.model_name.as_deref().expect("Model name is required"));
            Box::new(
                async_openai_alt::Client::with_config(config)
                    .with_http_client(create_reqwest_client(api_endpoint)),
            )
        }
        "openai/chat" | "mistral/chat" => {
            let config = OpenAIConfig::default()
                .with_api_base(api_endpoint)
                .with_api_key(model.api_key.clone().unwrap_or_default());

            let mut builder = ExtendedOpenAIConfig::builder();
            builder
                .base(config)
                .kind(model.kind.clone())
                .supported_models(model.supported_models.clone())
                .model_name(model.model_name.as_deref().expect("Model name is required"));

            Box::new(
                async_openai_alt::Client::with_config(
                    builder.build().expect("Failed to build config"),
                )
                .with_http_client(create_reqwest_client(api_endpoint)),
            )
        }
        _ => panic!("Unsupported model kind: {}", model.kind),
    };

    Arc::new(rate_limit::new_chat(
        engine,
        model.rate_limit.request_per_minute,
    ))
}

/// Обёртка, содержащая несколько внутренних потоков для чата, каждый со своим набором supported_models
pub struct MultiChatCompletion {
    handlers: Vec<HandlerEntry>,
}

/// Данные о конкретном обработчике: список названий моделей, а также поток ChatCompletionStream
struct HandlerEntry {
    supported: Vec<String>,
    engine: Arc<dyn ChatCompletionStream>,
}

impl MultiChatCompletion {
    /// Создаёт MultiChatCompletion из списка HttpModelConfig
    /// Каждый HttpModelConfig → создаём движок (Box<dyn ChatCompletionStream>) и сохраняем
    /// вместе с supported_models
    pub async fn new(configs: &[HttpModelConfig]) -> Arc<dyn ChatCompletionStream> {
        let mut entries = Vec::new();
        for conf in configs {
            // Инициализируем конкретный ChatCompletionStream
            let engine = create_logging_chat(conf).await; // вместо build_single_engine
            // Список поддерживаемых моделей
            let models = conf.supported_models.clone().unwrap_or_default();

            entries.push(HandlerEntry {
                supported: models,
                engine,
            });
        }
        Arc::new(MultiChatCompletion { handlers: entries })
    }
}


#[async_trait]
impl ChatCompletionStream for MultiChatCompletion {
    async fn chat(
        &self,
        mut request: CreateChatCompletionRequest
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        // Пытаемся найти движок, который поддерживает request.model
        // Если model не указана, нужно дополнительную логику
        let model_name = if request.model.is_empty() {
            // Берём первую по умолчанию или ошибку, при необходимости
            request.model.clone()
        } else {
            request.model.clone()
        };

        for entry in &self.handlers {
            if entry.supported.is_empty() || entry.supported.contains(&model_name) {
                return entry.engine.chat(request).await;
            }
        }
        Err(OpenAIError::ApiError(async_openai_alt::error::ApiError {
            message: "No matching model found".to_string(),
            r#type: None,
            param: None,
            code: None,
        }))
    }

    async fn chat_stream(
        &self,
        mut request: CreateChatCompletionRequest,
    ) -> Result<ChatCompletionResponseStream, OpenAIError> {
        let model_name = if request.model.is_empty() {
            request.model.clone()
        } else {
            request.model.clone()
        };

        for entry in &self.handlers {
            if entry.supported.is_empty() || entry.supported.contains(&model_name) {
                return entry.engine.chat_stream(request).await;
            }
        }
        Err(OpenAIError::ApiError(async_openai_alt::error::ApiError {
            message: "No matching model found".to_string(),
            r#type: None,
            param: None,
            code: None,
        }))
    }
}

// Новый тип-обёртка, который логирует наиболее важные поля запроса
pub struct LoggingChatCompletionStream {
    inner: Arc<dyn ChatCompletionStream>,
    api_endpoint: String,
    model_type: String,
}

#[async_trait]
impl ChatCompletionStream for LoggingChatCompletionStream {
    async fn chat(
        &self,
        mut request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        // Выводим в логи предварительную информацию по основным полям...
        println!("=== Chat Request ===");
        println!("API endpoint: {}", self.api_endpoint);
        println!("Model type: {}", self.model_type);
        // Выводим полный Debug-отчет запроса, показывающий все поля, имеющие текстовое представление
        println!("Full request: {:#?}", request);
        println!("====================");
        self.inner.chat(request).await
    }

    async fn chat_stream(
        &self,
        mut request: CreateChatCompletionRequest,
    ) -> Result<ChatCompletionResponseStream, OpenAIError> {
        println!("=== Chat Stream Request ===");
        println!("API endpoint: {}", self.api_endpoint);
        println!("Model type: {}", self.model_type);
        println!("Full request: {:#?}", request);
        println!("===========================");
        self.inner.chat_stream(request).await
    }
}
// Функция, которая создает обёртку LoggingChatCompletionStream, используя базовую реализацию create()
pub async fn create_logging_chat(model: &HttpModelConfig) -> Arc<dyn ChatCompletionStream> {
    let inner = create(model).await; // базовая функция создания ChatCompletionStream
    let api_endpoint = model.api_endpoint.clone().unwrap_or_default();
    let model_type = model.kind.clone();
    Arc::new(LoggingChatCompletionStream { inner, api_endpoint, model_type })
}

/// Переименованный create, теперь принимает список HttpModelConfig
pub async fn create_multi_chat(configs: Vec<HttpModelConfig>) -> Arc<dyn ChatCompletionStream> {
    MultiChatCompletion::new(&configs).await
}
