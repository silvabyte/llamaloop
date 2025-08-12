use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use futures_util::StreamExt;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct OllamaClient {
    pub client: Client,
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Model {
    pub name: String,
    pub size: u64,
    pub details: Option<ModelDetails>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelDetails {
    pub parameter_size: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunningModel {
    pub name: String,
    pub size: u64,
    pub size_vram: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct ModelListResponse {
    models: Vec<Model>,
}

#[derive(Debug, Deserialize)]
struct ProcessListResponse {
    models: Vec<RunningModel>,
}

#[derive(Debug, Serialize)]
struct PullRequest {
    name: String,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct DeleteRequest {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelsDevModel {
    pub id: String,
    pub open_weights: Option<bool>,
    pub limit: Option<Limit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Limit {
    pub context: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Provider {
    pub models: HashMap<String, ModelsDevModel>,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn check_status(&self) -> Result<bool> {
        match self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch models: {}", response.status());
        }

        let model_list: ModelListResponse = response.json().await?;
        Ok(model_list.models)
    }

    pub async fn list_running_models(&self) -> Result<Vec<RunningModel>> {
        let response = self
            .client
            .get(format!("{}/api/ps", self.base_url))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch running models: {}", response.status());
        }

        let process_list: ProcessListResponse = response.json().await?;
        Ok(process_list.models)
    }

    pub async fn pull_model(&self, model_name: &str) -> Result<()> {
        let request = PullRequest {
            name: model_name.to_string(),
            stream: false,
        };

        let response = self
            .client
            .post(format!("{}/api/pull", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to pull model: {}", response.status());
        }

        Ok(())
    }

    pub async fn delete_model(&self, model_name: &str) -> Result<()> {
        let request = DeleteRequest {
            name: model_name.to_string(),
        };

        let response = self
            .client
            .delete(format!("{}/api/delete", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to delete model: {}", response.status());
        }

        Ok(())
    }

    pub async fn fetch_available_models(&self) -> Result<Vec<ModelsDevModel>> {
        let response = self
            .client
            .get("https://models.dev/api.json")
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to fetch models from models.dev: {}",
                response.status()
            );
        }

        // Parse as a map of providers
        let providers: HashMap<String, Provider> = response.json().await?;

        // Extract and filter for Ollama-compatible models
        let mut ollama_models: Vec<ModelsDevModel> = Vec::new();

        for (provider_name, provider) in providers {
            // Look for providers that have Ollama-compatible models
            if provider_name == "ollama"
                || provider_name == "llama"
                || provider_name == "meta"
                || provider_name == "mistral"
                || provider_name == "microsoft"
                || provider_name.contains("llama")
            {
                for (_, mut model) in provider.models {
                    // Only include open-weight models or known compatible ones
                    if model.open_weights.unwrap_or(false)
                        || model.id.contains("llama")
                        || model.id.contains("mistral")
                        || model.id.contains("phi")
                        || model.id.contains("gemma")
                        || model.id.contains("qwen")
                        || model.id.contains("deepseek")
                        || model.id.contains("codestral")
                        || model.id.contains("mixtral")
                    {
                        // Clean up the model ID for Ollama compatibility
                        if model.id.contains("/") {
                            // Extract the model name after the slash
                            if let Some(name) = model.id.split('/').next_back() {
                                model.id = name.to_string();
                            }
                        }

                        ollama_models.push(model);
                    }
                }
            }
        }

        // Deduplicate by model ID
        ollama_models.sort_by(|a, b| a.id.cmp(&b.id));
        ollama_models.dedup_by(|a, b| a.id == b.id);

        Ok(ollama_models)
    }

    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<serde_json::Value>,
        response_sender: mpsc::Sender<ChatResponse>,
    ) -> Result<()> {
        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: true,
        };

        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Chat request failed: {}", response.status());
        }

        let mut stream = response.bytes_stream();
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            
            // Parse each line as a separate JSON response
            for line in text.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                
                if let Ok(response) = serde_json::from_str::<ChatResponse>(line) {
                    let is_done = response.done.unwrap_or(false);
                    response_sender.send(response).await?;
                    
                    if is_done {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn generate(
        &self,
        model: &str,
        prompt: &str,
        response_sender: mpsc::Sender<GenerateResponse>,
    ) -> Result<()> {
        let request = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: true,
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Generate request failed: {}", response.status());
        }

        let mut stream = response.bytes_stream();
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            
            for line in text.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                
                if let Ok(response) = serde_json::from_str::<GenerateResponse>(line) {
                    let is_done = response.done;
                    response_sender.send(response).await?;
                    
                    if is_done {
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<serde_json::Value>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub model: Option<String>,
    pub created_at: Option<String>,
    pub message: Option<ChatMessage>,
    pub done: Option<bool>,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub context: Option<Vec<u32>>,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u64>,
}
