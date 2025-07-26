//! Healthcare compliance utilities and HIPAA/GDPR support

use crate::errors::{SharedError, SharedResult};
use serde::{Deserialize, Serialize};

/// HIPAA compliance utilities
pub mod hipaa {
    use super::*;
    use chrono::{DateTime, Utc};

    /// HIPAA audit log entry
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HipaaAuditEntry {
        pub entry_id: uuid::Uuid,
        pub timestamp: DateTime<Utc>,
        pub user_id: Option<uuid::Uuid>,
        pub patient_id: Option<uuid::Uuid>,
        pub action: HipaaAction,
        pub resource_type: String,
        pub resource_id: String,
        pub ip_address: Option<String>,
        pub user_agent: Option<String>,
        pub outcome: AuditOutcome,
        pub details: serde_json::Value,
    }

    /// HIPAA auditable actions
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum HipaaAction {
        Create,
        Read,
        Update,
        Delete,
        Access,
        Login,
        Logout,
        Export,
        Print,
        Share,
        Decrypt,
        Backup,
        Restore,
    }

    /// Audit outcome status
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AuditOutcome {
        Success,
        Failure,
        Warning,
    }

    /// Protected Health Information (PHI) classifier
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PhiClassification {
        pub contains_phi: bool,
        pub phi_types: Vec<PhiType>,
        pub risk_level: RiskLevel,
        pub required_protections: Vec<String>,
    }

    /// Types of PHI that can be identified
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum PhiType {
        Name,
        Address,
        DateOfBirth,
        PhoneNumber,
        Email,
        SocialSecurityNumber,
        MedicalRecordNumber,
        AccountNumber,
        LicenseNumber,
        BiometricData,
        PhotographicImage,
        IpAddress,
    }

