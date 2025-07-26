//! Utility functions and helpers for MyDR24 shared library

use crate::errors::{SharedError, SharedResult};
use std::collections::HashMap;

/// Validation utilities for healthcare data
pub mod validation {
    use super::*;
    use regex::Regex;

    /// Validate email address format
    pub fn validate_email(email: &str) -> SharedResult<()> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|e| SharedError::ValidationError(format!("Regex error: {}", e)))?;
        
        if email_regex.is_match(email) {
            Ok(())
        } else {
            Err(SharedError::ValidationError("Invalid email format".to_string()))
        }
    }

    /// Validate phone number (international format)
    pub fn validate_phone(phone: &str) -> SharedResult<()> {
        let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$")
            .map_err(|e| SharedError::ValidationError(format!("Regex error: {}", e)))?;
        
        if phone_regex.is_match(phone) {
            Ok(())
        } else {
            Err(SharedError::ValidationError("Invalid phone number format".to_string()))
        }
    }

    /// Validate medical license number
    pub fn validate_medical_license(license: &str) -> SharedResult<()> {
        if license.len() < 6 || license.len() > 20 {
            return Err(SharedError::ValidationError(
                "Medical license must be 6-20 characters".to_string()
            ));
        }

        let license_regex = Regex::new(r"^[A-Z0-9]{6,20}$")
            .map_err(|e| SharedError::ValidationError(format!("Regex error: {}", e)))?;
        
        if license_regex.is_match(license) {
            Ok(())
        } else {
            Err(SharedError::ValidationError(
                "Medical license must contain only uppercase letters and numbers".to_string()
            ))
        }
    }

    /// Validate password strength
    pub fn validate_password(password: &str) -> SharedResult<()> {
        if password.len() < 12 {
            return Err(SharedError::ValidationError(
                "Password must be at least 12 characters long".to_string()
            ));
        }

        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

        if !(has_upper && has_lower && has_digit && has_special) {
            return Err(SharedError::ValidationError(
                "Password must contain uppercase, lowercase, digit, and special character".to_string()
            ));
        }

        Ok(())
    }

    /// Validate date of birth (must be realistic for healthcare)
    pub fn validate_date_of_birth(dob: &chrono::NaiveDate) -> SharedResult<()> {
        let today = chrono::Utc::now().naive_utc().date();
        let min_date = today - chrono::Duration::days(365 * 150); // 150 years ago
        let max_date = today; // Cannot be in the future

        if *dob < min_date {
            return Err(SharedError::ValidationError(
                "Date of birth cannot be more than 150 years ago".to_string()
            ));
        }

        if *dob > max_date {
            return Err(SharedError::ValidationError(
                "Date of birth cannot be in the future".to_string()
            ));
        }

        Ok(())
    }
}

/// Date and time utilities for healthcare scheduling
pub mod datetime {
    use super::*;
    use chrono::{DateTime, Utc, NaiveTime, Weekday, Datelike, Timelike, Duration};

    /// Check if a time slot is within business hours
    pub fn is_business_hours(datetime: &DateTime<Utc>, timezone: &str) -> SharedResult<bool> {
        let tz: chrono_tz::Tz = timezone.parse()
            .map_err(|e| SharedError::ValidationError(format!("Invalid timezone: {}", e)))?;
        
        let local_time = datetime.with_timezone(&tz).time();
        let business_start = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let business_end = NaiveTime::from_hms_opt(18, 0, 0).unwrap();

        Ok(local_time >= business_start && local_time <= business_end)
    }

    /// Check if a date is a weekday
    pub fn is_weekday(datetime: &DateTime<Utc>) -> bool {
        let weekday = datetime.weekday();
        !matches!(weekday, Weekday::Sat | Weekday::Sun)
    }

    /// Calculate appointment duration in minutes
    pub fn calculate_duration(start: &DateTime<Utc>, end: &DateTime<Utc>) -> i64 {
        (end.timestamp() - start.timestamp()) / 60
    }

    /// Get next available time slot
    pub fn next_available_slot(
        current: &DateTime<Utc>,
        duration_minutes: i64,
        timezone: &str
    ) -> SharedResult<DateTime<Utc>> {
        let mut next_slot = *current;
        
        // Round up to next 15-minute interval
        let minutes = next_slot.minute();
        let rounded_minutes = ((minutes + 14) / 15) * 15;
        next_slot = next_slot.with_minute(rounded_minutes % 60).unwrap();
        if rounded_minutes >= 60 {
            next_slot = next_slot + chrono::Duration::hours(1);
            next_slot = next_slot.with_minute(0).unwrap();
        }

        // Ensure it's during business hours and weekday
        while !is_business_hours(&next_slot, timezone)? || !is_weekday(&next_slot) {
            next_slot = next_slot + chrono::Duration::hours(1);
        }

        Ok(next_slot)
    }
}

/// String manipulation utilities
pub mod strings {
    use super::*;

