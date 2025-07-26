// MyDR24 Healthcare Platform - Shared WebSocket Client
// Real-time communication client for Patient and Provider apps
// Provides: Auto-reconnect, message queuing, authentication, error handling

use serde::{Serialize, Deserialize};
use serde_json::{json, Value as JsonValue};
use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use futures::{StreamExt, SinkExt};
use gloo_net::websocket::{futures::WebSocket, Message, WebSocketError};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

// WebSocket message types matching backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePriority {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub id: String,
    pub message_type: MessageType,
    pub sender_id: String,
    pub recipient_id: Option<String>,
    pub channel: String,
    pub payload: JsonValue,
    pub timestamp: DateTime<Utc>,
    pub priority: MessagePriority,
}

// Connection states
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

// WebSocket client configuration
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub url: String,
    pub auth_token: Option<String>,
    pub user_id: String,
    pub user_role: String, // "patient" or "provider"
    pub auto_reconnect: bool,
    pub max_reconnect_attempts: u32,
    pub heartbeat_interval: u64, // seconds
    pub connection_timeout: u64, // seconds
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
    pub heading: Option<f64>,
    pub speed: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub status: String, // "en_route", "arrived", "offline"
}

// Booking status updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingStatusUpdate {
    pub booking_id: String,
    pub status: String,
    pub provider_id: Option<String>,
    pub patient_id: String,
    pub message: String,
    pub estimated_arrival: Option<DateTime<Utc>>,
    pub location: Option<(f64, f64)>,
}

// Emergency alert structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyAlert {
    pub alert_id: String,
    pub patient_id: String,
    pub location: (f64, f64),
    pub alert_type: String, // "medical", "safety", "urgent"
    pub severity: u8, // 1-10
    pub description: String,
    pub medical_conditions: Vec<String>,
    pub emergency_contacts: Vec<String>,
}

// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: String,
    pub booking_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub content: String,
    pub message_type: String, // "text", "image", "location", "voice"
    pub metadata: Option<JsonValue>,
}

// Payment notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentNotification {
    pub payment_id: String,
    pub booking_id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String, // "success", "failed", "pending"
    pub payment_method: String,
    pub receipt_url: Option<String>,
}

// WebSocket client state
pub struct WebSocketClient {
    config: WebSocketConfig,
    state: Arc<Mutex<ConnectionState>>,
    message_queue: Arc<Mutex<VecDeque<WebSocketMessage>>>,
    reconnect_attempts: Arc<Mutex<u32>>,
    
    // Leptos signals for reactive UI updates
    pub connection_state: ReadSignal<ConnectionState>,
    set_connection_state: WriteSignal<ConnectionState>,
    
    pub incoming_messages: ReadSignal<Vec<WebSocketMessage>>,
    set_incoming_messages: WriteSignal<Vec<WebSocketMessage>>,
    
    pub location_updates: ReadSignal<Vec<LocationUpdate>>,
    set_location_updates: WriteSignal<Vec<LocationUpdate>>,
    
    pub booking_updates: ReadSignal<Vec<BookingStatusUpdate>>,
    set_booking_updates: WriteSignal<Vec<BookingStatusUpdate>>,
    
    pub emergency_alerts: ReadSignal<Vec<EmergencyAlert>>,
    set_emergency_alerts: WriteSignal<Vec<EmergencyAlert>>,
    
    pub chat_messages: ReadSignal<Vec<ChatMessage>>,
    set_chat_messages: WriteSignal<Vec<ChatMessage>>,
    
    pub payment_notifications: ReadSignal<Vec<PaymentNotification>>,
    set_payment_notifications: WriteSignal<Vec<PaymentNotification>>,
}

impl WebSocketClient {
    pub fn new(config: WebSocketConfig) -> Self {
        let (connection_state, set_connection_state) = create_signal(ConnectionState::Disconnected);
        let (incoming_messages, set_incoming_messages) = create_signal(Vec::<WebSocketMessage>::new());
        let (location_updates, set_location_updates) = create_signal(Vec::<LocationUpdate>::new());
        let (booking_updates, set_booking_updates) = create_signal(Vec::<BookingStatusUpdate>::new());
        let (emergency_alerts, set_emergency_alerts) = create_signal(Vec::<EmergencyAlert>::new());
        let (chat_messages, set_chat_messages) = create_signal(Vec::<ChatMessage>::new());
        let (payment_notifications, set_payment_notifications) = create_signal(Vec::<PaymentNotification>::new());
        
        Self {
            config,
            state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            message_queue: Arc::new(Mutex::new(VecDeque::new())),
            reconnect_attempts: Arc::new(Mutex::new(0)),
            connection_state,
            set_connection_state,
            incoming_messages,
            set_incoming_messages,
            location_updates,
            set_location_updates,
            booking_updates,
            set_booking_updates,
            emergency_alerts,
            set_emergency_alerts,
            chat_messages,
            set_chat_messages,
            payment_notifications,
            set_payment_notifications,
        }
    }
    
