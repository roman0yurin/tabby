use std::{fs, sync::Arc};

pub use llama_cpp_server::PromptInfo;
use tabby_common::config::ModelConfig;
use tabby_download::{download_model, ModelKind};
use tabby_inference::{ChatCompletionStream, CodeGeneration, CompletionStream, Embedding};
use tracing::info;

pub async fn load_embedding(config: &ModelConfig) -> Arc<dyn Embedding> {
    llama_cpp_server::create_embedding(config).await
}

pub async fn load_code_generation_and_chat(
    completion_model: Option<ModelConfig>,
    chat_model: Vec<ModelConfig>,
) -> (
    Option<Arc<CodeGeneration>>,
    Option<Arc<dyn CompletionStream>>,
    Option<Arc<dyn ChatCompletionStream>>,
    Option<PromptInfo>,
) {
    let (engine, prompt_info, chat) =
        load_completion_and_chat(completion_model.clone(), chat_model).await;
    let code = engine
        .clone()
        .map(|engine| Arc::new(CodeGeneration::new(engine, completion_model)));
    (code, engine, chat, prompt_info)
}

async fn load_completion_and_chat(
    completion_model: Option<ModelConfig>,
    chat_model: Vec<ModelConfig>
) -> (
    Option<Arc<dyn CompletionStream>>,
    Option<PromptInfo>,
    Option<Arc<dyn ChatCompletionStream>>,
) {
    if let (Some(ModelConfig::Local(completion)), chat_vec) = (&completion_model, &chat_model) {
        if let Some(ModelConfig::Local(chat)) = chat_vec.first() {
            let (completion, prompt, chat) =
                llama_cpp_server::create_completion_and_chat(completion, chat).await;
            return (Some(completion), Some(prompt), Some(chat));
        }
    }

    let (completion, prompt) = if let Some(completion_model) = completion_model {
        match completion_model {
            ModelConfig::Http(http) => {
                let engine = http_api_bindings::create(&http).await;
                let (prompt_template, chat_template) =
                    http_api_bindings::build_completion_prompt(&http);
                (
                    Some(engine),
                    Some(PromptInfo {
                        prompt_template,
                        chat_template,
                    }),
                )
            }
            ModelConfig::Local(llama) => {
                let (stream, prompt) = llama_cpp_server::create_completion(&llama).await;
                (Some(stream), Some(prompt))
            }
        }
    } else {
        (None, None)
    };

    let chat: Option<Arc<dyn ChatCompletionStream>> = {
        // Собираем HTTP модели из вектора
        let http_models: Vec<_> = chat_model.iter().filter_map(|m| {
            if let ModelConfig::Http(http) = m { Some(http.clone()) } else { None }
        }).collect();
        if !http_models.is_empty() {
            // Если найдены HTTP модели, создаём чат через create_multi_chat
            Some(http_api_bindings::create_multi_chat(http_models).await)
        } else if let Some(ModelConfig::Local(local)) = chat_model.iter().find(|m| matches!(m, ModelConfig::Local(_))) {
            // Если отсутствуют HTTP, ожидается единственная локальная модель, обрабатываем её
            Some(llama_cpp_server::create_chat_completion(local).await)
        } else {
            None
        }
    };

    (completion, prompt, chat)
}

pub async fn download_model_if_needed(model: &str, kind: ModelKind) {
    if fs::metadata(model).is_ok() {
        info!("Loading model from local path {}", model);
    } else {
        download_model(model, true, Some(kind)).await;
    }
}
