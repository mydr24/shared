use leptos::prelude::*;
use crate::ui::{Icon, IconSize, Priority, HealthcareStatus, StatsTrend, CardVariant};

#[derive(Debug, Clone)]
pub struct PatientInfo {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub gender: String,
    pub blood_type: Option<String>,
    pub medical_id: String,
    pub avatar_url: Option<String>,
    pub status: HealthcareStatus,
    pub last_visit: Option<String>,
    pub next_appointment: Option<String>,
    pub primary_doctor: Option<String>,
    pub emergency_contact: Option<String>,
    pub allergies: Vec<String>,
    pub chronic_conditions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HealthMetric {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub normal_range: Option<String>,
    pub status: HealthcareStatus,
    pub trend: StatsTrend,
    pub last_updated: String,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct AppointmentInfo {
    pub id: String,
    pub patient_name: String,
    pub doctor_name: String,
    pub specialty: String,
    pub appointment_type: String,
    pub date: String,
    pub time: String,
    pub duration: String,
    pub location: String,
    pub status: AppointmentStatus,
    pub priority: Priority,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppointmentStatus {
    Scheduled,
    Confirmed,
    InProgress,
    Completed,
    Cancelled,
    NoShow,
}

impl AppointmentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppointmentStatus::Scheduled => "scheduled",
            AppointmentStatus::Confirmed => "confirmed",
            AppointmentStatus::InProgress => "in-progress",
            AppointmentStatus::Completed => "completed",
            AppointmentStatus::Cancelled => "cancelled",
            AppointmentStatus::NoShow => "no-show",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            AppointmentStatus::Scheduled => "hsl(48 96% 53%)",    // Yellow
            AppointmentStatus::Confirmed => "hsl(210 100% 50%)",  // Blue
            AppointmentStatus::InProgress => "hsl(280 65% 60%)",  // Purple
            AppointmentStatus::Completed => "hsl(142 76% 36%)",   // Green
            AppointmentStatus::Cancelled => "hsl(0 84% 60%)",     // Red
            AppointmentStatus::NoShow => "hsl(0 0% 50%)",         // Gray
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            AppointmentStatus::Scheduled => "interface-calendar",
            AppointmentStatus::Confirmed => "interface-check",
            AppointmentStatus::InProgress => "interface-clock",
            AppointmentStatus::Completed => "interface-check-circle",
            AppointmentStatus::Cancelled => "interface-close",
            AppointmentStatus::NoShow => "interface-alert",
        }
    }
}

