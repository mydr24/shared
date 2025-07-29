use leptos::prelude::*;
use crate::ui::{Icon, IconSize, Button, Variant, Priority, HealthcareStatus};

#[                            {alerts.into_iter().map(|alert| view! {
                                <EmergencyAlertCard 
                                    alert=alert.clone()
                                    on_accept=Box::new(move |id| {
                                        if let Some(callback) = &on_accept {
                                            callback(id);
                                        }
                                    })
                                    on_decline=Box::new(move |id| {
                                        if let Some(callback) = &on_decline {
                                            callback(id);
                                        }
                                    })
                                    show_patient_info=show_patient_info
                                />
                            }).collect_view()}one)]
pub struct EmergencyAlert {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub location: Option<String>,
    pub patient_info: Option<PatientInfo>,
    pub response_time: Option<String>,
    pub status: EmergencyStatus,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct PatientInfo {
    pub name: String,
    pub age: u8,
    pub medical_id: String,
    pub blood_type: Option<String>,
    pub allergies: Vec<String>,
    pub emergency_contact: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmergencyStatus {
    Pending,
    Accepted,
    EnRoute,
    OnScene,
    Resolved,
    Cancelled,
}

impl EmergencyStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmergencyStatus::Pending => "pending",
            EmergencyStatus::Accepted => "accepted",
            EmergencyStatus::EnRoute => "en-route",
            EmergencyStatus::OnScene => "on-scene",
            EmergencyStatus::Resolved => "resolved",
            EmergencyStatus::Cancelled => "cancelled",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            EmergencyStatus::Pending => "hsl(48 96% 53%)",     // Yellow
            EmergencyStatus::Accepted => "hsl(210 100% 50%)",  // Blue
            EmergencyStatus::EnRoute => "hsl(280 65% 60%)",    // Purple
            EmergencyStatus::OnScene => "hsl(25 95% 55%)",     // Orange
            EmergencyStatus::Resolved => "hsl(142 76% 36%)",   // Green
            EmergencyStatus::Cancelled => "hsl(0 0% 50%)",     // Gray
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            EmergencyStatus::Pending => "interface-clock",
            EmergencyStatus::Accepted => "interface-check",
            EmergencyStatus::EnRoute => "transport-car",
            EmergencyStatus::OnScene => "location-pin",
            EmergencyStatus::Resolved => "interface-check-circle",
            EmergencyStatus::Cancelled => "interface-close",
        }
    }
}

