// MyDR24 Healthcare Platform - Simplified Chat Component
// Real-time messaging for patient-provider communication

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use web_sys::{console, window};
use crate::websocket_simple::{SimpleWebSocketClient, ChatMessage, MessageType, create_chat_message};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub chat_id: String,
    pub patient_id: String,
    pub provider_id: String,
    pub messages: Vec<ChatMessage>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

pub struct SimpleChatManager {
    pub current_user_id: String,
    pub current_user_role: String, // "patient" or "provider"
    pub active_sessions: HashMap<String, ChatSession>,
    pub websocket_client: Option<SimpleWebSocketClient>,
}

impl SimpleChatManager {
    pub fn new(user_id: String, user_role: String) -> Self {
        Self {
            current_user_id: user_id,
            current_user_role: user_role,
            active_sessions: HashMap::new(),
            websocket_client: None,
        }
    }
    
    pub fn set_websocket_client(&mut self, client: SimpleWebSocketClient) {
        // Register for chat message callbacks
        client.on_message(MessageType::ChatMessage, {
            let user_id = self.current_user_id.clone();
            move |message| {
                console::log_1(&format!("Received chat message: {:?}", message).into());
                // Handle incoming chat message
                if let Ok(chat_msg) = serde_json::from_value::<ChatMessage>(message.payload) {
                    // Update UI or store message
                    console::log_1(&format!("Chat message from {}: {}", 
                        chat_msg.sender_id, chat_msg.content).into());
                }
            }
        });
        
        self.websocket_client = Some(client);
    }
    
    pub fn create_chat_session(&mut self, other_user_id: String) -> String {
        let chat_id = Uuid::new_v4().to_string();
        
        let (patient_id, provider_id) = if self.current_user_role == "patient" {
            (self.current_user_id.clone(), other_user_id)
        } else {
            (other_user_id, self.current_user_id.clone())
        };
        
        let session = ChatSession {
            chat_id: chat_id.clone(),
            patient_id,
            provider_id,
            messages: Vec::new(),
            is_active: true,
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        
        self.active_sessions.insert(chat_id.clone(), session);
        chat_id
    }
    
    pub async fn send_message(&mut self, chat_id: String, content: String) -> Result<(), String> {
        let chat_message = create_chat_message(
            chat_id.clone(),
            self.current_user_id.clone(),
            "provider-001".to_string(), // TODO: Get actual receiver_id
            content.clone(),
            "text".to_string(),
        );
        
        // Add to local session
        if let Some(session) = self.active_sessions.get_mut(&chat_id) {
            session.messages.push(chat_message.clone());
            session.last_activity = Utc::now();
        }
        
        // Send via WebSocket
        if let Some(client) = &self.websocket_client {
            client.send_chat_message(chat_message).await?;
        }
        
        Ok(())
    }
    
    pub fn get_chat_messages(&self, chat_id: &str) -> Vec<ChatMessage> {
        self.active_sessions
            .get(chat_id)
            .map(|session| session.messages.clone())
            .unwrap_or_default()
    }
    
    pub fn get_active_chats(&self) -> Vec<&ChatSession> {
        self.active_sessions
            .values()
            .filter(|session| session.is_active)
            .collect()
    }
}

// Quick response templates for providers
pub fn get_provider_quick_responses() -> Vec<(&'static str, &'static str)> {
    vec![
        ("I'm on my way", "I'm currently on my way to your location."),
        ("Running late", "I'm running a few minutes late, will be there soon."),
        ("Arrived", "I have arrived at your location."),
        ("Completed", "The consultation has been completed."),
        ("Follow up", "Please follow the prescribed treatment and follow up if needed."),
        ("Emergency", "This appears to be an emergency. Please call 108 immediately."),
    ]
}

// Message encryption helpers (simplified)
pub fn encrypt_message(content: &str, _key: &str) -> String {
    // Simplified encryption - in production use proper encryption
    general_purpose::STANDARD.encode(content)
}

pub fn decrypt_message(encrypted: &str, _key: &str) -> Result<String, String> {
    // Simplified decryption
    general_purpose::STANDARD.decode(encrypted)
        .map_err(|e| format!("Decryption error: {}", e))
        .and_then(|bytes| String::from_utf8(bytes)
            .map_err(|e| format!("UTF-8 error: {}", e)))
}

// HIPAA compliance helpers
pub fn sanitize_message_for_storage(message: &ChatMessage) -> ChatMessage {
    let mut sanitized = message.clone();
    
    // Remove or encrypt sensitive data
    if sanitized.content.contains("SSN") || 
       sanitized.content.contains("social security") ||
       sanitized.content.contains("insurance") {
        sanitized.content = "[SENSITIVE DATA REDACTED]".to_string();
    }
    
    sanitized
}

pub fn validate_message_content(content: &str) -> Result<(), String> {
    if content.trim().is_empty() {
        return Err("Message cannot be empty".to_string());
    }
    
    if content.len() > 1000 {
        return Err("Message too long (max 1000 characters)".to_string());
    }
    
    // Check for inappropriate content
    let blocked_words = ["spam", "advertisement", "promotion"];
    for word in blocked_words {
        if content.to_lowercase().contains(word) {
            return Err("Message contains inappropriate content".to_string());
        }
    }
    
    Ok(())
}

// Message status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageWithStatus {
    pub message: ChatMessage,
    pub status: MessageStatus,
    pub delivery_timestamp: Option<DateTime<Utc>>,
    pub read_timestamp: Option<DateTime<Utc>>,
}

// Typing indicator
pub struct TypingIndicator {
    pub user_id: String,
    pub chat_id: String,
    pub is_typing: bool,
    pub last_updated: DateTime<Utc>,
}

impl TypingIndicator {
    pub fn new(user_id: String, chat_id: String) -> Self {
        Self {
            user_id,
            chat_id,
            is_typing: false,
            last_updated: Utc::now(),
        }
    }
    
    pub fn start_typing(&mut self) {
        self.is_typing = true;
        self.last_updated = Utc::now();
    }
    
    pub fn stop_typing(&mut self) {
        self.is_typing = false;
        self.last_updated = Utc::now();
    }
    
    pub fn is_stale(&self) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.last_updated);
        duration.num_seconds() > 5 // Consider stale after 5 seconds
    }
}
