use crate::api::{Model, ModelsDevModel, OllamaClient, RunningModel, ChatResponse};
use crate::chat::{ChatState, MessageRole, InputMode};
use crate::theme::Sparkle;
use chrono::{DateTime, Local};
use ratatui::widgets::ListState;
use serde_json;
use std::collections::VecDeque;
use tokio::sync::mpsc;

pub struct App {
    pub selected_tab: usize,
    pub current_screen: CurrentScreen,
    pub should_quit: bool,
    pub models: Vec<Model>,
    pub available_models: Vec<ModelsDevModel>,
    pub running_models: Vec<RunningModel>,
    pub logs: VecDeque<LogEntry>,
    pub selected_model_index: usize,
    pub models_list_state: ListState,
    pub show_pull_dialog: bool,
    pub pull_model_name: String,
    pub show_delete_confirmation: bool,
    pub model_to_delete: Option<String>,
    pub ollama_client: OllamaClient,
    pub status: SystemStatus,
    pub last_refresh: DateTime<Local>,
    pub sparkle: Sparkle,
    pub animation_tick: usize,
    pub models_view_mode: ModelsViewMode,
    pub models_tab_view: ModelsTabView,
    pub api_explorer_state: ApiExplorerState,
    pub discovered_services: Vec<DiscoveredService>,
    pub chat_state: ChatState,
    pub chat_response_receiver: Option<mpsc::Receiver<ChatResponse>>,
}

#[derive(Clone, PartialEq)]
pub enum CurrentScreen {
    Dashboard,
    Models,
    Logs,
    Chat,
    Help,
}

#[derive(Clone, PartialEq)]
pub enum ModelsViewMode {
    Installed,
    Available,
    All,
}

#[derive(Clone, PartialEq)]
pub enum ModelsTabView {
    Library,    // Model library view (current models view)
    ApiExplorer, // API explorer view
    Network,     // Network discovery view
}

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct SystemStatus {
    pub is_running: bool,
    pub version: String,
    pub models_loaded: usize,
    pub total_memory: u64,
    pub used_memory: u64,
}

#[derive(Clone)]
pub struct ApiExplorerState {
    pub selected_endpoint: usize,
    pub endpoints_list_state: ListState,
    pub request_body: String,
    pub response_body: String,
    pub show_request_editor: bool,
    pub network_urls: Vec<String>,
}

#[derive(Clone)]
pub struct DiscoveredService {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub is_reachable: bool,
    pub version: Option<String>,
}

