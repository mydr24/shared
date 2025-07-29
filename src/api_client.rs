// MyDR24 API Client - Frontend Integration Service
// Connects Leptos frontend applications to the MyDR24 backend API

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

// API Configuration
const API_BASE_URL: &str = "http://localhost:8080";
const API_VERSION: &str = "v1";

// Common API Response Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

// Authentication Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserProfile,
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub phone: Option<String>,
    pub created_at: String,
}

// Healthcare Data Types for API Client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPatient {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub age: Option<u32>,
    pub medical_id: Option<String>,
    pub emergency_contact: Option<ApiEmergencyContact>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiProvider {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub specialization: Vec<String>,
    pub license_number: String,
    pub verification_status: String,
    pub rating: Option<f32>,
    pub active_patients: u32,
    pub location: Option<ApiLocation>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmergencyContact {
    pub name: String,
    pub phone: String,
    pub relationship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_patients: u32,
    pub active_providers: u32,
    pub total_appointments: u32,
    pub emergency_cases: u32,
    pub revenue_today: f64,
    pub system_health: String,
}

// Emergency Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmergencyRequest {
    pub patient_id: String,
    pub emergency_type: String,
    pub severity: String,
    pub location: ApiLocation,
    pub description: String,
    pub medical_history: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmergencyResponse {
    pub request_id: String,
    pub status: String,
    pub assigned_provider: Option<ApiProvider>,
    pub eta_minutes: Option<u32>,
    pub tracking_url: Option<String>,
}

// API Client Service
pub struct ApiClient {
    base_url: String,
    auth_token: Option<String>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            base_url: API_BASE_URL.to_string(),
            auth_token: None,
        }
    }

    pub fn with_auth(token: String) -> Self {
        Self {
            base_url: API_BASE_URL.to_string(),
            auth_token: Some(token),
        }
    }

    // Helper method to build request with auth headers
    fn build_request(&self, method: &str, endpoint: &str) -> gloo_net::http::RequestBuilder {
        let url = format!("{}/api/{}/{}", self.base_url, API_VERSION, endpoint);
        let mut request = match method {
            "GET" => Request::get(&url),
            "POST" => Request::post(&url),
            "PUT" => Request::put(&url),
            "DELETE" => Request::delete(&url),
            _ => Request::get(&url),
        };

        if let Some(token) = &self.auth_token {
            request = request.header("Authorization", &format!("Bearer {}", token));
        }

        request.header("Content-Type", "application/json")
    }

    // Health Check
    pub async fn health_check() -> Result<HealthCheck, String> {
        let url = format!("{}/health", API_BASE_URL);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<HealthCheck>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            Err(format!("API error: {}", response.status()))
        }
    }

    // Authentication Endpoints
    pub async fn login(&self, email: String, password: String) -> Result<LoginResponse, String> {
        let login_request = LoginRequest { email, password };
        
        let request_result = self
            .build_request("POST", "auth/login")
            .json(&login_request);
        
        let request = match request_result {
            Ok(req) => req,
            Err(e) => return Err(format!("Failed to serialize login request: {}", e)),
        };
        
        let response = request
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<LoginResponse>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Login failed: {}", error_text))
        }
    }

    pub async fn get_profile(&self) -> Result<UserProfile, String> {
        let response = self
            .build_request("GET", "auth/profile")
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<UserProfile>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            Err(format!("Failed to get profile: {}", response.status()))
        }
    }

    // Dashboard Endpoints
    pub async fn get_dashboard_stats(&self) -> Result<DashboardStats, String> {
        let response = self
            .build_request("GET", "dashboard/stats")
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<DashboardStats>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            Err(format!("Failed to get dashboard stats: {}", response.status()))
        }
    }

    // Patient Endpoints
    pub async fn get_patients(&self) -> Result<Vec<ApiPatient>, String> {
        let response = self
            .build_request("GET", "patients")
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<Vec<ApiPatient>>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            Err(format!("Failed to get patients: {}", response.status()))
        }
    }

    pub async fn get_patient(&self, patient_id: &str) -> Result<ApiPatient, String> {
        let endpoint = format!("patients/{}", patient_id);
        let response = self
            .build_request("GET", &endpoint)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<ApiPatient>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            Err(format!("Failed to get patient: {}", response.status()))
        }
    }

    // Provider Endpoints
    pub async fn get_providers(&self) -> Result<Vec<ApiProvider>, String> {
        let response = self
            .build_request("GET", "providers")
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<Vec<ApiProvider>>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            Err(format!("Failed to get providers: {}", response.status()))
        }
    }

    pub async fn get_provider(&self, provider_id: &str) -> Result<ApiProvider, String> {
        let endpoint = format!("providers/{}", provider_id);
        let response = self
            .build_request("GET", &endpoint)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<ApiProvider>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            Err(format!("Failed to get provider: {}", response.status()))
        }
    }

    // Emergency Endpoints
    pub async fn create_emergency_request(&self, request: ApiEmergencyRequest) -> Result<ApiEmergencyResponse, String> {
        let request_result = self
            .build_request("POST", "emergency/request")
            .json(&request);
        
        let request_body = match request_result {
            Ok(req) => req,
            Err(e) => return Err(format!("Failed to serialize emergency request: {}", e)),
        };
        
        let response = request_body
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json::<ApiEmergencyResponse>()
                .await
                .map_err(|e| format!("Parse error: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Emergency request failed: {}", error_text))
        }
    }
}

// Reactive API Hooks for Leptos
pub fn use_api_client() -> ApiClient {
    // In a real application, you might want to get the auth token from local storage
    // or from a global auth context
    ApiClient::new()
}

pub fn use_authenticated_api_client() -> Option<ApiClient> {
    // This would typically read from local storage or auth context
    // For now, returning None - implement based on your auth strategy
    None
}

// Helper function to handle async API calls in Leptos components
pub fn spawn_api_call<F, T, E>(
    future: F,
    on_success: impl Fn(T) + 'static,
    on_error: impl Fn(E) + 'static,
) where
    F: std::future::Future<Output = Result<T, E>> + 'static,
    T: 'static,
    E: 'static,
{
    spawn_local(async move {
        match future.await {
            Ok(data) => on_success(data),
            Err(error) => on_error(error),
        }
    });
}