    /// Risk levels for PHI exposure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum RiskLevel {
        Low,
        Medium,
        High,
        Critical,
    }

    impl HipaaAuditEntry {
        pub fn new(
            action: HipaaAction,
            resource_type: &str,
            resource_id: &str,
            outcome: AuditOutcome,
        ) -> Self {
            Self {
                entry_id: uuid::Uuid::new_v4(),
                timestamp: Utc::now(),
                user_id: None,
                patient_id: None,
                action,
                resource_type: resource_type.to_string(),
                resource_id: resource_id.to_string(),
                ip_address: None,
                user_agent: None,
                outcome,
                details: serde_json::Value::Null,
            }
        }

        pub fn with_user(mut self, user_id: uuid::Uuid) -> Self {
            self.user_id = Some(user_id);
            self
        }

        pub fn with_patient(mut self, patient_id: uuid::Uuid) -> Self {
            self.patient_id = Some(patient_id);
            self
        }

        pub fn with_request_info(mut self, ip: Option<String>, user_agent: Option<String>) -> Self {
            self.ip_address = ip;
            self.user_agent = user_agent;
            self
        }

        pub fn with_details<T: Serialize>(mut self, details: &T) -> Self {
            self.details = serde_json::to_value(details).unwrap_or(serde_json::Value::Null);
            self
        }
    }

    /// Classify text for PHI content
    pub fn classify_phi(text: &str) -> PhiClassification {
        let mut phi_types = Vec::new();
        let mut contains_phi = false;

        // Email detection
        if regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")
            .unwrap()
            .is_match(text)
        {
            phi_types.push(PhiType::Email);
            contains_phi = true;
        }

        // Phone number detection
        if regex::Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b")
            .unwrap()
            .is_match(text)
        {
            phi_types.push(PhiType::PhoneNumber);
            contains_phi = true;
        }

        // SSN detection
        if regex::Regex::new(r"\b\d{3}-\d{2}-\d{4}\b")
            .unwrap()
            .is_match(text)
        {
            phi_types.push(PhiType::SocialSecurityNumber);
            contains_phi = true;
        }

        // Date patterns that might be DOB
        if regex::Regex::new(r"\b\d{1,2}/\d{1,2}/\d{4}\b")
            .unwrap()
            .is_match(text)
        {
            phi_types.push(PhiType::DateOfBirth);
            contains_phi = true;
        }

        // Medical record number patterns
        if regex::Regex::new(r"\bMRN[-:]?\s*\d+\b")
            .unwrap()
            .is_match(text)
        {
            phi_types.push(PhiType::MedicalRecordNumber);
            contains_phi = true;
        }

        let risk_level = if phi_types.contains(&PhiType::SocialSecurityNumber) {
            RiskLevel::Critical
        } else if phi_types.len() >= 3 {
            RiskLevel::High
        } else if phi_types.len() >= 2 {
            RiskLevel::Medium
        } else if contains_phi {
            RiskLevel::Low
        } else {
            RiskLevel::Low
        };

        let required_protections = match risk_level {
            RiskLevel::Critical => vec![
                "Encryption at rest".to_string(),
                "Encryption in transit".to_string(),
                "Access logging".to_string(),
                "Multi-factor authentication".to_string(),
                "Data loss prevention".to_string(),
            ],
            RiskLevel::High => vec![
                "Encryption at rest".to_string(),
                "Encryption in transit".to_string(),
                "Access logging".to_string(),
                "Authentication required".to_string(),
            ],
            RiskLevel::Medium => vec![
                "Encryption in transit".to_string(),
                "Access logging".to_string(),
                "Authentication required".to_string(),
            ],
            RiskLevel::Low => vec![
                "Access logging".to_string(),
            ],
        };

        PhiClassification {
            contains_phi,
            phi_types,
            risk_level,
            required_protections,
        }
    }

    /// Generate minimum necessary access justification
    pub fn validate_minimum_necessary_access(
        user_role: &str,
        requested_data: &[&str],
        purpose: &str,
    ) -> SharedResult<bool> {
        let allowed_data = match user_role {
            "physician" => vec![
                "medical_history", "current_medications", "lab_results", 
                "imaging", "vital_signs", "treatment_notes", "patient_demographics"
            ],
            "nurse" => vec![
                "vital_signs", "current_medications", "treatment_notes", 
                "care_plans", "patient_demographics"
            ],
            "technician" => vec![
                "lab_results", "imaging", "vital_signs"
            ],
            "admin" => vec![
                "patient_demographics", "insurance_info", "billing_info"
            ],
            "patient" => vec![
                "own_medical_history", "own_medications", "own_lab_results",
                "own_imaging", "own_vital_signs", "own_treatment_notes"
            ],
            _ => vec![],
        };

        let unauthorized_access: Vec<&str> = requested_data
            .iter()
            .filter(|&&data| !allowed_data.contains(&data))
            .copied()
            .collect();

        if unauthorized_access.is_empty() {
            Ok(true)
        } else {
            Err(SharedError::HipaaViolation(format!(
                "User role '{}' not authorized to access: {} for purpose: {}",
                user_role,
                unauthorized_access.join(", "),
                purpose
            )))
        }
    }
}

/// GDPR compliance utilities
pub mod gdpr {
    use super::*;
    use chrono::{DateTime, Utc};

    /// GDPR consent record
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConsentRecord {
        pub consent_id: uuid::Uuid,
        pub user_id: uuid::Uuid,
        pub purpose: DataProcessingPurpose,
        pub legal_basis: LegalBasis,
        pub granted_at: DateTime<Utc>,
        pub expires_at: Option<DateTime<Utc>>,
        pub withdrawn_at: Option<DateTime<Utc>>,
        pub consent_text: String,
        pub consent_version: String,
        pub ip_address: Option<String>,
        pub user_agent: Option<String>,
    }

    /// Data processing purposes under GDPR
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum DataProcessingPurpose {
        Healthcare,
        Research,
        Marketing,
        Analytics,
        Communication,
        Legal,
        Emergency,
    }

    /// Legal basis for processing under GDPR Article 6
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum LegalBasis {
        Consent,
        Contract,
        LegalObligation,
        VitalInterests,
        PublicTask,
        LegitimateInterests,
    }

    /// Data subject rights under GDPR
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DataSubjectRight {
        Access,
        Rectification,
        Erasure,
        Portability,
        Restriction,
        Objection,
        WithdrawConsent,
    }

    /// GDPR compliance status
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ComplianceStatus {
        pub is_compliant: bool,
        pub missing_consents: Vec<DataProcessingPurpose>,
        pub expired_consents: Vec<uuid::Uuid>,
        pub required_actions: Vec<String>,
        pub retention_violations: Vec<String>,
    }

