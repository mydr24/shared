// MyDR24 Healthcare Platform - Simplified Location Tracker
// Provider location tracking and emergency alert reception

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use web_sys::{console, Geolocation, Position, PositionError, PositionOptions};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use gloo_timers::callback::Interval;
use crate::websocket_simple::{
    SimpleWebSocketClient, LocationUpdate, EmergencyAlert, MessageType, 
    create_location_update
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderStatus {
    Available,
    EnRoute,
    Arrived,
    Busy,
    Offline,
}

impl ProviderStatus {
    pub fn to_string(&self) -> String {
        match self {
            ProviderStatus::Available => "available".to_string(),
            ProviderStatus::EnRoute => "en_route".to_string(),
            ProviderStatus::Arrived => "arrived".to_string(),
            ProviderStatus::Busy => "busy".to_string(),
            ProviderStatus::Offline => "offline".to_string(),
        }
    }
    
    pub fn get_color(&self) -> &'static str {
        match self {
            ProviderStatus::Available => "#00AA00",   // Green
            ProviderStatus::EnRoute => "#0066CC",     // Blue
            ProviderStatus::Arrived => "#9966CC",     // Purple
            ProviderStatus::Busy => "#FF6600",        // Orange
            ProviderStatus::Offline => "#666666",     // Gray
        }
    }
}

pub struct SimpleLocationTracker {
    pub provider_id: String,
    pub current_status: ProviderStatus,
    pub current_location: Option<(f64, f64)>,
    pub websocket_client: Option<SimpleWebSocketClient>,
    pub tracking_active: bool,
    pub location_interval: Option<Interval>,
    pub emergency_alerts: Vec<EmergencyAlert>,
    pub location_history: Vec<LocationUpdate>,
}

impl SimpleLocationTracker {
    pub fn new(provider_id: String) -> Self {
        Self {
            provider_id,
            current_status: ProviderStatus::Offline,
            current_location: None,
            websocket_client: None,
            tracking_active: false,
            location_interval: None,
            emergency_alerts: Vec::new(),
            location_history: Vec::new(),
        }
    }
    
    pub fn set_websocket_client(&mut self, client: SimpleWebSocketClient) {
        // Register for emergency alerts
        client.on_message(MessageType::EmergencyAlert, {
            let provider_id = self.provider_id.clone();
            move |message| {
                console::log_1(&"🚨 EMERGENCY ALERT RECEIVED".into());
                if let Ok(alert) = serde_json::from_value::<EmergencyAlert>(message.payload) {
                    console::log_1(&format!("Emergency from patient: {}", alert.patient_id).into());
                    
                    // Show emergency notification
                    if let Some(window) = web_sys::window() {
                        let alert_msg = format!(
                            "🚨 EMERGENCY ALERT\n\nPatient: {}\nLocation: {:.4}, {:.4}\nCondition: {}\nContact: {}",
                            alert.patient_id,
                            alert.location.latitude, alert.location.longitude,
                            alert.medical_condition.as_deref().unwrap_or("Unknown"),
                            alert.emergency_contact.as_deref().unwrap_or("None provided")
                        );
                        let _ = window.alert_with_message(&alert_msg);
                    }
                }
            }
        });
        
        self.websocket_client = Some(client);
    }
    
    pub fn set_status(&mut self, status: ProviderStatus) {
        self.current_status = status;
        console::log_1(&format!("Provider status updated: {:?}", self.current_status).into());
    }
    
    // Start location tracking
    pub async fn start_tracking(&mut self) -> Result<(), String> {
        if self.tracking_active {
            return Ok(());
        }
        
        console::log_1(&"Starting location tracking...".into());
        
        // Set status to available
        self.set_status(ProviderStatus::Available);
        
        // Start periodic location updates
        let provider_id = self.provider_id.clone();
        let ws_client = self.websocket_client.as_ref()
            .ok_or("WebSocket client not set")?;
        
        // Get initial location
        self.update_location().await?;
        
        // Set up interval for location updates (every 30 seconds)
        let interval = Interval::new(30000, move || {
            console::log_1(&"Sending location update...".into());
            // Note: In a real implementation, you'd need to handle this differently
            // as we can't easily share mutable state across the interval closure
        });
        
        self.location_interval = Some(interval);
        self.tracking_active = true;
        
        Ok(())
    }
    
    // Stop location tracking
    pub fn stop_tracking(&mut self) {
        if !self.tracking_active {
            return;
        }
        
        console::log_1(&"Stopping location tracking...".into());
        
        // Clear interval
        if let Some(interval) = self.location_interval.take() {
            drop(interval);
        }
        
        self.set_status(ProviderStatus::Offline);
        self.tracking_active = false;
    }
    
    // Update current location
    pub async fn update_location(&mut self) -> Result<(), String> {
        let location = self.get_current_location().await?;
        self.current_location = Some(location);
        
        // Send location update via WebSocket
        if let Some(client) = &self.websocket_client {
            let location_update = create_location_update(
                self.provider_id.clone(),
                location.0,
                location.1,
                10.0, // accuracy in meters
                self.current_status.to_string(),
            );
            
            // Store in history
            self.location_history.push(location_update.clone());
            
            // Send to server
            client.send_location_update(location_update).await?;
        }
        
        Ok(())
    }
    
