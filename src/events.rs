//! WebSocket events and real-time messaging structures

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::Appointment;

/// WebSocket event types for real-time communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketEvent {
    // Authentication events
    UserConnected { user_id: Uuid, role: String },
    UserDisconnected { user_id: Uuid },
    
    // Appointment events
    AppointmentScheduled { appointment: Appointment },
    AppointmentUpdated { appointment: Appointment },
    AppointmentCancelled { appointment_id: Uuid, reason: String },
    AppointmentReminder { appointment_id: Uuid, minutes_until: u32 },
    
    // Chat and communication
    ChatMessage { message: ChatMessage },
    TypingIndicator { user_id: Uuid, is_typing: bool },
    
    // Emergency and alerts
    EmergencyAlert { alert: EmergencyAlert },
    SystemNotification { notification: SystemNotification },
    
    // Provider location and availability
    ProviderLocationUpdate { provider_id: Uuid, location: GeoLocation },
    ProviderAvailabilityUpdate { provider_id: Uuid, is_available: bool },
    
    // Booking and status updates
    BookingStatusUpdate { booking_id: Uuid, status: String },
    PaymentStatusUpdate { payment_id: Uuid, status: String },
    
    // System events
    SystemMaintenance { message: String, scheduled_time: DateTime<Utc> },
    ServerStatus { status: ServerStatus },
    
    // Call and video conference
    IncomingCall { call_id: Uuid, from_user: Uuid, to_user: Uuid },
    CallEnded { call_id: Uuid },
    VideoStreamUpdate { call_id: Uuid, stream_info: StreamInfo },
}

/// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub id: Uuid,
    pub appointment_id: Uuid,
    pub sender_id: Uuid,
    pub sender_role: String, // "patient", "provider", "system"
    pub message: String,
    pub message_type: MessageType,
    pub timestamp: DateTime<Utc>,
    pub is_encrypted: bool,
    pub attachments: Vec<MessageAttachment>,
    pub reply_to: Option<Uuid>, // ID of message being replied to
}

/// Message type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Text,
    Image,
    Document,
    Voice,
    Video,
    System,
    Prescription,
    LabReport,
}

/// Message attachment structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageAttachment {
    pub id: Uuid,
    pub filename: String,
    pub file_type: String,
    pub file_size: u64,
    pub url: String,
    pub is_encrypted: bool,
}

/// Emergency alert structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmergencyAlert {
    pub id: Uuid,
    pub alert_type: EmergencyType,
    pub severity: AlertSeverity,
    pub message: String,
    pub affected_users: Vec<Uuid>,
    pub location: Option<GeoLocation>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub action_required: bool,
    pub emergency_contact: Option<String>,
}

/// Emergency alert types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmergencyType {
    MedicalEmergency,
    SystemOutage,
    SecurityBreach,
    NetworkFailure,
    WeatherAlert,
    ProviderUnavailable,
    AppointmentConflict,
    PaymentFailure,
    Other(String),
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Critical,  // Immediate action required
    High,      // Action required within minutes
    Medium,    // Action required within hours
    Low,       // Informational, no immediate action
    Info,      // General information
}

/// System notification structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemNotification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub target_users: Vec<Uuid>,
    pub target_roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub action_url: Option<String>,
    pub is_dismissible: bool,
}

/// Notification types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    AppointmentReminder,
    PaymentDue,
    SystemUpdate,
    FeatureAnnouncement,
    PolicyUpdate,
    PromotionalOffer,
    SecurityAlert,
    AccountUpdate,
}

/// Geographic location structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: Option<f64>, // meters
    pub altitude: Option<f64>, // meters
    pub timestamp: DateTime<Utc>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

/// Server status information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerStatus {
    pub status: ServiceStatus,
    pub uptime: u64, // seconds
    pub active_connections: u32,
    pub total_users_online: u32,
    pub last_maintenance: DateTime<Utc>,
    pub next_maintenance: Option<DateTime<Utc>>,
    pub version: String,
}

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Online,
    Degraded,
    Maintenance,
    Offline,
}

/// Video/audio stream information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamInfo {
    pub stream_id: Uuid,
    pub stream_type: StreamType,
    pub quality: StreamQuality,
    pub bitrate: u32,
    pub resolution: Option<String>, // "1920x1080", "1280x720", etc.
    pub codec: String,
    pub is_active: bool,
}

/// Stream type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamType {
    Audio,
    Video,
    Screen,
}

/// Stream quality levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamQuality {
    Low,
    Medium,
    High,
    UltraHigh,
}

/// WebSocket connection info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub connection_id: Uuid,
    pub user_id: Uuid,
    pub user_role: String,
    pub connected_at: DateTime<Utc>,
    pub last_ping: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: Option<String>,
}

impl ChatMessage {
    pub fn new_text_message(
        appointment_id: Uuid,
        sender_id: Uuid,
        sender_role: &str,
        message: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            appointment_id,
            sender_id,
            sender_role: sender_role.to_string(),
            message: message.to_string(),
            message_type: MessageType::Text,
            timestamp: Utc::now(),
            is_encrypted: true, // Default to encrypted for healthcare
            attachments: vec![],
            reply_to: None,
        }
    }
}

impl EmergencyAlert {
    pub fn new_medical_emergency(
        message: &str,
        affected_users: Vec<Uuid>,
        location: Option<GeoLocation>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            alert_type: EmergencyType::MedicalEmergency,
            severity: AlertSeverity::Critical,
            message: message.to_string(),
            affected_users,
            location,
            created_at: Utc::now(),
            expires_at: None, // Medical emergencies don't expire automatically
            action_required: true,
            emergency_contact: Some("911".to_string()),
        }
    }
}

impl SystemNotification {
    pub fn new_appointment_reminder(
        appointment_id: Uuid,
        patient_id: Uuid,
        message: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "Appointment Reminder".to_string(),
            message: message.to_string(),
            notification_type: NotificationType::AppointmentReminder,
            target_users: vec![patient_id],
            target_roles: vec!["patient".to_string()],
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::days(1)),
            action_url: Some(format!("/appointments/{}", appointment_id)),
            is_dismissible: true,
        }
    }
}