    impl ConsentRecord {
        pub fn new(
            user_id: uuid::Uuid,
            purpose: DataProcessingPurpose,
            legal_basis: LegalBasis,
            consent_text: String,
            consent_version: String,
        ) -> Self {
            Self {
                consent_id: uuid::Uuid::new_v4(),
                user_id,
                purpose,
                legal_basis,
                granted_at: Utc::now(),
                expires_at: None,
                withdrawn_at: None,
                consent_text,
                consent_version,
                ip_address: None,
                user_agent: None,
            }
        }

        pub fn with_expiry(mut self, expires_at: DateTime<Utc>) -> Self {
            self.expires_at = Some(expires_at);
            self
        }

        pub fn with_request_info(mut self, ip: String, user_agent: String) -> Self {
            self.ip_address = Some(ip);
            self.user_agent = Some(user_agent);
            self
        }

        pub fn is_valid(&self) -> bool {
            self.withdrawn_at.is_none() &&
            self.expires_at.map_or(true, |exp| exp > Utc::now())
        }

        pub fn withdraw(&mut self) {
            self.withdrawn_at = Some(Utc::now());
        }
    }

    /// Check GDPR compliance for data processing
    pub fn check_compliance(
        user_consents: &[ConsentRecord],
        required_purposes: &[DataProcessingPurpose],
    ) -> ComplianceStatus {
        let valid_consents: Vec<&ConsentRecord> = user_consents
            .iter()
            .filter(|consent| consent.is_valid())
            .collect();

        let granted_purposes: Vec<&DataProcessingPurpose> = valid_consents
            .iter()
            .map(|consent| &consent.purpose)
            .collect();

        let missing_consents: Vec<DataProcessingPurpose> = required_purposes
            .iter()
            .filter(|&purpose| !granted_purposes.contains(&purpose))
            .cloned()
            .collect();

        let expired_consents: Vec<uuid::Uuid> = user_consents
            .iter()
            .filter(|consent| !consent.is_valid() && consent.withdrawn_at.is_none())
            .map(|consent| consent.consent_id)
            .collect();

        let mut required_actions = Vec::new();
        if !missing_consents.is_empty() {
            required_actions.push("Obtain missing consents".to_string());
        }
        if !expired_consents.is_empty() {
            required_actions.push("Renew expired consents".to_string());
        }

        ComplianceStatus {
            is_compliant: missing_consents.is_empty() && expired_consents.is_empty(),
            missing_consents,
            expired_consents,
            required_actions,
            retention_violations: vec![], // TODO: Implement retention policy checks
        }
    }

    /// Generate data export for GDPR Article 15 (Right to Access)
    pub fn generate_data_export(user_id: uuid::Uuid, user_data: serde_json::Value) -> SharedResult<String> {
        let export_data = serde_json::json!({
            "export_id": uuid::Uuid::new_v4(),
            "user_id": user_id,
            "export_date": Utc::now(),
            "data_categories": {
                "personal_data": user_data,
                "consent_records": [],
                "processing_activities": [],
                "data_retention": []
            },
            "your_rights": {
                "access": "You have the right to access your personal data",
                "rectification": "You have the right to correct inaccurate data",
                "erasure": "You have the right to request deletion of your data",
                "portability": "You have the right to receive your data in a portable format",
                "restriction": "You have the right to restrict processing of your data",
                "objection": "You have the right to object to processing of your data",
                "withdraw_consent": "You have the right to withdraw consent at any time"
            }
        });

        serde_json::to_string_pretty(&export_data)
            .map_err(|e| SharedError::SerializationError(e.to_string()))
    }
}

/// Data retention and lifecycle management
pub mod retention {
    use super::*;
    use chrono::{DateTime, Utc, Duration};

    /// Data retention policy
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RetentionPolicy {
        pub policy_id: uuid::Uuid,
        pub data_category: String,
        pub retention_period: Duration,
        pub legal_basis: String,
        pub deletion_method: DeletionMethod,
        pub exceptions: Vec<String>,
    }

    /// Methods for data deletion
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DeletionMethod {
        SoftDelete,
        HardDelete,
        Anonymization,
        Archival,
    }

