pub mod models;
pub mod ui;
pub mod websocket_simple;
pub mod chat_simple;
pub mod emergency_simple;
pub mod location_simple;

pub use models::*;
pub use ui::*;
pub use websocket_simple::*;
pub use chat_simple::*;
pub use emergency_simple::*;
pub use location_simple::*;

// Healthcare compliance utilities
pub mod compliance {
    use crate::models::*;
    
    /// HIPAA compliance checker
    pub fn validate_hipaa_compliance(patient_data: &Patient) -> Result<(), String> {
        // TODO: Implement HIPAA validation logic
        Ok(())
    }
    
    /// NMC (National Medical Commission) provider verification
    pub fn verify_nmc_registration(provider: &Provider) -> Result<bool, String> {
        // TODO: Implement NMC verification logic
        Ok(true)
    }
    
    /// GDPR data protection compliance
    pub fn ensure_gdpr_compliance(data: &str) -> Result<String, String> {
        // TODO: Implement GDPR anonymization
        Ok(data.to_string())
    }
}

// Utility functions
pub mod utils {
    use uuid::Uuid;
    
    pub fn generate_medical_record_number() -> String {
        format!("MR{}", Uuid::new_v4().to_string().replace("-", "").to_uppercase()[..8].to_string())
    }
    
    pub fn validate_phone_number(phone: &str) -> bool {
        // Basic Indian phone number validation
        phone.len() >= 10 && phone.chars().all(|c| c.is_ascii_digit() || c == '+' || c == '-' || c == ' ')
    }
}

// Error handling
pub mod errors {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum MyDR24Error {
        #[error("Database error: {0}")]
        Database(String),
        
        #[error("Authentication error: {0}")]
        Authentication(String),
        
        #[error("Authorization error: {0}")]
        Authorization(String),
        
        #[error("Validation error: {0}")]
        Validation(String),
        
        #[error("Healthcare compliance error: {0}")]
        Compliance(String),
        
        #[error("External service error: {0}")]
        ExternalService(String),
    }
}