    // Get current location using Web Geolocation API
    async fn get_current_location(&self) -> Result<(f64, f64), String> {
        let window = web_sys::window().ok_or("No window object")?;
        let geolocation = window.navigator().geolocation()
            .map_err(|_| "Geolocation not supported")?;
        
        let options = PositionOptions::new();
        options.set_enable_high_accuracy(true);
        options.set_timeout(10000); // 10 seconds
        options.set_maximum_age(60000); // 1 minute
        
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
        
        Ok((lat, lng))
    }
    
    // Respond to emergency alert
    pub async fn respond_to_emergency(&mut self, alert_id: String, response_type: String) -> Result<(), String> {
        console::log_1(&format!("Responding to emergency: {} with {}", alert_id, response_type).into());
        
        // Update status based on response
        match response_type.as_str() {
            "acknowledged" => {
                console::log_1(&"Emergency acknowledged".into());
            }
            "en_route" => {
                self.set_status(ProviderStatus::EnRoute);
                console::log_1(&"Provider is en route to emergency".into());
            }
            "arrived" => {
                self.set_status(ProviderStatus::Arrived);
                console::log_1(&"Provider has arrived at emergency location".into());
            }
            "completed" => {
                self.set_status(ProviderStatus::Available);
                console::log_1(&"Emergency response completed".into());
            }
            _ => {
                console::log_1(&"Unknown response type".into());
            }
        }
        
        // Update location
        self.update_location().await?;
        
        Ok(())
    }
    
    // Get distance to a location (simplified calculation)
    pub fn calculate_distance_to(&self, target_lat: f64, target_lng: f64) -> Option<f64> {
        if let Some((current_lat, current_lng)) = self.current_location {
            // Simplified distance calculation (not accurate for long distances)
            let lat_diff = target_lat - current_lat;
            let lng_diff = target_lng - current_lng;
            let distance = ((lat_diff * lat_diff) + (lng_diff * lng_diff)).sqrt() * 111000.0; // Rough conversion to meters
            Some(distance)
        } else {
            None
        }
    }
    
    // Get estimated time to location (simplified)
    pub fn estimate_arrival_time(&self, target_lat: f64, target_lng: f64) -> Option<u32> {
        if let Some(distance) = self.calculate_distance_to(target_lat, target_lng) {
            // Assume average speed of 40 km/h in city
            let time_hours = distance / 40000.0;
            let time_minutes = (time_hours * 60.0) as u32;
            Some(time_minutes)
        } else {
            None
        }
    }
    
    // Get location tracking status for UI
    pub fn get_tracking_status(&self) -> String {
        if self.tracking_active {
            format!("🟢 Tracking Active - Status: {:?}", self.current_status)
        } else {
            "🔴 Tracking Inactive".to_string()
        }
    }
    
    // Get recent location history
    pub fn get_recent_locations(&self, limit: usize) -> Vec<LocationUpdate> {
        let mut recent = self.location_history.clone();
        recent.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        recent.truncate(limit);
        recent
    }
    
    // Clear old location history
    pub fn cleanup_old_locations(&mut self, hours: i64) {
        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        self.location_history.retain(|loc| loc.timestamp > cutoff);
    }
}

// Booking notification handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingNotification {
    pub booking_id: String,
    pub patient_id: String,
    pub patient_location: Option<(f64, f64)>,
    pub service_type: String,
    pub urgency: String,
    pub notes: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl BookingNotification {
    pub fn new(
        booking_id: String,
        patient_id: String,
        patient_location: Option<(f64, f64)>,
        service_type: String,
        urgency: String,
    ) -> Self {
        Self {
            booking_id,
            patient_id,
            patient_location,
            service_type,
            urgency,
            notes: None,
            timestamp: Utc::now(),
        }
    }
    
    pub fn get_urgency_color(&self) -> &'static str {
        match self.urgency.to_lowercase().as_str() {
            "emergency" => "#FF0000",
            "urgent" => "#FF6600",
            "normal" => "#00AA00",
            "low" => "#666666",
            _ => "#000000",
        }
    }
}

// Provider notification system
pub struct ProviderNotificationManager {
    pub provider_id: String,
    pub notifications: Vec<BookingNotification>,
    pub emergency_alerts: Vec<EmergencyAlert>,
}

impl ProviderNotificationManager {
    pub fn new(provider_id: String) -> Self {
        Self {
            provider_id,
            notifications: Vec::new(),
            emergency_alerts: Vec::new(),
        }
    }
    
    pub fn add_booking_notification(&mut self, notification: BookingNotification) {
        self.notifications.push(notification);
        console::log_1(&"New booking notification received".into());
    }
    
    pub fn add_emergency_alert(&mut self, alert: EmergencyAlert) {
        self.emergency_alerts.push(alert);
        console::log_1(&"🚨 New emergency alert received".into());
    }
    
    pub fn get_unread_count(&self) -> usize {
        self.notifications.len() + self.emergency_alerts.len()
    }
    
    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
        console::log_1(&"Notifications cleared".into());
    }
    
    pub fn clear_emergency_alerts(&mut self) {
        self.emergency_alerts.clear();
        console::log_1(&"Emergency alerts cleared".into());
    }
}