    /// Sanitize string for HIPAA compliance (remove PII patterns)
    pub fn sanitize_for_logging(input: &str) -> String {
        let email_regex = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        let phone_regex = regex::Regex::new(r"\b\d{3}-?\d{3}-?\d{4}\b").unwrap();
        let ssn_regex = regex::Regex::new(r"\b\d{3}-?\d{2}-?\d{4}\b").unwrap();
        
        let mut sanitized = email_regex.replace_all(input, "[EMAIL_REDACTED]").to_string();
        sanitized = phone_regex.replace_all(&sanitized, "[PHONE_REDACTED]").to_string();
        sanitized = ssn_regex.replace_all(&sanitized, "[SSN_REDACTED]").to_string();
        
        sanitized
    }

    /// Generate random alphanumeric string
    pub fn generate_random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Format medical record number
    pub fn format_medical_record_number(patient_id: &uuid::Uuid) -> String {
        format!("MRN-{}", patient_id.to_string().replace("-", "").to_uppercase())
    }

    /// Truncate string with ellipsis
    pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }
}

/// Encryption and security utilities
pub mod security {
    use super::*;
    use sha2::{Sha256, Digest};

    /// Hash sensitive data for logging/audit
    pub fn hash_for_audit(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Generate secure session token
    pub fn generate_session_token() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Mask sensitive data for display
    pub fn mask_sensitive_data(data: &str, visible_chars: usize) -> String {
        if data.len() <= visible_chars * 2 {
            "*".repeat(data.len())
        } else {
            let start = &data[..visible_chars];
            let end = &data[data.len() - visible_chars..];
            format!("{}{}{}",
                start,
                "*".repeat(data.len() - visible_chars * 2),
                end
            )
        }
    }
}

/// Configuration and environment utilities
pub mod config {
    use super::*;
    use std::env;

    /// Get environment variable with default
    pub fn get_env_or_default(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Parse boolean from environment variable
    pub fn get_env_bool(key: &str, default: bool) -> bool {
        env::var(key)
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(default)
    }

    /// Parse integer from environment variable
    pub fn get_env_u16(key: &str, default: u16) -> SharedResult<u16> {
        env::var(key)
            .map(|v| v.parse::<u16>())
            .unwrap_or(Ok(default))
            .map_err(|e| SharedError::ConfigurationError(format!("Invalid {}: {}", key, e)))
    }

    /// Validate required environment variables
    pub fn validate_required_env_vars(vars: &[&str]) -> SharedResult<()> {
        let missing: Vec<&str> = vars.iter()
            .filter(|&&var| env::var(var).is_err())
            .copied()
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(SharedError::ConfigurationError(
                format!("Missing required environment variables: {}", missing.join(", "))
            ))
        }
    }
}

/// HTTP utilities for API communication
pub mod http {
    use super::*;
    use std::collections::HashMap;

    /// Extract Bearer token from Authorization header
    pub fn extract_bearer_token(auth_header: &str) -> SharedResult<String> {
        if !auth_header.starts_with("Bearer ") {
            return Err(SharedError::AuthenticationError(
                "Invalid Authorization header format".to_string()
            ));
        }

        let token = auth_header.strip_prefix("Bearer ").unwrap().trim();
        if token.is_empty() {
            return Err(SharedError::AuthenticationError(
                "Missing token in Authorization header".to_string()
            ));
        }

        Ok(token.to_string())
    }

    /// Build CORS headers for healthcare APIs
    pub fn build_cors_headers() -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string());
        headers.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization, X-Request-ID".to_string());
        headers.insert("Access-Control-Max-Age".to_string(), "86400".to_string());
        headers
    }

    /// Generate request ID for tracing
    pub fn generate_request_id() -> String {
        format!("req_{}", uuid::Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(validation::validate_email("test@example.com").is_ok());
        assert!(validation::validate_email("invalid-email").is_err());
    }

    #[test]
    fn test_phone_validation() {
        assert!(validation::validate_phone("+1234567890").is_ok());
        assert!(validation::validate_phone("123456789012345").is_ok());
        assert!(validation::validate_phone("invalid").is_err());
    }

    #[test]
    fn test_password_validation() {
        assert!(validation::validate_password("SecurePass123!").is_ok());
        assert!(validation::validate_password("weak").is_err());
        assert!(validation::validate_password("NoSpecialChar123").is_err());
    }

    #[test]
    fn test_sanitize_logging() {
        let input = "User test@example.com called 555-123-4567";
        let sanitized = strings::sanitize_for_logging(input);
        assert!(!sanitized.contains("test@example.com"));
        assert!(!sanitized.contains("555-123-4567"));
    }

    #[test]
    fn test_mask_sensitive_data() {
        let data = "1234567890";
        let masked = security::mask_sensitive_data(data, 2);
        assert_eq!(masked, "12******90");
    }

    #[test]
    fn test_bearer_token_extraction() {
        assert!(http::extract_bearer_token("Bearer token123").is_ok());
        assert!(http::extract_bearer_token("Invalid header").is_err());
    }
}
