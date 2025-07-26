use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Patient {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 10, max = 15))]
    pub phone: Option<String>,
    pub date_of_birth: chrono::NaiveDate,
    pub gender: Gender,
    pub address: Address,
    pub medical_record_number: String,
    pub emergency_contact: EmergencyContact,
    pub insurance_info: Option<InsuranceInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Provider {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 10, max = 15))]
    pub phone: String,
    pub specialization: MedicalSpecialization,
    pub license_number: String,
    pub nmc_registration: String,
    pub qualification: String,
    pub experience_years: u32,
    pub availability_schedule: AvailabilitySchedule,
    pub consultation_fee: ConsultationFee,
    pub rating: Option<f32>,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Appointment {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub provider_id: Uuid,
    pub appointment_type: AppointmentType,
    pub scheduled_time: DateTime<Utc>,
    pub duration_minutes: i32,
    pub status: AppointmentStatus,
    pub consultation_notes: Option<String>,
    pub prescription: Option<Prescription>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Address {
    #[validate(length(min = 1, max = 200))]
    pub street: String,
    #[validate(length(min = 1, max = 100))]
    pub city: String,
    #[validate(length(min = 1, max = 100))]
    pub state: String,
    #[validate(length(min = 5, max = 10))]
    pub postal_code: String,
    #[validate(length(min = 1, max = 100))]
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EmergencyContact {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub relationship: String,
    #[validate(length(min = 10, max = 15))]
    pub phone: String,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceInfo {
    pub provider_name: String,
    pub policy_number: String,
    pub group_number: Option<String>,
    pub coverage_type: CoverageType,
    pub expiry_date: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilitySchedule {
    pub monday: Option<TimeSlot>,
    pub tuesday: Option<TimeSlot>,
    pub wednesday: Option<TimeSlot>,
    pub thursday: Option<TimeSlot>,
    pub friday: Option<TimeSlot>,
    pub saturday: Option<TimeSlot>,
    pub sunday: Option<TimeSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start_time: chrono::NaiveTime,
    pub end_time: chrono::NaiveTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationFee {
    pub base_fee: f64,
    pub currency: String,
    pub emergency_multiplier: f64,
    pub follow_up_discount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Prescription {
    pub medications: Vec<Medication>,
    pub instructions: String,
    pub follow_up_date: Option<chrono::NaiveDate>,
    pub digital_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Medication {
    pub name: String,
    pub dosage: String,
    pub frequency: String,
    pub duration: String,
    pub instructions: Option<String>,
}

// Enums
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MedicalSpecialization {
    GeneralMedicine,
    Cardiology,
    Dermatology,
    Neurology,
    Orthopedics,
    Pediatrics,
    Psychiatry,
    Radiology,
    Surgery,
    Gynecology,
    Ophthalmology,
    ENT,
    Dentistry,
    Physiotherapy,
    Nursing,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppointmentType {
    InPerson,
    Telemedicine,
    HomeVisit,
    Emergency,
    FollowUp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppointmentStatus {
    Scheduled,
    Confirmed,
    InProgress,
    Completed,
    Cancelled,
    NoShow,
    Rescheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Rejected,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoverageType {
    Basic,
    Premium,
    Corporate,
    Government,
}
