//! Error types and handling for MyDR24 shared library

use thiserror::Error;

/// Shared error types for MyDR24 platform
#[derive(Error, Debug, Clone)]
pub enum SharedError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Cryptographic error: {0}")]
    CryptographicError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Healthcare compliance error: {0}")]
    ComplianceError(String),

    #[error("HIPAA violation: {0}")]
    HipaaViolation(String),

    #[error("GDPR violation: {0}")]
    GdprViolation(String),

    #[error("Audit error: {0}")]
    AuditError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailableError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Medical record error: {0}")]
    MedicalRecordError(String),

    #[error("Appointment scheduling error: {0}")]
    SchedulingError(String),

    #[error("Payment processing error: {0}")]
    PaymentError(String),

    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("Emergency alert error: {0}")]
    EmergencyError(String),

    #[error("Integration error: {0}")]
    IntegrationError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

/// Result type alias for MyDR24 operations
pub type SharedResult<T> = Result<T, SharedError>;

/// Error context for debugging and audit trails
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorContext {
    pub error_id: uuid::Uuid,
    pub error_code: String,
    pub error_message: String,
    pub user_id: Option<uuid::Uuid>,
    pub request_id: Option<uuid::Uuid>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub service: String,
    pub operation: String,
    pub additional_data: serde_json::Value,
}

impl ErrorContext {
    pub fn new(error: &SharedError, service: &str, operation: &str) -> Self {
        Self {
            error_id: uuid::Uuid::new_v4(),
            error_code: error.error_code(),
            error_message: error.to_string(),
            user_id: None,
            request_id: None,
            timestamp: chrono::Utc::now(),
            service: service.to_string(),
            operation: operation.to_string(),
            additional_data: serde_json::Value::Null,
        }
    }

    pub fn with_user_id(mut self, user_id: uuid::Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_request_id(mut self, request_id: uuid::Uuid) -> Self {
        self.request_id = Some(request_id);
        self
    }

    pub fn with_data<T: serde::Serialize>(mut self, data: &T) -> Self {
        self.additional_data = serde_json::to_value(data).unwrap_or(serde_json::Value::Null);
        self
    }
}

impl SharedError {
    /// Get error code for classification and monitoring
    pub fn error_code(&self) -> String {
        match self {
            SharedError::AuthenticationError(_) => "AUTH_001".to_string(),
            SharedError::AuthorizationError(_) => "AUTH_002".to_string(),
            SharedError::ValidationError(_) => "VAL_001".to_string(),
            SharedError::DatabaseError(_) => "DB_001".to_string(),
            SharedError::NetworkError(_) => "NET_001".to_string(),
            SharedError::CryptographicError(_) => "CRYPTO_001".to_string(),
            SharedError::ConfigurationError(_) => "CONFIG_001".to_string(),
            SharedError::ComplianceError(_) => "COMP_001".to_string(),
            SharedError::HipaaViolation(_) => "HIPAA_001".to_string(),
            SharedError::GdprViolation(_) => "GDPR_001".to_string(),
            SharedError::AuditError(_) => "AUDIT_001".to_string(),
            SharedError::RateLimitError(_) => "RATE_001".to_string(),
            SharedError::NotFoundError(_) => "NOT_FOUND_001".to_string(),
            SharedError::ServiceUnavailableError(_) => "SVC_001".to_string(),
            SharedError::TimeoutError(_) => "TIMEOUT_001".to_string(),
            SharedError::SerializationError(_) => "SER_001".to_string(),
            SharedError::MedicalRecordError(_) => "MED_001".to_string(),
            SharedError::SchedulingError(_) => "SCHED_001".to_string(),
            SharedError::PaymentError(_) => "PAY_001".to_string(),
            SharedError::WebSocketError(_) => "WS_001".to_string(),
            SharedError::EmergencyError(_) => "EMRG_001".to_string(),
            SharedError::IntegrationError(_) => "INT_001".to_string(),
            SharedError::InternalError(_) => "INT_500".to_string(),
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SharedError::NetworkError(_)
                | SharedError::ServiceUnavailableError(_)
                | SharedError::TimeoutError(_)
                | SharedError::DatabaseError(_)
        )
    }

    /// Check if error requires immediate attention
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            SharedError::HipaaViolation(_)
                | SharedError::GdprViolation(_)
                | SharedError::EmergencyError(_)
                | SharedError::CryptographicError(_)
        )
    }

    /// Check if error should be logged for audit
    pub fn requires_audit(&self) -> bool {
        matches!(
            self,
            SharedError::AuthenticationError(_)
                | SharedError::AuthorizationError(_)
                | SharedError::HipaaViolation(_)
                | SharedError::GdprViolation(_)
                | SharedError::ComplianceError(_)
                | SharedError::MedicalRecordError(_)
                | SharedError::EmergencyError(_)
        )
    }

    /// Get HTTP status code equivalent
    pub fn http_status_code(&self) -> u16 {
        match self {
            SharedError::AuthenticationError(_) => 401,
            SharedError::AuthorizationError(_) => 403,
            SharedError::ValidationError(_) => 400,
            SharedError::NotFoundError(_) => 404,
            SharedError::RateLimitError(_) => 429,
            SharedError::ServiceUnavailableError(_) => 503,
            SharedError::TimeoutError(_) => 408,
            SharedError::HipaaViolation(_) | SharedError::GdprViolation(_) => 451,
            _ => 500,
        }
    }
}

// Implement From for common error types
impl From<serde_json::Error> for SharedError {
    fn from(err: serde_json::Error) -> Self {
        SharedError::SerializationError(err.to_string())
    }
}

impl From<uuid::Error> for SharedError {
    fn from(err: uuid::Error) -> Self {
        SharedError::ValidationError(format!("Invalid UUID: {}", err))
    }
}

impl From<chrono::ParseError> for SharedError {
    fn from(err: chrono::ParseError) -> Self {
        SharedError::ValidationError(format!("Invalid datetime: {}", err))
    }
}

impl From<base64::DecodeError> for SharedError {
    fn from(err: base64::DecodeError) -> Self {
        SharedError::CryptographicError(format!("Base64 decode error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        let auth_error = SharedError::AuthenticationError("test".to_string());
        assert_eq!(auth_error.error_code(), "AUTH_001");
        
        let hipaa_error = SharedError::HipaaViolation("test".to_string());
        assert_eq!(hipaa_error.error_code(), "HIPAA_001");
    }

    #[test]
    fn test_error_classification() {
        let network_error = SharedError::NetworkError("test".to_string());
        assert!(network_error.is_retryable());
        assert!(!network_error.is_critical());

        let hipaa_error = SharedError::HipaaViolation("test".to_string());
        assert!(!hipaa_error.is_retryable());
        assert!(hipaa_error.is_critical());
        assert!(hipaa_error.requires_audit());
    }

    #[test]
    fn test_http_status_codes() {
        assert_eq!(
            SharedError::AuthenticationError("test".to_string()).http_status_code(),
            401
        );
        assert_eq!(
            SharedError::NotFoundError("test".to_string()).http_status_code(),
            404
        );
        assert_eq!(
            SharedError::HipaaViolation("test".to_string()).http_status_code(),
            451
        );
    }

    #[test]
    fn test_error_context() {
        let error = SharedError::ValidationError("test error".to_string());
        let context = ErrorContext::new(&error, "auth_service", "register_user")
            .with_user_id(uuid::Uuid::new_v4());

        assert_eq!(context.error_code, "VAL_001");
        assert_eq!(context.service, "auth_service");
        assert_eq!(context.operation, "register_user");
        assert!(context.user_id.is_some());
    }
}
