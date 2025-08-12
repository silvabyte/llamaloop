use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Model {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    pub details: Option<ModelDetails>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Option<Vec<String>>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunningModel {
    pub name: String,
    pub model: String,
    pub size: u64,
    pub digest: String,
    pub details: Option<ModelDetails>,
    pub expires_at: String,
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

#[derive(Debug, Deserialize)]
pub struct PullResponse {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub response: String,
    pub done: bool,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn check_status(&self) -> Result<bool> {
        match self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let response = self.client
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
        let response = self.client
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

        let response = self.client
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

        let response = self.client
            .delete(format!("{}/api/delete", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to delete model: {}", response.status());
        }

        Ok(())
    }

    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String> {
        let request = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to generate: {}", response.status());
        }

        let generate_response: GenerateResponse = response.json().await?;
        Ok(generate_response.response)
    }
}