#[derive(Clone)]
pub struct ApiEndpoint {
    pub name: String,
    pub method: String,
    pub path: String,
    pub description: String,
    pub example_body: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let mut logs = VecDeque::new();
        logs.push_back(LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Info,
            message: "🔄 llamaloop initialized - entering the eternal dance".to_string(),
        });

        let mut models_list_state = ListState::default();
        models_list_state.select(Some(0));

        let mut api_endpoints_state = ListState::default();
        api_endpoints_state.select(Some(0));

        Self {
            selected_tab: 0,
            current_screen: CurrentScreen::Dashboard,
            should_quit: false,
            models: Vec::new(),
            available_models: Vec::new(),
            running_models: Vec::new(),
            logs,
            selected_model_index: 0,
            models_list_state,
            show_pull_dialog: false,
            pull_model_name: String::new(),
            show_delete_confirmation: false,
            model_to_delete: None,
            ollama_client: OllamaClient::new("http://localhost:11434".to_string()),
            status: SystemStatus {
                is_running: false,
                version: "Unknown".to_string(),
                models_loaded: 0,
                total_memory: 0,
                used_memory: 0,
            },
            last_refresh: Local::now(),
            sparkle: Sparkle::new(),
            animation_tick: 0,
            models_view_mode: ModelsViewMode::All,
            models_tab_view: ModelsTabView::Library,
            api_explorer_state: ApiExplorerState {
                selected_endpoint: 0,
                endpoints_list_state: api_endpoints_state,
                request_body: String::new(),
                response_body: String::new(),
                show_request_editor: false,
                network_urls: Vec::new(),
            },
            discovered_services: Vec::new(),
            chat_state: ChatState::new(Vec::new()),
            chat_response_receiver: None,
        }
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 4;
        self.update_screen_from_tab();
    }

    pub fn previous_tab(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = 3;
        }
        self.update_screen_from_tab();
    }

    fn update_screen_from_tab(&mut self) {
        self.current_screen = match self.selected_tab {
            0 => CurrentScreen::Dashboard,
            1 => CurrentScreen::Models,
            2 => CurrentScreen::Logs,
            3 => CurrentScreen::Chat,
            _ => CurrentScreen::Dashboard,
        };
        
        // Initialize chat if we're switching to it and haven't initialized yet
        if self.current_screen == CurrentScreen::Chat && self.chat_state.available_models.is_empty() {
            self.initialize_chat();
        }
    }

    pub fn toggle_models_tab(&mut self) {
        self.models_tab_view = match self.models_tab_view {
            ModelsTabView::Library => ModelsTabView::ApiExplorer,
            ModelsTabView::ApiExplorer => ModelsTabView::Network,
            ModelsTabView::Network => ModelsTabView::Library,
        };
        // Reset selections when switching tabs
        if self.models_tab_view == ModelsTabView::Library {
            self.models_list_state.select(Some(0));
        } else if self.models_tab_view == ModelsTabView::ApiExplorer {
            self.api_explorer_state.endpoints_list_state.select(Some(0));
        }
    }

    pub fn on_up(&mut self) {
        if self.current_screen == CurrentScreen::Models {
            let total_models = self.get_total_models_count();
            if total_models > 0 {
                let current = self.models_list_state.selected().unwrap_or(0);
                if current > 0 {
                    self.models_list_state.select(Some(current - 1));
                    self.selected_model_index = current - 1;
                }
            }
        }
    }

    pub fn on_down(&mut self) {
        if self.current_screen == CurrentScreen::Models {
            let total_models = self.get_total_models_count();
            if total_models > 0 {
                let current = self.models_list_state.selected().unwrap_or(0);
                if current < total_models - 1 {
                    self.models_list_state.select(Some(current + 1));
                    self.selected_model_index = current + 1;
                }
            }
        }
    }

    pub async fn on_enter(&mut self) {
        if self.show_pull_dialog && !self.pull_model_name.is_empty() {
            self.start_pull_model().await;
        }
    }

    pub async fn on_tick(&mut self) {
        self.animation_tick = self.animation_tick.wrapping_add(1);
        self.sparkle.update();

        if Local::now()
            .signed_duration_since(self.last_refresh)
            .num_seconds()
            > 5
        {
            self.refresh().await;
        }
    }

    pub async fn refresh(&mut self) {
        self.last_refresh = Local::now();

        match self.ollama_client.check_status().await {
            Ok(is_running) => {
                self.status.is_running = is_running;
                if is_running {
                    self.add_log(LogLevel::Info, "Ollama is running");

                    if let Ok(models) = self.ollama_client.list_models().await {
                        self.models = models;
                        self.status.models_loaded = self.models.len();
                    }

                    if let Ok(running) = self.ollama_client.list_running_models().await {
                        self.running_models = running;

                        self.status.used_memory = self.running_models.iter().map(|m| m.size).sum();
                    }
                } else {
                    self.add_log(LogLevel::Warning, "Ollama is not responding");
                }
            }
            Err(e) => {
                self.add_log(LogLevel::Error, &format!("Failed to check status: {e}"));
                self.status.is_running = false;
            }
        }

        // Fetch available models from models.dev if we haven't already
        if self.available_models.is_empty() {
            match self.ollama_client.fetch_available_models().await {
                Ok(available) => {
                    self.available_models = available;
                    self.add_log(
                        LogLevel::Info,
                        &format!(
                            "Loaded {} available models from models.dev",
                            self.available_models.len()
                        ),
                    );
                }
                Err(e) => {
                    self.add_log(
                        LogLevel::Warning,
                        &format!("Failed to fetch available models: {e}"),
                    );
                }
            }
        }
    }

    pub fn toggle_pull_dialog(&mut self) {
        self.show_pull_dialog = !self.show_pull_dialog;
        if self.show_pull_dialog {
            self.pull_model_name.clear();
        }
    }

    pub async fn start_pull_model(&mut self) {
        let model_name = self.pull_model_name.clone();
        self.show_pull_dialog = false;

        self.add_log(LogLevel::Info, &format!("Pulling model: {model_name}"));

        match self.ollama_client.pull_model(&model_name).await {
            Ok(_) => {
                self.add_log(
                    LogLevel::Info,
                    &format!("Successfully pulled model: {model_name}"),
                );
                self.refresh().await;
            }
            Err(e) => {
                self.add_log(LogLevel::Error, &format!("Failed to pull model: {e}"));
            }
        }
    }

    pub fn request_delete_model(&mut self) {
        if let Some(model) = self.models.get(self.selected_model_index) {
            self.model_to_delete = Some(model.name.clone());
            self.show_delete_confirmation = true;
            self.add_log(LogLevel::Warning, &format!("Delete confirmation requested for: {}", model.name));
        }
    }

    pub async fn confirm_delete_model(&mut self) {
        if let Some(model_name) = self.model_to_delete.clone() {
            self.add_log(LogLevel::Warning, &format!("Deleting model: {}", model_name));
            match self.ollama_client.delete_model(&model_name).await {
                Ok(_) => {
                    self.add_log(LogLevel::Info, &format!("Successfully deleted: {}", model_name));
                    self.refresh().await;
                }
                Err(e) => {
                    self.add_log(LogLevel::Error, &format!("Failed to delete {}: {}", model_name, e));
                }
            }
        }
        self.show_delete_confirmation = false;
        self.model_to_delete = None;
    }

    pub fn cancel_delete(&mut self) {
        self.show_delete_confirmation = false;
        self.model_to_delete = None;
        self.add_log(LogLevel::Info, "Delete cancelled");
    }

    fn add_log(&mut self, level: LogLevel, message: &str) {
        self.logs.push_back(LogEntry {
            timestamp: Local::now(),
            level,
            message: message.to_string(),
        });

        if self.logs.len() > 1000 {
            self.logs.pop_front();
        }
    }

    pub fn toggle_models_view(&mut self) {
        self.models_view_mode = match self.models_view_mode {
            ModelsViewMode::All => ModelsViewMode::Installed,
            ModelsViewMode::Installed => ModelsViewMode::Available,
            ModelsViewMode::Available => ModelsViewMode::All,
        };
        self.selected_model_index = 0;
        self.models_list_state.select(Some(0));
    }

    pub fn get_total_models_count(&self) -> usize {
        match self.models_view_mode {
            ModelsViewMode::Installed => self.models.len(),
            ModelsViewMode::Available => self.available_models.len(),
            ModelsViewMode::All => self.models.len() + self.available_models.len(),
        }
    }

    pub fn get_api_endpoints() -> Vec<ApiEndpoint> {
        vec![
            ApiEndpoint {
                name: "List Models".to_string(),
                method: "GET".to_string(),
                path: "/api/tags".to_string(),
                description: "List available models".to_string(),
                example_body: None,
            },
            ApiEndpoint {
                name: "Generate Completion".to_string(),
                method: "POST".to_string(),
                path: "/api/generate".to_string(),
                description: "Generate a response for a given prompt".to_string(),
                example_body: Some(r#"{
  "model": "llama3.2",
  "prompt": "Why is the sky blue?",
  "stream": false
}"#.to_string()),
            },
            ApiEndpoint {
                name: "Chat Completion".to_string(),
                method: "POST".to_string(),
                path: "/api/chat".to_string(),
                description: "Generate chat completion".to_string(),
                example_body: Some(r#"{
  "model": "llama3.2",
  "messages": [
    {"role": "user", "content": "Hello!"}
  ]
}"#.to_string()),
            },
            ApiEndpoint {
                name: "Pull Model".to_string(),
                method: "POST".to_string(),
                path: "/api/pull".to_string(),
                description: "Download a model".to_string(),
                example_body: Some(r#"{
  "name": "llama3.2"
}"#.to_string()),
            },
            ApiEndpoint {
                name: "Push Model".to_string(),
                method: "POST".to_string(),
                path: "/api/push".to_string(),
                description: "Push a model to registry".to_string(),
                example_body: Some(r#"{
  "name": "llama3.2"
}"#.to_string()),
            },
            ApiEndpoint {
                name: "Create Model".to_string(),
                method: "POST".to_string(),
                path: "/api/create".to_string(),
                description: "Create a model from Modelfile".to_string(),
                example_body: Some(r#"{
  "name": "my-model",
  "modelfile": "FROM llama3.2\nSYSTEM You are a helpful assistant."
}"#.to_string()),
            },
            ApiEndpoint {
                name: "Copy Model".to_string(),
                method: "POST".to_string(),
                path: "/api/copy".to_string(),
                description: "Copy a model".to_string(),
                example_body: Some(r#"{
  "source": "llama3.2",
  "destination": "my-llama"
}"#.to_string()),
            },
            ApiEndpoint {
                name: "Delete Model".to_string(),
                method: "DELETE".to_string(),
                path: "/api/delete".to_string(),
                description: "Delete a model".to_string(),
                example_body: Some(r#"{
  "name": "llama3.2"
}"#.to_string()),
            },
            ApiEndpoint {
                name: "Show Model Info".to_string(),
                method: "POST".to_string(),
                path: "/api/show".to_string(),
                description: "Show model information".to_string(),
                example_body: Some(r#"{
  "name": "llama3.2"
}"#.to_string()),
            },
            ApiEndpoint {
                name: "List Running Models".to_string(),
                method: "GET".to_string(),
                path: "/api/ps".to_string(),
                description: "List running models".to_string(),
                example_body: None,
            },
            ApiEndpoint {
                name: "Generate Embeddings".to_string(),
                method: "POST".to_string(),
                path: "/api/embed".to_string(),
                description: "Generate embeddings from a model".to_string(),
                example_body: Some(r#"{
  "model": "llama3.2",
  "input": "Here is some text to embed"
}"#.to_string()),
            },
        ]
    }

    pub async fn discover_network_urls(&mut self) {
        self.api_explorer_state.network_urls.clear();
        
        // Add localhost
        self.api_explorer_state.network_urls.push("http://localhost:11434".to_string());
        
        // Try to get local IP addresses
        if let Ok(output) = tokio::process::Command::new("ifconfig")
            .output()
            .await
        {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                if line.contains("inet ") && !line.contains("127.0.0.1") {
                    if let Some(ip) = line.split_whitespace().nth(1) {
                        self.api_explorer_state.network_urls.push(format!("http://{}:11434", ip));
                    }
                }
            }
        }
    }

    pub async fn discover_local_services(&mut self) {
        self.discovered_services.clear();
        self.add_log(LogLevel::Info, "Scanning local network for Ollama services...");
        
        // Scan common ports on local subnet
        let base_ip = "192.168.1"; // Common local subnet
        let ports = vec![11434, 11435, 8080, 3000];
        
        for i in 1..255 {
            for &port in &ports {
                let ip = format!("{}.{}", base_ip, i);
                let url = format!("http://{}:{}/api/tags", ip, port);
                
                // Try to connect with a short timeout
                match self.ollama_client.client.get(&url)
                    .timeout(std::time::Duration::from_millis(100))
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        self.discovered_services.push(DiscoveredService {
                            name: format!("Ollama@{}", ip),
                            ip: ip.clone(),
                            port,
                            is_reachable: true,
                            version: None,
                        });
                        self.add_log(LogLevel::Info, &format!("Found Ollama service at {}:{}", ip, port));
                    }
                    _ => {}
                }
            }
        }
        
        if self.discovered_services.is_empty() {
            self.add_log(LogLevel::Info, "No additional Ollama services found on local network");
        }
    }

    pub async fn execute_selected_endpoint(&mut self) {
        let endpoints = Self::get_api_endpoints();
        if let Some(endpoint) = endpoints.get(self.api_explorer_state.selected_endpoint) {
            self.add_log(LogLevel::Info, &format!("Executing {} {}", endpoint.method, endpoint.path));
            self.api_explorer_state.response_body = String::from("Loading...");
            
            let url = format!("{}{}", self.ollama_client.base_url, endpoint.path);
            
            let result = match endpoint.method.as_str() {
                "GET" => {
                    self.ollama_client.client
                        .get(&url)
                        .send()
                        .await
                }
                "POST" => {
                    let body = endpoint.example_body.clone().unwrap_or_else(|| "{}".to_string());
                    self.ollama_client.client
                        .post(&url)
                        .header("Content-Type", "application/json")
                        .body(body)
                        .send()
                        .await
                }
                "DELETE" => {
                    let body = endpoint.example_body.clone().unwrap_or_else(|| "{}".to_string());
                    self.ollama_client.client
                        .delete(&url)
                        .header("Content-Type", "application/json")
                        .body(body)
                        .send()
                        .await
                }
                _ => {
                    self.api_explorer_state.response_body = format!("Unsupported method: {}", endpoint.method);
                    return;
                }
            };
            
            match result {
                Ok(response) => {
                    let status = response.status();
                    let headers = response.headers().clone();
                    
                    match response.text().await {
                        Ok(body) => {
                            // Try to format as JSON if possible
                            let formatted_body = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                                serde_json::to_string_pretty(&json).unwrap_or(body)
                            } else {
                                body
                            };
                            
                            self.api_explorer_state.response_body = format!(
                                "Status: {}\n\nHeaders:\n{}\n\nBody:\n{}",
                                status,
                                headers.iter()
                                    .map(|(k, v)| format!("  {}: {:?}", k, v))
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                formatted_body
                            );
                            
                            self.add_log(LogLevel::Info, &format!("API call successful: {}", status));
                        }
                        Err(e) => {
                            self.api_explorer_state.response_body = format!("Failed to read response: {}", e);
                            self.add_log(LogLevel::Error, &format!("Failed to read response: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.api_explorer_state.response_body = format!("Request failed: {}", e);
                    self.add_log(LogLevel::Error, &format!("API call failed: {}", e));
                }
            }
        }
    }

    pub async fn install_selected_available_model(&mut self) {
        if self.models_view_mode == ModelsViewMode::Available
            || self.models_view_mode == ModelsViewMode::All
        {
            let installed_count = if self.models_view_mode == ModelsViewMode::All {
                self.models.len()
            } else {
                0
            };

            if self.selected_model_index >= installed_count {
                let available_index = self.selected_model_index - installed_count;
                if let Some(model) = self.available_models.get(available_index) {
                    let model_name = model.id.clone();
                    self.add_log(LogLevel::Info, &format!("Installing model: {model_name}"));

                    match self.ollama_client.pull_model(&model_name).await {
                        Ok(_) => {
                            self.add_log(
                                LogLevel::Info,
                                &format!("Successfully installed: {model_name}"),
                            );
                            self.refresh().await;
                        }
                        Err(e) => {
                            self.add_log(
                                LogLevel::Error,
                                &format!("Failed to install {model_name}: {e}"),
                            );
                        }
                    }
                }
            }
        }
    }

    // Chat interface methods
    pub async fn send_chat_message(&mut self) {
        if self.chat_state.current_session().input_buffer.is_empty() {
            return;
        }

        // Check if we have models available
        if self.models.is_empty() {
            self.add_log(LogLevel::Error, "No models available. Please pull a model first!");
            self.chat_state.current_session().add_message(
                MessageRole::System, 
                "❌ No models available. Please go to Models tab and pull a model first!".to_string()
            );
            return;
        }

        let message = self.chat_state.current_session().input_buffer.clone();
        self.chat_state.current_session().input_buffer.clear();
        
        // Add user message
        self.chat_state.current_session().add_message(MessageRole::User, message.clone());
        
        // Start streaming response
        self.chat_state.current_session().is_streaming = true;
        self.chat_state.current_session().current_response.clear();
        
        let model = self.chat_state.current_session().current_model.clone();
        let model_name = model.clone();
        let messages = self.chat_state.current_session().get_context_for_api();
        
        let (tx, rx) = mpsc::channel(100);
        
        let ollama_client = self.ollama_client.clone();
        let error_tx = tx.clone();
        
        // Store receiver
        self.chat_response_receiver = Some(rx);
        
        tokio::spawn(async move {
            if let Err(e) = ollama_client.chat(&model, messages, tx).await {
                eprintln!("Chat error: {}", e);
                // Send error as a special response
                let error_response = ChatResponse {
                    model: Some(model.clone()),
                    created_at: None,
                    message: Some(crate::api::ChatMessage {
                        role: "system".to_string(),
                        content: format!("❌ Error: {}. Make sure the model '{}' is installed.", e, model),
                    }),
                    done: Some(true),
                    total_duration: None,
                    load_duration: None,
                    prompt_eval_count: None,
                    prompt_eval_duration: None,
                    eval_count: None,
                    eval_duration: None,
                };
                let _ = error_tx.send(error_response).await;
            }
        });
        
        self.add_log(LogLevel::Info, &format!("Sending message to {}", model_name));
    }

    pub async fn process_chat_response(&mut self) {
        if let Some(receiver) = &mut self.chat_response_receiver {
            // Process all available messages for smooth streaming
            let mut messages_processed = 0;
            const MAX_MESSAGES_PER_BATCH: usize = 10; // Process up to 10 messages per call
            
            while messages_processed < MAX_MESSAGES_PER_BATCH {
                match receiver.try_recv() {
                    Ok(response) => {
                        messages_processed += 1;
                        
                        if let Some(message) = response.message {
                            match message.role.as_str() {
                                "assistant" => {
                                    self.chat_state.current_session().current_response.push_str(&message.content);
                                }
                                "system" => {
                                    // This is an error message
                                    self.chat_state.current_session().add_message(MessageRole::System, message.content);
                                    self.chat_state.current_session().is_streaming = false;
                                    self.chat_response_receiver = None;
                                    return;
                                }
                                _ => {}
                            }
                        }
                        
                        if response.done.unwrap_or(false) {
                            // Finalize the response
                            let final_response = self.chat_state.current_session().current_response.clone();
                            if !final_response.is_empty() {
                                self.chat_state.current_session().add_message(MessageRole::Assistant, final_response);
                            }
                            self.chat_state.current_session().is_streaming = false;
                            self.chat_state.current_session().current_response.clear();
                            
                            // Update token count
                            if let Some(eval_count) = response.eval_count {
                                self.chat_state.current_session().total_tokens += eval_count as usize;
                            }
                            
                            self.chat_response_receiver = None;
                            self.add_log(LogLevel::Info, "Response completed");
                            return;
                        }
                    }
                    Err(mpsc::error::TryRecvError::Empty) => {
                        // No more messages available
                        break;
                    }
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        // Channel disconnected
                        self.chat_state.current_session().is_streaming = false;
                        self.chat_response_receiver = None;
                        self.add_log(LogLevel::Error, "Chat stream disconnected");
                        break;
                    }
                }
            }
        }
    }

    pub fn handle_chat_input(&mut self, c: char) {
        if self.chat_state.input_mode == InputMode::Editing {
            self.chat_state.current_session().input_buffer.push(c);
        }
    }

    pub fn handle_chat_backspace(&mut self) {
        if self.chat_state.input_mode == InputMode::Editing {
            self.chat_state.current_session().input_buffer.pop();
        }
    }

    pub fn toggle_chat_input_mode(&mut self) {
        self.chat_state.input_mode = match self.chat_state.input_mode {
            InputMode::Normal => InputMode::Editing,
            InputMode::Editing => InputMode::Normal,
            InputMode::ModelSelection => InputMode::Normal,
        };
    }

    pub fn initialize_chat(&mut self) {
        // Update available models for chat
        let model_names: Vec<String> = self.models.iter()
            .map(|m| m.name.clone())
            .collect();
        
        if !model_names.is_empty() {
            self.chat_state = ChatState::new(model_names.clone());
            self.add_log(LogLevel::Info, &format!("Chat initialized with {} models", model_names.len()));
            
            // Set the first available model as current
            if let Some(first_model) = model_names.first() {
                self.chat_state.current_session().current_model = first_model.clone();
                self.add_log(LogLevel::Info, &format!("Using model: {}", first_model));
            }
        } else {
            self.add_log(LogLevel::Warning, "No models available for chat. Pull a model first!");
        }
    }
}
