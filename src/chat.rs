use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Local>,
    pub model: Option<String>,
    pub tokens: Option<usize>,
    pub response_time: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub messages: VecDeque<ChatMessage>,
    pub current_model: String,
    pub is_streaming: bool,
    pub current_response: String,
    pub input_buffer: String,
    pub total_tokens: usize,
    pub session_started: DateTime<Local>,
}

impl ChatSession {
    pub fn new(model: String) -> Self {
        Self {
            messages: VecDeque::new(),
            current_model: model,
            is_streaming: false,
            current_response: String::new(),
            input_buffer: String::new(),
            total_tokens: 0,
            session_started: Local::now(),
        }
    }

    pub fn add_message(&mut self, role: MessageRole, content: String) {
        let message = ChatMessage {
            role,
            content,
            timestamp: Local::now(),
            model: if role == MessageRole::Assistant {
                Some(self.current_model.clone())
            } else {
                None
            },
            tokens: None,
            response_time: None,
        };
        self.messages.push_back(message);
        
        // Keep only last 100 messages to prevent memory issues
        while self.messages.len() > 100 {
            self.messages.pop_front();
        }
    }

    pub fn clear_session(&mut self) {
        self.messages.clear();
        self.current_response.clear();
        self.input_buffer.clear();
        self.total_tokens = 0;
        self.session_started = Local::now();
    }

    pub fn change_model(&mut self, new_model: String) {
        self.current_model = new_model;
        self.add_message(
            MessageRole::System,
            format!("Switched to model: {}", self.current_model),
        );
    }

    pub fn export_session(&self) -> String {
        let mut export = String::new();
        export.push_str(&format!("# Chat Session - {}\n", self.session_started.format("%Y-%m-%d %H:%M:%S")));
        export.push_str(&format!("Model: {}\n\n", self.current_model));
        
        for msg in &self.messages {
            let role = match msg.role {
                MessageRole::System => "System",
                MessageRole::User => "User",
                MessageRole::Assistant => "Assistant",
            };
            export.push_str(&format!("## {} [{}]\n", role, msg.timestamp.format("%H:%M:%S")));
            export.push_str(&format!("{}\n\n", msg.content));
        }
        
        export
    }

    pub fn get_context_for_api(&self) -> Vec<serde_json::Value> {
        self.messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|msg| {
                serde_json::json!({
                    "role": match msg.role {
                        MessageRole::User => "user",
                        MessageRole::Assistant => "assistant",
                        MessageRole::System => "system",
                    },
                    "content": msg.content
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ChatState {
    pub sessions: Vec<ChatSession>,
    pub active_session_index: usize,
    pub scroll_offset: usize,
    pub input_mode: InputMode,
    pub show_model_selector: bool,
    pub available_models: Vec<String>,
    pub selected_model_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
    ModelSelection,
}

impl ChatState {
    pub fn new(available_models: Vec<String>) -> Self {
        let default_model = available_models.first()
            .cloned()
            .unwrap_or_else(|| "llama3.1:latest".to_string());
        
        let mut sessions = Vec::new();
        sessions.push(ChatSession::new(default_model));
        
        Self {
            sessions,
            active_session_index: 0,
            scroll_offset: 0,
            input_mode: InputMode::Normal,
            show_model_selector: false,
            available_models,
            selected_model_index: 0,
        }
    }

    pub fn current_session(&mut self) -> &mut ChatSession {
        &mut self.sessions[self.active_session_index]
    }

    pub fn new_session(&mut self) {
        let model = self.current_session().current_model.clone();
        self.sessions.push(ChatSession::new(model));
        self.active_session_index = self.sessions.len() - 1;
    }

    pub fn delete_session(&mut self) {
        if self.sessions.len() > 1 {
            self.sessions.remove(self.active_session_index);
            if self.active_session_index >= self.sessions.len() {
                self.active_session_index = self.sessions.len() - 1;
            }
        }
    }

    pub fn next_session(&mut self) {
        if self.active_session_index < self.sessions.len() - 1 {
            self.active_session_index += 1;
        }
    }

    pub fn previous_session(&mut self) {
        if self.active_session_index > 0 {
            self.active_session_index -= 1;
        }
    }

    pub fn toggle_model_selector(&mut self) {
        self.show_model_selector = !self.show_model_selector;
        if self.show_model_selector {
            self.input_mode = InputMode::ModelSelection;
        } else {
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn select_model(&mut self) {
        if self.selected_model_index < self.available_models.len() {
            let new_model = self.available_models[self.selected_model_index].clone();
            self.current_session().change_model(new_model);
        }
        self.show_model_selector = false;
        self.input_mode = InputMode::Normal;
    }
}