    /// Data lifecycle status
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LifecycleStatus {
        pub created_at: DateTime<Utc>,
        pub last_accessed: Option<DateTime<Utc>>,
        pub retention_until: DateTime<Utc>,
        pub scheduled_deletion: Option<DateTime<Utc>>,
        pub status: DataStatus,
    }

    /// Current status of data in lifecycle
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DataStatus {
        Active,
        Archived,
        ScheduledForDeletion,
        Deleted,
        Anonymized,
    }

    impl RetentionPolicy {
        pub fn new(
            data_category: String,
            retention_years: i64,
            legal_basis: String,
            deletion_method: DeletionMethod,
        ) -> Self {
            Self {
                policy_id: uuid::Uuid::new_v4(),
                data_category,
                retention_period: Duration::days(365 * retention_years),
                legal_basis,
                deletion_method,
                exceptions: vec![],
            }
        }

        pub fn calculate_deletion_date(&self, created_at: DateTime<Utc>) -> DateTime<Utc> {
            created_at + self.retention_period
        }
    }

    /// Healthcare-specific retention policies
    pub fn get_healthcare_retention_policies() -> Vec<RetentionPolicy> {
        vec![
            RetentionPolicy::new(
                "medical_records".to_string(),
                7, // 7 years for medical records
                "Legal requirement for medical record retention".to_string(),
                DeletionMethod::Archival,
            ),
            RetentionPolicy::new(
                "audit_logs".to_string(),
                6, // 6 years for HIPAA audit logs
                "HIPAA compliance requirement".to_string(),
                DeletionMethod::Archival,
            ),
            RetentionPolicy::new(
                "consent_records".to_string(),
                3, // 3 years for consent records
                "GDPR compliance requirement".to_string(),
                DeletionMethod::SoftDelete,
            ),
            RetentionPolicy::new(
                "billing_records".to_string(),
                7, // 7 years for billing
                "Financial record retention requirement".to_string(),
                DeletionMethod::Archival,
            ),
            RetentionPolicy::new(
                "session_logs".to_string(),
                1, // 1 year for session logs
                "Security monitoring requirement".to_string(),
                DeletionMethod::HardDelete,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_classification() {
        let text = "Patient John Doe, DOB: 01/15/1980, Phone: 555-123-4567, Email: john@example.com";
        let classification = hipaa::classify_phi(text);
        
        assert!(classification.contains_phi);
        assert!(classification.phi_types.contains(&hipaa::PhiType::Email));
        assert!(classification.phi_types.contains(&hipaa::PhiType::PhoneNumber));
        assert!(classification.phi_types.contains(&hipaa::PhiType::DateOfBirth));
    }

    #[test]
    fn test_minimum_necessary_access() {
        let result = hipaa::validate_minimum_necessary_access(
            "physician",
            &["medical_history", "lab_results"],
            "diagnosis"
        );
        assert!(result.is_ok());

        let result = hipaa::validate_minimum_necessary_access(
            "technician",
            &["medical_history", "lab_results"],
            "lab_work"
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_consent_record() {
        let mut consent = gdpr::ConsentRecord::new(
            uuid::Uuid::new_v4(),
            gdpr::DataProcessingPurpose::Healthcare,
            gdpr::LegalBasis::Consent,
            "I consent to processing".to_string(),
            "v1.0".to_string(),
        );

        assert!(consent.is_valid());
        
        consent.withdraw();
        assert!(!consent.is_valid());
    }

    #[test]
    fn test_gdpr_compliance_check() {
        let user_id = uuid::Uuid::new_v4();
        let consents = vec![
            gdpr::ConsentRecord::new(
                user_id,
                gdpr::DataProcessingPurpose::Healthcare,
                gdpr::LegalBasis::Consent,
                "Healthcare consent".to_string(),
                "v1.0".to_string(),
            )
        ];

        let required = vec![
            gdpr::DataProcessingPurpose::Healthcare,
            gdpr::DataProcessingPurpose::Analytics,
        ];

        let status = gdpr::check_compliance(&consents, &required);
        assert!(!status.is_compliant);
        assert_eq!(status.missing_consents.len(), 1);
    }
}
