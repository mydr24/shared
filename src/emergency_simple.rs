// MyDR24 Healthcare Platform - Simplified Emergency System
// Patient emergency button with location services

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use web_sys::{console, Geolocation, Position, PositionError, PositionOptions};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use crate::websocket_simple::{SimpleWebSocketClient, EmergencyAlert, create_emergency_alert};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyContact {
    pub name: String,
    pub phone: String,
    pub relationship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalInfo {
    pub condition: String,
    pub medications: Vec<String>,
    pub allergies: Vec<String>,
    pub blood_type: Option<String>,
}

pub struct SimpleEmergencySystem {
    pub patient_id: String,
    pub emergency_contacts: Vec<EmergencyContact>,
    pub medical_info: Option<MedicalInfo>,
    pub websocket_client: Option<SimpleWebSocketClient>,
    pub last_location: Option<(f64, f64)>,
}

impl SimpleEmergencySystem {
    pub fn new(patient_id: String) -> Self {
        Self {
            patient_id,
            emergency_contacts: Vec::new(),
            medical_info: None,
            websocket_client: None,
            last_location: None,
        }
    }
    
    pub fn set_websocket_client(&mut self, client: SimpleWebSocketClient) {
        self.websocket_client = Some(client);
    }
    
    pub fn add_emergency_contact(&mut self, contact: EmergencyContact) {
        self.emergency_contacts.push(contact);
    }
    
    pub fn set_medical_info(&mut self, info: MedicalInfo) {
        self.medical_info = Some(info);
    }
    
    // Get current location using Web Geolocation API
    pub async fn get_current_location(&mut self) -> Result<(f64, f64), String> {
        let window = web_sys::window().ok_or("No window object")?;
        let geolocation = window.navigator().geolocation()
            .map_err(|_| "Geolocation not supported")?;
        
        let options = PositionOptions::new();
        options.set_enable_high_accuracy(true);
        options.set_timeout(10000); // 10 seconds
        options.set_maximum_age(300000); // 5 minutes
        
        // Create a promise for getting position
        let promise = Promise::new(&mut |resolve, reject| {
            let success_callback = Closure::wrap(Box::new(move |position: Position| {
                let coords = position.coords();
                let lat = coords.latitude();
                let lng = coords.longitude();
                resolve.call2(&JsValue::NULL, &JsValue::from(lat), &JsValue::from(lng)).unwrap();
            }) as Box<dyn FnMut(Position)>);
            
            let error_callback = Closure::wrap(Box::new(move |error: PositionError| {
                reject.call1(&JsValue::NULL, &JsValue::from(error.message())).unwrap();
            }) as Box<dyn FnMut(PositionError)>);
            
            geolocation.get_current_position_with_error_callback_and_options(
                success_callback.as_ref().unchecked_ref(),
                Some(error_callback.as_ref().unchecked_ref()),
                &options,
            ).unwrap();
            
            success_callback.forget();
            error_callback.forget();
        });
        
        // Convert to Rust future and await
        let js_result = JsFuture::from(promise).await
            .map_err(|e| format!("Geolocation error: {:?}", e))?;
        
        // Extract coordinates from the result
        let lat = js_result.as_f64().unwrap_or(0.0);
        let lng = js_result.as_f64().unwrap_or(0.0);
        
        self.last_location = Some((lat, lng));
        Ok((lat, lng))
    }
    
    // Trigger emergency alert
    pub async fn trigger_emergency(&mut self) -> Result<(), String> {
        console::log_1(&"🚨 EMERGENCY ALERT TRIGGERED".into());
        
        // Get current location
        let location = match self.get_current_location().await {
            Ok(coords) => Some(coords),
            Err(e) => {
                console::log_1(&format!("Could not get location: {}", e).into());
                self.last_location
            }
        };
        
        // Prepare medical condition info
        let medical_condition = self.medical_info.as_ref()
            .map(|info| format!("Condition: {}, Blood Type: {}, Allergies: {}", 
                info.condition,
                info.blood_type.as_deref().unwrap_or("Unknown"),
                info.allergies.join(", ")
            ));
        
        // Prepare emergency contact info
        let emergency_contact = self.emergency_contacts.first()
            .map(|contact| format!("{}: {}", contact.name, contact.phone));
        
        // Create emergency alert
        let alert = create_emergency_alert(
            self.patient_id.clone(),
            location,
            medical_condition,
            emergency_contact,
        );
        
        // Send via WebSocket
        if let Some(client) = &self.websocket_client {
            client.send_emergency_alert(alert).await?;
            console::log_1(&"Emergency alert sent via WebSocket".into());
        } else {
            console::log_1(&"No WebSocket client available".into());
        }
        
        // Also try to call emergency services if possible
        self.call_emergency_services().await?;
        
        Ok(())
    }
    
    // Attempt to call emergency services
    async fn call_emergency_services(&self) -> Result<(), String> {
        // In a real app, this would integrate with the phone's dialer
        // For web, we can only show instructions
        
        let window = web_sys::window().ok_or("No window object")?;
        
        // Show emergency instructions
        let alert_message = "EMERGENCY ALERT SENT!\n\n\
            1. Healthcare providers have been notified\n\
            2. If this is a life-threatening emergency, call 108 immediately\n\
            3. Stay calm and wait for help\n\
            4. Keep your phone nearby for updates";
        
        window.alert_with_message(alert_message)
            .map_err(|_| "Could not show alert")?;
        
        Ok(())
    }
    
    // Cancel emergency alert (false alarm)
    pub async fn cancel_emergency(&self) -> Result<(), String> {
        console::log_1(&"Emergency alert cancelled".into());
        
        // TODO: Send cancellation message via WebSocket
        let window = web_sys::window().ok_or("No window object")?;
        window.alert_with_message("Emergency alert has been cancelled")
            .map_err(|_| "Could not show alert")?;
        
        Ok(())
    }
    
    // Get emergency status for UI
    pub fn get_emergency_button_text(&self) -> &'static str {
        if self.websocket_client.is_some() {
            "🚨 EMERGENCY"
        } else {
            "🚨 EMERGENCY (Offline)"
        }
    }
    
    // Validate emergency system setup
    pub fn validate_setup(&self) -> Result<(), String> {
        if self.patient_id.is_empty() {
            return Err("Patient ID not set".to_string());
        }
        
        if self.emergency_contacts.is_empty() {
            return Err("No emergency contacts configured".to_string());
        }
        
        if self.websocket_client.is_none() {
            return Err("WebSocket client not connected".to_string());
        }
        
        Ok(())
    }
}

