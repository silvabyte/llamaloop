use crate::api::{OllamaClient, Model, RunningModel};
use crate::theme::Sparkle;
use chrono::{DateTime, Local};
use std::collections::VecDeque;

pub struct App {
    pub selected_tab: usize,
    pub current_screen: CurrentScreen,
    pub should_quit: bool,
    pub models: Vec<Model>,
    pub running_models: Vec<RunningModel>,
    pub logs: VecDeque<LogEntry>,
    pub selected_model_index: usize,
    pub show_pull_dialog: bool,
    pub pull_model_name: String,
    pub pull_progress: Option<PullProgress>,
    pub ollama_client: OllamaClient,
    pub status: SystemStatus,
    pub last_refresh: DateTime<Local>,
}

#[derive(Clone, PartialEq)]
pub enum CurrentScreen {
    Dashboard,
    Models,
    Logs,
    Chat,
    Help,
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
    Debug,
}

#[derive(Clone)]
pub struct PullProgress {
    pub model: String,
    pub status: String,
    pub percentage: f64,
    pub downloaded: u64,
    pub total: u64,
}

#[derive(Clone)]
pub struct SystemStatus {
    pub is_running: bool,
    pub version: String,
    pub models_loaded: usize,
    pub total_memory: u64,
    pub used_memory: u64,
}

impl App {
    pub fn new() -> Self {
        let mut logs = VecDeque::new();
        logs.push_back(LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Info,
            message: "Ollamamon started".to_string(),
        });

        Self {
            selected_tab: 0,
            current_screen: CurrentScreen::Dashboard,
            should_quit: false,
            models: Vec::new(),
            running_models: Vec::new(),
            logs,
            selected_model_index: 0,
            show_pull_dialog: false,
            pull_model_name: String::new(),
            pull_progress: None,
            ollama_client: OllamaClient::new("http://localhost:11434".to_string()),
            status: SystemStatus {
                is_running: false,
                version: "Unknown".to_string(),
                models_loaded: 0,
                total_memory: 0,
                used_memory: 0,
            },
            last_refresh: Local::now(),
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
    }

    pub fn on_up(&mut self) {
        if self.current_screen == CurrentScreen::Models && !self.models.is_empty() {
            if self.selected_model_index > 0 {
                self.selected_model_index -= 1;
            }
        }
    }

    pub fn on_down(&mut self) {
        if self.current_screen == CurrentScreen::Models && !self.models.is_empty() {
            if self.selected_model_index < self.models.len() - 1 {
                self.selected_model_index += 1;
            }
        }
    }

    pub async fn on_enter(&mut self) {
        if self.show_pull_dialog && !self.pull_model_name.is_empty() {
            self.start_pull_model().await;
        }
    }

    pub async fn on_tick(&mut self) {
        if Local::now().signed_duration_since(self.last_refresh).num_seconds() > 5 {
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
                        
                        self.status.used_memory = self.running_models
                            .iter()
                            .map(|m| m.size)
                            .sum();
                    }
                } else {
                    self.add_log(LogLevel::Warning, "Ollama is not responding");
                }
            }
            Err(e) => {
                self.add_log(LogLevel::Error, &format!("Failed to check status: {}", e));
                self.status.is_running = false;
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
        
        self.add_log(LogLevel::Info, &format!("Pulling model: {}", model_name));
        
        match self.ollama_client.pull_model(&model_name).await {
            Ok(_) => {
                self.add_log(LogLevel::Info, &format!("Successfully pulled model: {}", model_name));
                self.refresh().await;
            }
            Err(e) => {
                self.add_log(LogLevel::Error, &format!("Failed to pull model: {}", e));
            }
        }
    }

    pub async fn delete_selected_model(&mut self) {
        if let Some(model) = self.models.get(self.selected_model_index) {
            let model_name = model.name.clone();
            
            match self.ollama_client.delete_model(&model_name).await {
                Ok(_) => {
                    self.add_log(LogLevel::Info, &format!("Deleted model: {}", model_name));
                    self.refresh().await;
                }
                Err(e) => {
                    self.add_log(LogLevel::Error, &format!("Failed to delete model: {}", e));
                }
            }
        }
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
}