    // Connect to WebSocket server
    pub async fn connect(&self) -> Result<(), WebSocketError> {
        log::info!("🔌 Connecting to WebSocket: {}", self.config.url);
        self.set_connection_state.set(ConnectionState::Connecting);
        
        // Build connection URL with authentication
        let mut url = self.config.url.clone();
        if let Some(token) = &self.config.auth_token {
            url.push_str(&format!("?token={}&user_id={}&role={}", 
                token, 
                self.config.user_id, 
                self.config.user_role
            ));
        }
        
        // Establish WebSocket connection
        match WebSocket::open(&url) {
            Ok(ws) => {
                log::info!("✅ WebSocket connection established");
                self.set_connection_state.set(ConnectionState::Connected);
                
                // Reset reconnect attempts on successful connection
                *self.reconnect_attempts.lock().unwrap() = 0;
                
                // Start message handling
                self.handle_websocket_stream(ws).await;
                
                Ok(())
            },
            Err(e) => {
                log::error!("❌ WebSocket connection failed: {:?}", e);
                self.set_connection_state.set(ConnectionState::Failed);
                
                // Attempt reconnection if enabled
                if self.config.auto_reconnect {
                    self.attempt_reconnect().await;
                }
                
                Err(e)
            }
        }
    }
    
    // Handle incoming WebSocket messages
    async fn handle_websocket_stream(&self, ws: WebSocket) {
        let (mut write, mut read) = ws.split();
        
        // Send initial connection message
        let connect_msg = json!({
            "type": "connect",
            "user_id": self.config.user_id,
            "role": self.config.user_role,
            "timestamp": Utc::now().to_rfc3339()
        });
        
        if let Err(e) = write.send(Message::Text(connect_msg.to_string())).await {
            log::error!("Failed to send connect message: {:?}", e);
        }
        
        // Process incoming messages
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Err(e) = self.handle_text_message(text).await {
                        log::error!("Error handling message: {:?}", e);
                    }
                },
                Ok(Message::Bytes(bytes)) => {
                    log::debug!("Received binary message: {} bytes", bytes.len());
                },
                Err(e) => {
                    log::error!("WebSocket error: {:?}", e);
                    self.set_connection_state.set(ConnectionState::Failed);
                    
                    if self.config.auto_reconnect {
                        self.attempt_reconnect().await;
                    }
                    break;
                }
            }
        }
    }
    
    // Process text messages from server
    async fn handle_text_message(&self, text: String) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("📨 Received message: {}", text);
        
        // Parse JSON message
        let message: WebSocketMessage = serde_json::from_str(&text)?;
        
        // Add to incoming messages
        self.set_incoming_messages.update(|messages| {
            messages.push(message.clone());
            // Keep only last 100 messages
            if messages.len() > 100 {
                messages.remove(0);
            }
        });
        
        // Route message based on type
        match message.message_type {
            MessageType::ProviderLocationUpdate => {
                if let Ok(location) = serde_json::from_value::<LocationUpdate>(message.payload) {
                    self.set_location_updates.update(|updates| {
                        updates.push(location);
                        if updates.len() > 50 {
                            updates.remove(0);
                        }
                    });
                }
            },
            MessageType::BookingStatusUpdate => {
                if let Ok(booking) = serde_json::from_value::<BookingStatusUpdate>(message.payload) {
                    self.set_booking_updates.update(|updates| {
                        updates.push(booking);
                        if updates.len() > 20 {
                            updates.remove(0);
                        }
                    });
                }
            },
            MessageType::EmergencyAlert => {
                if let Ok(alert) = serde_json::from_value::<EmergencyAlert>(message.payload) {
                    self.set_emergency_alerts.update(|alerts| {
                        alerts.push(alert);
                        if alerts.len() > 10 {
                            alerts.remove(0);
                        }
                    });
                }
            },
            MessageType::ChatMessage => {
                if let Ok(chat) = serde_json::from_value::<ChatMessage>(message.payload) {
                    self.set_chat_messages.update(|messages| {
                        messages.push(chat);
                        if messages.len() > 100 {
                            messages.remove(0);
                        }
                    });
                }
            },
            MessageType::PaymentNotification => {
                if let Ok(payment) = serde_json::from_value::<PaymentNotification>(message.payload) {
                    self.set_payment_notifications.update(|notifications| {
                        notifications.push(payment);
                        if notifications.len() > 20 {
                            notifications.remove(0);
                        }
                    });
                }
            },
            MessageType::ConnectionAck => {
                log::info!("✅ Connection acknowledged by server");
            },
            MessageType::Heartbeat => {
                log::debug!("💓 Heartbeat received");
            }
        }
        
        Ok(())
    }
    
    // Send message to server
    pub async fn send_message(&self, message: WebSocketMessage) -> Result<(), Box<dyn std::error::Error>> {
        let json_msg = serde_json::to_string(&message)?;
        log::debug!("📤 Sending message: {}", json_msg);
        
        // For now, queue the message (in real implementation, send via WebSocket)
        self.message_queue.lock().unwrap().push_back(message);
        
        Ok(())
    }
    
    // Send emergency alert
    pub async fn send_emergency_alert(&self, alert: EmergencyAlert) -> Result<(), Box<dyn std::error::Error>> {
        let message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::EmergencyAlert,
            sender_id: self.config.user_id.clone(),
            recipient_id: None,
            channel: "emergency".to_string(),
            payload: serde_json::to_value(alert)?,
            timestamp: Utc::now(),
            priority: MessagePriority::Critical,
        };
        
        self.send_message(message).await
    }
    
    // Send location update (for providers)
    pub async fn send_location_update(&self, location: LocationUpdate) -> Result<(), Box<dyn std::error::Error>> {
        let message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::ProviderLocationUpdate,
            sender_id: self.config.user_id.clone(),
            recipient_id: None,
            channel: "provider_tracking".to_string(),
            payload: serde_json::to_value(location)?,
            timestamp: Utc::now(),
            priority: MessagePriority::High,
        };
        
        self.send_message(message).await
    }
    
    // Send chat message
    pub async fn send_chat_message(&self, chat: ChatMessage) -> Result<(), Box<dyn std::error::Error>> {
        let message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::ChatMessage,
            sender_id: self.config.user_id.clone(),
            recipient_id: Some(chat.recipient_id.clone()),
            channel: format!("chat_{}", chat.booking_id),
            payload: serde_json::to_value(chat)?,
            timestamp: Utc::now(),
            priority: MessagePriority::Normal,
        };
        
        self.send_message(message).await
    }
    
    // Attempt reconnection with exponential backoff
    async fn attempt_reconnect(&self) {
        let mut attempts = self.reconnect_attempts.lock().unwrap();
        
        if *attempts >= self.config.max_reconnect_attempts {
            log::error!("❌ Max reconnection attempts reached");
            self.set_connection_state.set(ConnectionState::Failed);
            return;
        }
        
        *attempts += 1;
        let delay = std::cmp::min(1000 * 2_u64.pow(*attempts), 30000); // Max 30 seconds
        
        log::info!("🔄 Reconnecting in {}ms (attempt {}/{})", 
                  delay, *attempts, self.config.max_reconnect_attempts);
        
        self.set_connection_state.set(ConnectionState::Reconnecting);
        
        // Wait before reconnecting
        TimeoutFuture::new(delay as u32).await;
        
        // Attempt reconnection
        if let Err(e) = self.connect().await {
            log::error!("Reconnection failed: {:?}", e);
        }
    }
    
    // Disconnect from server
    pub async fn disconnect(&self) {
        log::info!("🔌 Disconnecting WebSocket");
        self.set_connection_state.set(ConnectionState::Disconnected);
        *self.reconnect_attempts.lock().unwrap() = 0;
    }
    
    // Get connection statistics
    pub fn get_stats(&self) -> JsonValue {
        let state = self.state.lock().unwrap();
        let queue_size = self.message_queue.lock().unwrap().len();
        let attempts = *self.reconnect_attempts.lock().unwrap();
        
        json!({
            "connection_state": format!("{:?}", *state),
            "queued_messages": queue_size,
            "reconnect_attempts": attempts,
            "user_id": self.config.user_id,
            "user_role": self.config.user_role,
            "auto_reconnect": self.config.auto_reconnect
        })
    }
}

// Convenience functions for common operations
impl WebSocketClient {
    // Quick emergency button press
    pub async fn emergency_button_pressed(&self, location: (f64, f64), alert_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        let alert = EmergencyAlert {
            alert_id: Uuid::new_v4().to_string(),
            patient_id: self.config.user_id.clone(),
            location,
            alert_type: alert_type.to_string(),
            severity: 9, // High severity for button press
            description: "Emergency button activated".to_string(),
            medical_conditions: vec![],
            emergency_contacts: vec![],
        };
        
        self.send_emergency_alert(alert).await
    }
    
    // Update provider status (for provider app)
    pub async fn update_provider_status(&self, status: &str, location: Option<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some((lat, lng)) = location {
            let location_update = LocationUpdate {
                provider_id: self.config.user_id.clone(),
                latitude: lat,
                longitude: lng,
                accuracy: 10.0,
                heading: None,
                speed: None,
                timestamp: Utc::now(),
                status: status.to_string(),
            };
            
            self.send_location_update(location_update).await
        } else {
            Ok(())
        }
    }
}
