// MyDR24 Healthcare Platform - Simplified WebSocket Client
// Real-time communication client for Patient and Provider apps

use serde::{Serialize, Deserialize};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use futures::{StreamExt, SinkExt};
use gloo_net::websocket::{futures::WebSocket, Message, WebSocketError};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

// WebSocket message types matching backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageType {
    #[serde(rename = "provider_location")]
    ProviderLocationUpdate,
    #[serde(rename = "booking_status")]
    BookingStatusUpdate,
    #[serde(rename = "emergency_alert")]
    EmergencyAlert,
    #[serde(rename = "chat_message")]
    ChatMessage,
    #[serde(rename = "payment_notification")]
    PaymentNotification,
    #[serde(rename = "connection_ack")]
    ConnectionAck,
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "error")]
    Error,
}

// Connection states
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error(String),
}

// Main WebSocket message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub id: String,
    pub message_type: MessageType,
    pub payload: JsonValue,
    pub timestamp: DateTime<Utc>,
    pub sender_id: String,
    pub recipient_id: Option<String>,
}

// Configuration for WebSocket connection
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub url: String,
    pub auth_token: Option<String>,
    pub user_id: String,
    pub user_role: String,
    pub auto_reconnect: bool,
    pub max_reconnect_attempts: u32,
    pub heartbeat_interval: u64,
    pub connection_timeout: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            url: "ws://127.0.0.1:8000/api/v1/websocket/connect".to_string(),
            auth_token: None,
            user_id: String::new(),
            user_role: "patient".to_string(),
            auto_reconnect: true,
            max_reconnect_attempts: 5,
            heartbeat_interval: 30,
            connection_timeout: 10,
        }
    }
}

// Real-time location data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationUpdate {
    pub provider_id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}

// Booking status updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingStatusUpdate {
    pub booking_id: String,
    pub status: String,
    pub message: Option<String>,
    pub estimated_time: Option<String>,
    pub timestamp: DateTime<Utc>,
}

// Emergency alert structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyAlert {
    pub alert_id: String,
    pub patient_id: String,
    pub alert_type: String,
    pub severity: String,
    pub location: Location,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub medical_condition: Option<String>,
    pub emergency_contact: Option<String>,
    pub priority: String,
}

// Location structure for emergency alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
    pub timestamp: DateTime<Utc>,
}

// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: String,
    pub chat_id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub content: String,
    pub message_type: String,
    pub timestamp: DateTime<Utc>,
    pub is_read: bool,
    pub is_encrypted: bool,
}

// Payment notification structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentNotification {
    pub payment_id: String,
    pub booking_id: String,
    pub amount: f64,
    pub status: String,
    pub gateway_response: Option<String>,
    pub timestamp: DateTime<Utc>,
}

// Message callback type
pub type MessageCallback = Arc<dyn Fn(WebSocketMessage) + Send + Sync + 'static>;

// Simplified WebSocket client without reactive signals
#[derive(Clone)]
pub struct SimpleWebSocketClient {
    config: WebSocketConfig,
    state: Arc<Mutex<ConnectionState>>,
    callbacks: Arc<Mutex<HashMap<MessageType, Vec<MessageCallback>>>>,
    reconnect_attempts: Arc<Mutex<u32>>,
}