#[component] 
pub fn PatientCard(
    #[prop()] patient: PatientInfo,
    #[prop(optional)] variant: Option<CardVariant>,
    #[prop(optional)] compact: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    let variant = variant.unwrap_or(CardVariant::Patient);
    let compact = compact.unwrap_or(false);
    let patient_id = patient.id.clone();
    
    let card_class = match variant {
        CardVariant::Emergency => "bg-gradient-to-br from-red-50 to-red-100 border-red-200 shadow-lg",
        CardVariant::Patient => "bg-gradient-to-br from-blue-50 to-blue-100 border-blue-200 shadow-md",
        _ => "bg-white border-slate-200 shadow-sm",
    };
    
    let status_color = match patient.status {
        HealthcareStatus::Active => "bg-green-500",
        HealthcareStatus::Critical => "bg-red-500 animate-pulse",
        HealthcareStatus::NeedsAttention => "bg-yellow-500",
        _ => "bg-gray-500",
    };
    
    view! {
        <div 
            class=format!("rounded-xl border-2 p-6 hover:shadow-lg transition-all duration-200 cursor-pointer {}", card_class)
            on:click=move |_| {
                if let Some(callback) = &on_click {
                    callback(patient_id.clone());
                }
            }
        >
            {
                if compact {
                    view! {
                        // Compact patient card
                        <div class="flex items-center space-x-4">
                            <div class="relative">
                                {
                                    if let Some(avatar) = &patient.avatar_url {
                                        view! {
                                            <img src=avatar.clone() class="w-12 h-12 rounded-full object-cover border-2 border-white shadow-sm" alt="Patient avatar" />
                                        }.into_any()
                                    } else {
                                        let initial = patient.name.chars().next().unwrap_or('P').to_string();
                                        view! {
                                            <div class="w-12 h-12 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center border-2 border-white shadow-sm">
                                                <span class="text-white font-bold">{initial}</span>
                                            </div>
                                        }.into_any()
                                    }
                                }
                                <div class=format!("absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-white shadow-sm {}", status_color)></div>
                            </div>
                            
                            <div class="flex-1 min-w-0">
                                <h3 class="font-semibold text-slate-900 truncate">{patient.name.clone()}</h3>
                                <p class="text-sm text-slate-600">
                                    {patient.age} years • {patient.gender.clone()}
                                </p>
                                <p class="text-xs text-slate-500 font-mono">{patient.medical_id.clone()}</p>
                            </div>
                            
                            {
                                if let Some(blood_type) = &patient.blood_type {
                                    view! {
                                        <div class="text-right">
                                            <div class="bg-red-100 text-red-800 px-3 py-1 rounded-full text-sm font-semibold">
                                                {blood_type.clone()}
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }
                        </div>
                    }.into_any()
                } else {
                    view! {
                        // Full patient card
                        <div class="space-y-4">
                            // Header with avatar and basic info
                            <div class="flex items-start space-x-4">
                                <div class="relative">
                                    {
                                        if let Some(avatar) = &patient.avatar_url {
                                            view! {
                                                <img src=avatar.clone() class="w-16 h-16 rounded-full object-cover border-3 border-white shadow-lg" alt="Patient avatar" />
                                            }.into_any()
                                        } else {
                                            let initial = patient.name.chars().next().unwrap_or('P').to_string();
                                            view! {
                                                <div class="w-16 h-16 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center border-3 border-white shadow-lg">
                                                    <span class="text-white text-xl font-bold">{initial}</span>
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                    <div class=format!("absolute -bottom-1 -right-1 w-5 h-5 rounded-full border-3 border-white shadow-sm {}", status_color)></div>
                                </div>
                                
                                <div class="flex-1">
                                    <h3 class="text-xl font-bold text-slate-900 mb-1">{patient.name.clone()}</h3>
                                    <div class="flex items-center space-x-4 text-sm text-slate-600 mb-2">
                                        <span>{patient.age} years old</span>
                                        <span>•</span>
                                        <span>{patient.gender.clone()}</span>
                                        {
                                            if let Some(blood_type) = &patient.blood_type {
                                                view! {
                                                    <>
                                                        <span>"•"</span>
                                                        <span class="bg-red-100 text-red-800 px-2 py-1 rounded-full text-xs font-semibold">
                                                            {blood_type.clone()}
                                                        </span>
                                                    </>
                                                }.into_any()
                                            } else {
                                                view! {}.into_any()
                                            }
                                        }
                                    </div>
                                    <p class="text-sm text-slate-500 font-mono">ID: {patient.medical_id.clone()}</p>
                                </div>
                            </div>
                            
                            // Medical information grid
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                {
                                    if let Some(doctor) = &patient.primary_doctor {
                                        view! {
                                            <div class="flex items-center space-x-2">
                                                <Icon name="user-doctor".to_string() size=IconSize::Sm class="text-slate-500".to_string() />
                                                <div>
                                                    <p class="text-xs text-slate-500">Primary Doctor</p>
                                                    <p class="text-sm font-medium text-slate-800">{doctor.clone()}</p>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                                
                                {
                                    if let Some(last_visit) = &patient.last_visit {
                                        view! {
                                            <div class="flex items-center space-x-2">
                                                <Icon name="interface-calendar".to_string() size=IconSize::Sm class="text-slate-500".to_string() />
                                                <div>
                                                    <p class="text-xs text-slate-500">Last Visit</p>
                                                    <p class="text-sm font-medium text-slate-800">{last_visit.clone()}</p>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                                
                                {
                                    if let Some(next_appointment) = &patient.next_appointment {
                                        view! {
                                            <div class="flex items-center space-x-2">
                                                <Icon name="interface-clock".to_string() size=IconSize::Sm class="text-slate-500".to_string() />
                                                <div>
                                                    <p class="text-xs text-slate-500">Next Appointment</p>
                                                    <p class="text-sm font-medium text-slate-800">{next_appointment.clone()}</p>
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
                                            <div class="flex items-center space-x-2">
                                                <Icon name="interface-phone".to_string() size=IconSize::Sm class="text-slate-500".to_string() />
                                                <div>
                                                    <p class="text-xs text-slate-500">Emergency Contact</p>
                                                    <p class="text-sm font-medium text-slate-800">{contact.clone()}</p>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                            </div>
                            
                            // Allergies and conditions
                            {
                                if !patient.allergies.is_empty() || !patient.chronic_conditions.is_empty() {
                                    view! {
                                        <div class="pt-4 border-t border-slate-200 space-y-3">
                                            {
                                                if !patient.allergies.is_empty() {
                                                    view! {
                                                        <div class="bg-red-50 border border-red-200 rounded-lg p-3">
                                                            <div class="flex items-center space-x-2 mb-2">
                                                                <Icon name="emergency-alert".to_string() size=IconSize::Sm class="text-red-600".to_string() />
                                                                <span class="font-semibold text-red-800">Allergies</span>
                                                            </div>
                                                            <div class="flex flex-wrap gap-2">
                                                                {patient.allergies.iter().map(|allergy| view! {
                                                                    <span class="bg-red-100 text-red-800 px-2 py-1 rounded-full text-xs font-medium">
                                                                        {allergy.clone()}
                                                                    </span>
                                                                }).collect_view()}
                                                            </div>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            }
                                            
                                            {
                                                if !patient.chronic_conditions.is_empty() {
                                                    view! {
                                                        <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-3">
                                                            <div class="flex items-center space-x-2 mb-2">
                                                                <Icon name="health-medical".to_string() size=IconSize::Sm class="text-yellow-600".to_string() />
                                                                <span class="font-semibold text-yellow-800">Chronic Conditions</span>
                                                            </div>
                                                            <div class="flex flex-wrap gap-2">
                                                                {patient.chronic_conditions.iter().map(|condition| view! {
                                                                    <span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded-full text-xs font-medium">
                                                                        {condition.clone()}
                                                                    </span>
                                                                }).collect_view()}
                                                            </div>
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
                            }
                        </div>
                    }.into_any()
                }
            }
        </div>
    }
}

#[component]
pub fn HealthMetricCard(
    #[prop()] metric: HealthMetric,
    #[prop(optional)] compact: Option<bool>,
) -> impl IntoView {
    let compact = compact.unwrap_or(false);
    
    let status_colors = match metric.status {
        HealthcareStatus::Critical => ("bg-red-50 border-red-200", "text-red-800", "text-red-600"),
        HealthcareStatus::NeedsAttention => ("bg-yellow-50 border-yellow-200", "text-yellow-800", "text-yellow-600"),
        HealthcareStatus::Stable => ("bg-green-50 border-green-200", "text-green-800", "text-green-600"),
        _ => ("bg-slate-50 border-slate-200", "text-slate-800", "text-slate-600"),
    };
    
    view! {
        <div class=format!("rounded-lg border p-4 hover:shadow-md transition-shadow duration-200 {}", status_colors.0)>
            <div class="flex items-center justify-between mb-3">
                <div class="flex items-center space-x-2">
                    <Icon name=metric.icon.clone() size=IconSize::Md class=status_colors.2.to_string() />
                    <h3 class=format!("font-semibold {}", status_colors.1)>{metric.name.clone()}</h3>
                </div>
                
                <div class="flex items-center space-x-1">
                    <span class=format!("text-sm {}", metric.trend.color())>
                        {metric.trend.icon()}
                    </span>
                </div>
            </div>
            
            <div class="space-y-2">
                <div class="flex items-baseline space-x-2">
                    <span class=format!("text-2xl font-bold {}", status_colors.1)>
                        {metric.value.clone()}
                    </span>
                    <span class=format!("text-sm {}", status_colors.2)>
                        {metric.unit.clone()}
                    </span>
                </div>
                
                {
                    if let Some(normal_range) = &metric.normal_range {
                        view! {
                            <p class=format!("text-xs {}", status_colors.2)>
                                Normal: {normal_range.clone()}
                            </p>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
                
                {
                    if !compact {
                        view! {
                            <p class="text-xs text-slate-500">
                                Last updated: {metric.last_updated.clone()}
                            </p>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
            </div>
        </div>
    }
}

#[component]
pub fn AppointmentCard(
    #[prop()] appointment: AppointmentInfo,
    #[prop(optional)] compact: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(String)>>,
    #[prop(optional)] show_patient: Option<bool>,
) -> impl IntoView {
    let compact = compact.unwrap_or(false);
    let show_patient = show_patient.unwrap_or(true);
    let appointment_id = appointment.id.clone();
    
    let priority_colors = match appointment.priority {
        Priority::Emergency => ("bg-red-50 border-red-300", "text-red-800", "text-red-600"),
        Priority::Urgent => ("bg-orange-50 border-orange-300", "text-orange-800", "text-orange-600"),
        _ => ("bg-blue-50 border-blue-200", "text-blue-800", "text-blue-600"),
    };
    
    view! {
        <div 
            class=format!("rounded-lg border-2 p-4 hover:shadow-lg transition-all duration-200 cursor-pointer {}", priority_colors.0)
            on:click=move |_| {
                if let Some(callback) = &on_click {
                    callback(appointment_id.clone());
                }
            }
        >
            <div class="flex items-start justify-between mb-3">
                <div class="flex-1">
                    <div class="flex items-center space-x-2 mb-1">
                        <h3 class=format!("font-semibold {}", priority_colors.1)>
                            {appointment.appointment_type.clone()}
                        </h3>
                        <span 
                            class="px-2 py-1 text-xs font-semibold rounded-full shadow-sm"
                            style=format!("background-color: {}; color: white;", appointment.status.color())
                        >
                            {appointment.status.as_str().replace("-", " ").to_uppercase()}
                        </span>
                    </div>
                    
                    {
                        if show_patient {
                            view! {
                                <p class=format!("text-sm font-medium {}", priority_colors.2)>
                                    {appointment.patient_name.clone()}
                                </p>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }
                    
                    <p class="text-sm text-slate-600">
                        {appointment.doctor_name.clone()} • {appointment.specialty.clone()}
                    </p>
                </div>
                
                <div class="text-right">
                    {
                        if appointment.priority != Priority::Normal {
                            view! {
                                <span class=format!("px-2 py-1 text-xs font-semibold rounded-full {}",
                                    match appointment.priority {
                                        Priority::Emergency => "bg-red-600 text-white",
                                        Priority::Urgent => "bg-orange-600 text-white",
                                        _ => "bg-blue-600 text-white",
                                    }
                                )>
                                    {appointment.priority.as_str().to_uppercase()}
                                </span>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }
                </div>
            </div>
            
            <div class="grid grid-cols-2 gap-4 text-sm">
                <div class="flex items-center space-x-2">
                    <Icon name="interface-calendar".to_string() size=IconSize::Sm class="text-slate-500".to_string() />
                    <span class="text-slate-700">{appointment.date.clone()}</span>
                </div>
                <div class="flex items-center space-x-2">
                    <Icon name="interface-clock".to_string() size=IconSize::Sm class="text-slate-500".to_string() />
                    <span class="text-slate-700">{appointment.time.clone()}</span>
                </div>
                <div class="flex items-center space-x-2">
                    <Icon name="location-pin".to_string() size=IconSize::Sm class="text-slate-500".to_string() />
                    <span class="text-slate-700">{appointment.location.clone()}</span>
                </div>
                <div class="flex items-center space-x-2">
                    <Icon name="interface-clock".to_string() size=IconSize::Sm class="text-slate-500".to_string() />
                    <span class="text-slate-700">{appointment.duration.clone()}</span>
                </div>
            </div>
            
            {
                if let Some(notes) = &appointment.notes {
                    view! {
                        <div class="mt-3 pt-3 border-t border-slate-200">
                            <p class="text-sm text-slate-600 italic">
                                {notes.clone()}
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }
        </div>
    }
}