#[component]
pub fn EmergencyAlertPanel(
    #[prop()] alerts: Vec<EmergencyAlert>,
    #[prop(optional)] on_accept: Option<Box<dyn Fn(String)>>,
    #[prop(optional)] on_decline: Option<Box<dyn Fn(String)>>,
    #[prop(optional)] show_patient_info: Option<bool>,
) -> impl IntoView {
    let show_patient_info = show_patient_info.unwrap_or(true);
    
    view! {
        <div class="space-y-4">
            <div class="flex items-center justify-between">
                <h2 class="text-xl font-bold text-slate-900 flex items-center">
                    <Icon name="emergency-alert".to_string() size=IconSize::Lg class="text-red-600 mr-3 animate-pulse".to_string() />
                    Emergency Alerts
                </h2>
                <div class="flex items-center space-x-2">
                    <div class="w-3 h-3 bg-red-500 rounded-full animate-pulse"></div>
                    <span class="text-sm font-medium text-slate-600">{alerts.len()} Active</span>
                </div>
            </div>
            
            {
                if alerts.is_empty() {
                    view! {
                        <div class="bg-green-50 border border-green-200 rounded-xl p-6 text-center">
                            <Icon name="interface-check-circle".to_string() size=IconSize::Xl class="text-green-600 mx-auto mb-3".to_string() />
                            <h3 class="text-lg font-semibold text-green-800 mb-2">No Active Emergencies</h3>
                            <p class="text-green-600">All emergency situations are currently resolved.</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-4">
                            {alerts.into_iter().map(|alert| view! {
                                <EmergencyAlertCard 
                                    alert=alert.clone()
                                    on_accept=move |id| {
                                        if let Some(callback) = &on_accept {
                                            callback(id);
                                        }
                                    }
                                    on_decline=move |id| {
                                        if let Some(callback) = &on_decline {
                                            callback(id);
                                        }
                                    }
                                    show_patient_info=show_patient_info
                                />
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
            }
        </div>
    }
}

#[component]
pub fn EmergencyAlertCard(
    #[prop()] alert: EmergencyAlert,
    #[prop(optional)] on_accept: Option<Box<dyn Fn(String)>>,
    #[prop(optional)] on_decline: Option<Box<dyn Fn(String)>>,
    #[prop(optional)] show_patient_info: Option<bool>,
) -> impl IntoView {
    let show_patient_info = show_patient_info.unwrap_or(true);
    let alert_id = alert.id.clone();
    let alert_id_accept = alert.id.clone();
    let alert_id_decline = alert.id.clone();
    
    let priority_bg = match alert.priority {
        Priority::Emergency => "bg-gradient-to-r from-red-50 to-red-100 border-red-300",
        Priority::Urgent => "bg-gradient-to-r from-orange-50 to-orange-100 border-orange-300",
        Priority::Normal => "bg-gradient-to-r from-blue-50 to-blue-100 border-blue-300",
        Priority::Low => "bg-gradient-to-r from-green-50 to-green-100 border-green-300",
    };
    
    let priority_text = match alert.priority {
        Priority::Emergency => "text-red-800",
        Priority::Urgent => "text-orange-800",
        Priority::Normal => "text-blue-800",
        Priority::Low => "text-green-800",
    };
    
    view! {
        <div class=format!("rounded-xl border-2 p-6 shadow-lg hover:shadow-xl transition-all duration-200 {}", priority_bg)>
            // Alert Header
            <div class="flex items-start justify-between mb-4">
                <div class="flex items-start space-x-3">
                    <div class=format!("p-3 rounded-full {} shadow-sm", 
                        match alert.priority {
                            Priority::Emergency => "bg-red-600",
                            Priority::Urgent => "bg-orange-600",
                            Priority::Normal => "bg-blue-600",
                            Priority::Low => "bg-green-600",
                        }
                    )>
                        <Icon name="emergency-alert".to_string() size=IconSize::Lg class="text-white".to_string() />
                    </div>
                    
                    <div class="flex-1">
                        <div class="flex items-center space-x-2 mb-1">
                            <h3 class=format!("text-lg font-bold {}", priority_text)>{alert.title.clone()}</h3>
                            <span class=format!("px-3 py-1 text-xs font-semibold rounded-full shadow-sm {}",
                                match alert.priority {
                                    Priority::Emergency => "bg-red-600 text-white",
                                    Priority::Urgent => "bg-orange-600 text-white",
                                    Priority::Normal => "bg-blue-600 text-white",
                                    Priority::Low => "bg-green-600 text-white",
                                }
                            )>
                                {alert.priority.as_str().to_uppercase()}
                            </span>
                        </div>
                        <p class=format!("text-sm {}", priority_text)>{alert.description.clone()}</p>
                    </div>
                </div>
                
                // Status Badge
                <div class="flex flex-col items-end space-y-2">
                    <div 
                        class="flex items-center space-x-2 px-3 py-1 rounded-full text-xs font-semibold border shadow-sm"
                        style=format!("background-color: {}; color: white;", alert.status.color())
                    >
                        <Icon name=alert.status.icon().to_string() size=IconSize::Xs class="text-white".to_string() />
                        <span>{alert.status.as_str().replace("-", " ").to_uppercase()}</span>
                    </div>
                    <div class=format!("text-xs {}", priority_text)>
                        {alert.created_at.clone()}
                    </div>
                </div>
            </div>
            
            // Location and Response Time
            <div class="flex items-center space-x-6 mb-4">
                {
                    if let Some(location) = &alert.location {
                        view! {
                            <div class="flex items-center space-x-2">
                                <Icon name="location-pin".to_string() size=IconSize::Sm class=format!("text-slate-600") />
                                <span class="text-sm font-medium text-slate-700">{location.clone()}</span>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
                
                {
                    if let Some(response_time) = &alert.response_time {
                        view! {
                            <div class="flex items-center space-x-2">
                                <Icon name="interface-clock".to_string() size=IconSize::Sm class="text-slate-600".to_string() />
                                <span class="text-sm font-medium text-slate-700">ETA: {response_time.clone()}</span>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
            </div>
            
            // Patient Information
            {
                if show_patient_info {
                    if let Some(patient) = &alert.patient_info {
                        view! {
                            <div class="bg-white bg-opacity-70 rounded-lg p-4 mb-4 border border-white border-opacity-50">
                                <h4 class="font-semibold text-slate-800 mb-3 flex items-center">
                                    <Icon name="user-patient".to_string() size=IconSize::Sm class="mr-2".to_string() />
                                    Patient Information
                                </h4>
                                
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-3 text-sm">
                                    <div>
                                        <span class="font-medium text-slate-600">Name:</span>
                                        <span class="ml-2 text-slate-800">{patient.name.clone()}</span>
                                    </div>
                                    <div>
                                        <span class="font-medium text-slate-600">Age:</span>
                                        <span class="ml-2 text-slate-800">{patient.age} years</span>
                                    </div>
                                    <div>
                                        <span class="font-medium text-slate-600">Medical ID:</span>
                                        <span class="ml-2 text-slate-800 font-mono">{patient.medical_id.clone()}</span>
                                    </div>
                                    {
                                        if let Some(blood_type) = &patient.blood_type {
                                            view! {
                                                <div>
                                                    <span class="font-medium text-slate-600">Blood Type:</span>
                                                    <span class="ml-2 text-slate-800 font-semibold">{blood_type.clone()}</span>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }
                                </div>
                                
                                {
                                    if !patient.allergies.is_empty() {
                                        view! {
                                            <div class="mt-3 p-3 bg-red-50 border border-red-200 rounded-lg">
                                                <div class="flex items-center space-x-2 mb-2">
                                                    <Icon name="emergency-alert".to_string() size=IconSize::Sm class="text-red-600".to_string() />
                                                    <span class="font-semibold text-red-800">Allergies:</span>
                                                </div>
                                                <div class="text-sm text-red-700">
                                                    {patient.allergies.join(", ")}
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                                
                                {
                                    if let Some(contact) = &patient.emergency_contact {
                                        view! {
                                            <div class="mt-3">
                                                <span class="font-medium text-slate-600">Emergency Contact:</span>
                                                <a href=format!("tel:{}", contact) class="ml-2 text-blue-600 hover:text-blue-800 underline font-medium">
                                                    {contact.clone()}
                                                </a>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                } else {
                    view! {}.into_any()
                }
            }
            
            // Action Buttons
            {
                if alert.status == EmergencyStatus::Pending {
                    view! {
                        <div class="flex space-x-3">
                            <button 
                                class="flex-1 bg-gradient-to-r from-green-600 to-green-700 text-white px-6 py-3 rounded-lg font-semibold hover:from-green-700 hover:to-green-800 transition-all duration-200 shadow-md hover:shadow-lg transform hover:scale-105 active:scale-95 flex items-center justify-center"
                                on:click=move |_| {
                                    if let Some(callback) = &on_accept {
                                        callback(alert_id_accept.clone());
                                    }
                                }
                            >
                                <Icon name="interface-check".to_string() size=IconSize::Md class="text-white mr-2".to_string() />
                                Accept Emergency
                            </button>
                            
                            <button 
                                class="flex-1 bg-gradient-to-r from-gray-500 to-gray-600 text-white px-6 py-3 rounded-lg font-semibold hover:from-gray-600 hover:to-gray-700 transition-all duration-200 shadow-md hover:shadow-lg transform hover:scale-105 active:scale-95 flex items-center justify-center"
                                on:click=move |_| {
                                    if let Some(callback) = &on_decline {
                                        callback(alert_id_decline.clone());
                                    }
                                }
                            >
                                <Icon name="interface-close".to_string() size=IconSize::Md class="text-white mr-2".to_string() />
                                Cannot Respond
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="text-center py-2">
                            <span class="text-sm font-medium text-slate-600">
                                Emergency response in progress...
                            </span>
                        </div>
                    }.into_any()
                }
            }
        </div>
    }
}

#[component]
pub fn EmergencyTriggerButton(
    #[prop(optional)] on_emergency: Option<Box<dyn Fn()>>,
    #[prop(optional)] size: Option<String>,
    #[prop(optional)] mobile: Option<bool>,
) -> impl IntoView {
    let size = size.unwrap_or_else(|| "large".to_string());
    let mobile = mobile.unwrap_or(false);
    
    let button_class = if mobile {
        "w-full bg-gradient-to-r from-red-600 to-red-700 text-white px-8 py-6 rounded-2xl font-bold text-lg shadow-xl hover:from-red-700 hover:to-red-800 transition-all duration-200 transform hover:scale-105 active:scale-95 flex items-center justify-center"
    } else {
        match size.as_str() {
            "small" => "bg-red-600 text-white px-4 py-2 rounded-lg font-semibold hover:bg-red-700 transition-colors duration-200 flex items-center",
            "large" => "bg-gradient-to-r from-red-600 to-red-700 text-white px-8 py-4 rounded-xl font-bold text-lg shadow-lg hover:from-red-700 hover:to-red-800 transition-all duration-200 transform hover:scale-105 active:scale-95 flex items-center justify-center",
            _ => "bg-red-600 text-white px-6 py-3 rounded-lg font-semibold hover:bg-red-700 transition-colors duration-200 flex items-center",
        }
    };
    
    view! {
        <button 
            class=button_class
            on:click=move |_| {
                if let Some(callback) = &on_emergency {
                    callback();
                }
            }
        >
            <Icon 
                name="emergency-alert".to_string() 
                size=if mobile { IconSize::Xl } else { IconSize::Lg }
                class="text-white mr-3 animate-pulse".to_string() 
            />
            <div class="text-left">
                <div class="font-bold">
                    {if mobile { "EMERGENCY ALERT" } else { "Emergency" }}
                </div>
                {
                    if mobile {
                        view! {
                            <div class="text-sm text-red-100">Tap for immediate help</div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
            </div>
        </button>
    }
}

#[component]
pub fn EmergencyStatusIndicator(
    #[prop()] status: EmergencyStatus,
    #[prop(optional)] show_text: Option<bool>,
) -> impl IntoView {
    let show_text = show_text.unwrap_or(true);
    
    view! {
        <div class="flex items-center space-x-2">
            <div 
                class="w-3 h-3 rounded-full shadow-sm"
                style=format!("background-color: {}", status.color())
            ></div>
            {
                if show_text {
                    view! {
                        <span class="text-sm font-medium text-slate-700">
                            {status.as_str().replace("-", " ").to_uppercase()}
                        </span>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }
        </div>
    }
}