impl SimpleWebSocketClient {
    pub fn new(config: WebSocketConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            reconnect_attempts: Arc::new(Mutex::new(0)),
        }
    }
    
    // Get current connection state
    pub fn get_state(&self) -> ConnectionState {
        self.state.lock().unwrap().clone()
    }
    
    // Register callback for specific message type
    pub fn on_message<F>(&self, message_type: MessageType, callback: F)
    where
        F: Fn(WebSocketMessage) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.entry(message_type).or_insert_with(Vec::new).push(Arc::new(callback));
    }
    
    // Connect to WebSocket server
    pub async fn connect(&self) -> Result<(), String> {
        console::log_1(&"Connecting to WebSocket...".into());
        
        // Update state to connecting
        {
            let mut state = self.state.lock().unwrap();
            *state = ConnectionState::Connecting;
        }
        
        // Build connection URL with authentication
        let mut url = self.config.url.clone();
        if let Some(token) = &self.config.auth_token {
            url = format!("{}?token={}&user_id={}&role={}", 
                url, token, self.config.user_id, self.config.user_role);
        }
        
        // Establish WebSocket connection
        let ws = WebSocket::open(&url).map_err(|e| format!("WebSocket open error: {:?}", e))?;
        
        // Update state to connected
        {
            let mut state = self.state.lock().unwrap();
            *state = ConnectionState::Connected;
        }
        
        // Reset reconnect attempts on successful connection
        {
            let mut attempts = self.reconnect_attempts.lock().unwrap();
            *attempts = 0;
        }
        
        // Handle incoming messages
        let (mut write, mut read) = ws.split();
        let callbacks = Arc::clone(&self.callbacks);
        let state = Arc::clone(&self.state);
        
        // Send connection acknowledgment
        let connect_msg = json!({
            "id": Uuid::new_v4().to_string(),
            "message_type": "connection_ack",
            "payload": {
                "user_id": self.config.user_id,
                "role": self.config.user_role
            },
            "timestamp": Utc::now(),
            "sender_id": self.config.user_id
        });
        
        if let Err(e) = write.send(Message::Text(connect_msg.to_string())).await {
            console::log_1(&format!("Failed to send connect message: {:?}", e).into());
        }
        
        // Message handling loop
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(ws_message) = serde_json::from_str::<WebSocketMessage>(&text) {
                            console::log_1(&format!("Received message: {:?}", ws_message.message_type).into());
                            
                            // Call registered callbacks
                            if let Ok(callbacks) = callbacks.lock() {
                                if let Some(handlers) = callbacks.get(&ws_message.message_type) {
                                    for handler in handlers {
                                        handler(ws_message.clone());
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Bytes(_)) => {
                        console::log_1(&"Received binary message".into());
                    }
                    Err(e) => {
                        console::log_1(&format!("WebSocket error: {:?}", e).into());
                        let mut state = state.lock().unwrap();
                        *state = ConnectionState::Error(format!("{:?}", e));
                        break;
                    }
                }
            }
        });
        
        Ok(())
    }
    
    // Send message to server
    pub async fn send_message(&self, message: WebSocketMessage) -> Result<(), String> {
        // For now, we'll use a simple approach and reconnect each time
        // In a production app, you'd maintain the connection
        let mut url = self.config.url.clone();
        if let Some(token) = &self.config.auth_token {
            url = format!("{}?token={}&user_id={}&role={}", 
                url, token, self.config.user_id, self.config.user_role);
        }
        
        match WebSocket::open(&url) {
            Ok(ws) => {
                let (mut write, _) = ws.split();
                let msg_json = serde_json::to_string(&message)
                    .map_err(|e| format!("Serialization error: {}", e))?;
                
                write.send(Message::Text(msg_json)).await
                    .map_err(|e| format!("Send error: {:?}", e))?;
                
                Ok(())
            }
            Err(e) => Err(format!("Connection error: {:?}", e))
        }
    }
    
    // Send emergency alert
    pub async fn send_emergency_alert(&self, alert: EmergencyAlert) -> Result<(), String> {
        let message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::EmergencyAlert,
            payload: serde_json::to_value(alert)
                .map_err(|e| format!("Serialization error: {}", e))?,
            timestamp: Utc::now(),
            sender_id: self.config.user_id.clone(),
            recipient_id: None,
        };
        
        self.send_message(message).await
    }
    
    // Send location update
    pub async fn send_location_update(&self, location: LocationUpdate) -> Result<(), String> {
        let message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::ProviderLocationUpdate,
            payload: serde_json::to_value(location)
                .map_err(|e| format!("Serialization error: {}", e))?,
            timestamp: Utc::now(),
            sender_id: self.config.user_id.clone(),
            recipient_id: None,
        };
        
        self.send_message(message).await
    }
    
    // Send chat message
    pub async fn send_chat_message(&self, chat_msg: ChatMessage) -> Result<(), String> {
        let message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::ChatMessage,
            payload: serde_json::to_value(chat_msg)
                .map_err(|e| format!("Serialization error: {}", e))?,
            timestamp: Utc::now(),
            sender_id: self.config.user_id.clone(),
            recipient_id: None,
        };
        
        self.send_message(message).await
    }
    
    // Disconnect from server
    pub fn disconnect(&self) {
        let mut state = self.state.lock().unwrap();
        *state = ConnectionState::Disconnected;
        console::log_1(&"WebSocket disconnected".into());
    }
}

// Helper functions for creating messages
pub fn create_emergency_alert(
    patient_id: String,
    location: Option<(f64, f64)>,
    medical_condition: Option<String>,
    emergency_contact: Option<String>,
) -> EmergencyAlert {
    let (latitude, longitude) = location.unwrap_or((0.0, 0.0));
    EmergencyAlert {
        alert_id: Uuid::new_v4().to_string(),
        patient_id,
        alert_type: "emergency".to_string(),
        severity: "high".to_string(),
        location: Location {
            latitude,
            longitude,
            address: None,
            timestamp: Utc::now(),
        },
        description: "Emergency alert".to_string(),
        timestamp: Utc::now(),
        status: "active".to_string(),
        medical_condition,
        emergency_contact,
        priority: "high".to_string(),
    }
}

pub fn create_location_update(
    provider_id: String,
    latitude: f64,
    longitude: f64,
    accuracy: f64,
    status: String,
) -> LocationUpdate {
    LocationUpdate {
        provider_id,
        latitude,
        longitude,
        accuracy,
        timestamp: Utc::now(),
        status,
    }
}

pub fn create_chat_message(
    chat_id: String,
    sender_id: String,
    receiver_id: String,
    content: String,
    message_type: String,
) -> ChatMessage {
    ChatMessage {
        message_id: Uuid::new_v4().to_string(),
        chat_id,
        sender_id,
        receiver_id,
        content,
        message_type,
        timestamp: Utc::now(),
        is_read: false,
        is_encrypted: false,
    }
}