// Helper functions for creating emergency data
pub fn create_emergency_contact(name: String, phone: String, relationship: String) -> EmergencyContact {
    EmergencyContact { name, phone, relationship }
}

pub fn create_medical_info(
    condition: String,
    medications: Vec<String>,
    allergies: Vec<String>,
    blood_type: Option<String>,
) -> MedicalInfo {
    MedicalInfo {
        condition,
        medications,
        allergies,
        blood_type,
    }
}

// Emergency alert priorities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyPriority {
    Critical,  // Life-threatening
    High,      // Urgent medical attention needed
    Medium,    // Medical assistance needed
    Low,       // Non-urgent but needs attention
}

impl EmergencyPriority {
    pub fn to_string(&self) -> String {
        match self {
            EmergencyPriority::Critical => "critical".to_string(),
            EmergencyPriority::High => "high".to_string(),
            EmergencyPriority::Medium => "medium".to_string(),
            EmergencyPriority::Low => "low".to_string(),
        }
    }
    
    pub fn get_color(&self) -> &'static str {
        match self {
            EmergencyPriority::Critical => "#FF0000", // Red
            EmergencyPriority::High => "#FF6600",     // Orange
            EmergencyPriority::Medium => "#FFCC00",   // Yellow
            EmergencyPriority::Low => "#00AA00",      // Green
        }
    }
}

// Emergency response tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyResponse {
    pub response_id: String,
    pub alert_id: String,
    pub responder_id: String,
    pub response_type: String, // "acknowledged", "en_route", "arrived", "completed"
    pub estimated_arrival: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl EmergencyResponse {
    pub fn new(alert_id: String, responder_id: String, response_type: String) -> Self {
        Self {
            response_id: Uuid::new_v4().to_string(),
            alert_id,
            responder_id,
            response_type,
            estimated_arrival: None,
            notes: None,
            timestamp: Utc::now(),
        }
    }
}
