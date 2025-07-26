use leptos::prelude::*;
use crate::ui::cn;

// Separator component
#[component]
pub fn Separator(
    #[prop(optional)] orientation: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or("horizontal");
    
    let base_classes = "shrink-0 bg-border";
    let orientation_classes = match orientation {
        "vertical" => "h-full w-[1px]",
        _ => "h-[1px] w-full",
    };
    
    let separator_classes = cn(&[
        base_classes,
        orientation_classes,
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=separator_classes />
    }
}

// Progress component
#[component]
pub fn Progress(
    value: f64,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] show_percentage: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let max_val = max.unwrap_or(100.0);
    let percentage = ((value / max_val) * 100.0).min(100.0).max(0.0);
    let show_percentage = show_percentage.unwrap_or(false);
    
    let container_classes = cn(&[
        "relative h-4 w-full overflow-hidden rounded-full bg-secondary",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=container_classes>
            <div 
                class="h-full w-full flex-1 bg-primary transition-all duration-300 ease-in-out"
                style=format!("transform: translateX(-{}%)", 100.0 - percentage)
            />
            {move || {
                if show_percentage {
                    view! {
                        <div class="absolute inset-0 flex items-center justify-center text-xs font-medium text-primary-foreground">
                            {format!("{:.0}%", percentage)}
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}

// Healthcare-specific Progress component
#[component]
pub fn HealthProgressBar(
    value: f64,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] color_scheme: Option<String>,
    #[prop(optional)] show_percentage: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let max_val = max.unwrap_or(100.0);
    let percentage = ((value / max_val) * 100.0).min(100.0).max(0.0);
    let show_percentage = show_percentage.unwrap_or(true);
    let color_scheme = color_scheme.unwrap_or_else(|| "primary".to_string());
    
    let progress_color = match color_scheme.as_str() {
        "health" => "bg-green-500",
        "warning" => "bg-yellow-500", 
        "danger" => "bg-red-500",
        "recovery" => "bg-blue-500",
        _ => "bg-primary",
    };
    
    let container_classes = cn(&[
        "relative h-4 w-full overflow-hidden rounded-full bg-secondary",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class="space-y-2">
            {move || {
                if let Some(label_text) = label.clone() {
                    view! {
                        <div class="flex justify-between text-sm">
                            <span class="font-medium">{label_text}</span>
                            {move || {
                                if show_percentage {
                                    view! {
                                        <span class="text-muted-foreground">
                                            {format!("{:.0}%", percentage)}
                                        </span>
                                    }.into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            <div class=container_classes>
                <div 
                    class=format!("h-full transition-all duration-300 ease-in-out {}", progress_color)
                    style=format!("width: {}%", percentage)
                />
            </div>
        </div>
    }
}

// Avatar component
#[component]
pub fn Avatar(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let avatar_classes = cn(&[
        "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full",
        class.unwrap_or(""),
    ]);
    
    view! {
        <span class=avatar_classes>
            {children()}
        </span>
    }
}

#[component]
pub fn AvatarImage(
    src: String,
    alt: String,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let img_classes = cn(&[
        "aspect-square h-full w-full object-cover",
        class.unwrap_or(""),
    ]);
    
    view! {
        <img src=src alt=alt class=img_classes />
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let fallback_classes = cn(&[
        "flex h-full w-full items-center justify-center rounded-full bg-muted text-sm font-medium",
        class.unwrap_or(""),
    ]);
    
    view! {
        <span class=fallback_classes>
            {children()}
        </span>
    }
}

// Healthcare-specific Avatar components
#[component]
pub fn MedicalRecordAvatar(
    #[prop(optional)] patient_name: Option<String>,
    #[prop(optional)] record_id: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let patient_name = patient_name.unwrap_or_else(|| "Unknown Patient".to_string());
    let record_id = record_id.unwrap_or_else(|| "N/A".to_string());
    
    let initials = patient_name
        .split_whitespace()
        .take(2)
        .map(|word| word.chars().next().unwrap_or('?'))
        .collect::<String>()
        .to_uppercase();
    
    let avatar_classes = cn(&[
        "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full",
        class.unwrap_or(""),
    ]);
    
    view! {
        <span class=avatar_classes>
            <span class="flex h-full w-full items-center justify-center rounded-full bg-blue-100 text-blue-800 text-sm font-medium">
                {initials}
            </span>
        </span>
    }
}

#[component]
pub fn ProviderAvatar(
    #[prop(optional)] provider_name: Option<String>,
    #[prop(optional)] provider_type: Option<String>,
    #[prop(optional)] image_url: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let provider_name = provider_name.unwrap_or_else(|| "Healthcare Provider".to_string());
    let provider_type = provider_type.unwrap_or_else(|| "Provider".to_string());
    
    let initials = provider_name
        .split_whitespace()
        .take(2)
        .map(|word| word.chars().next().unwrap_or('?'))
        .collect::<String>()
        .to_uppercase();
        
    let bg_color = match provider_type.to_lowercase().as_str() {
        "doctor" | "physician" => "bg-green-100 text-green-800",
        "nurse" => "bg-blue-100 text-blue-800",
        "specialist" => "bg-purple-100 text-purple-800",
        "therapist" => "bg-orange-100 text-orange-800",
        _ => "bg-gray-100 text-gray-800",
    };
    
    let avatar_classes = cn(&[
        "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full",
        class.unwrap_or(""),
    ]);
    
    view! {
        <span class=avatar_classes>
            {move || {
                if let Some(img_url) = image_url.clone() {
                    view! {
                        <img src=img_url alt=provider_name.clone() class="aspect-square h-full w-full object-cover" />
                    }.into_any()
                } else {
                    view! {
                        <span class=format!("flex h-full w-full items-center justify-center rounded-full text-sm font-medium {}", bg_color)>
                            {initials.clone()}
                        </span>
                    }.into_any()
                }
            }}
        </span>
    }
}

// Label component - removed duplicate, use the one from input.rs

// Loading Spinner
#[component]
pub fn LoadingSpinner(
    #[prop(optional)] size: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let size_classes = match size.unwrap_or("default") {
        "sm" => "h-4 w-4",
        "lg" => "h-8 w-8",
        "xl" => "h-12 w-12",
        _ => "h-6 w-6",
    };
    
    let spinner_classes = cn(&[
        "animate-spin rounded-full border-2 border-current border-t-transparent text-primary",
        size_classes,
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=spinner_classes role="status" aria-label="Loading">
            <span class="sr-only">"Loading..."</span>
        </div>
    }
}

// Healthcare-specific components

#[component]
pub fn HealthStatusIndicator(
    status: String,
    #[prop(optional)] pulse: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let pulse = pulse.unwrap_or(false);
    
    let (color_class, status_text) = match status.to_lowercase().as_str() {
        "excellent" | "healthy" => ("bg-green-500", "Excellent Health"),
        "good" => ("bg-blue-500", "Good Health"),
        "fair" | "moderate" => ("bg-yellow-500", "Fair Health"),
        "poor" | "critical" => ("bg-red-500", "Critical"),
        "unknown" => ("bg-gray-500", "Unknown Status"),
        _ => ("bg-gray-400", "Custom Status"),
    };
    
    let indicator_classes = cn(&[
        "flex items-center space-x-2 rounded-full px-3 py-1 text-white text-sm font-medium",
        color_class,
        if pulse { "animate-pulse" } else { "" },
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=indicator_classes>
            <div class="h-2 w-2 rounded-full bg-white"></div>
            <span>{status_text}</span>
        </div>
    }
}

#[component]
pub fn VitalSignsDisplay(
    heart_rate: Option<i32>,
    blood_pressure: Option<String>,
    temperature: Option<f64>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let container_classes = cn(&[
        "grid grid-cols-1 md:grid-cols-3 gap-4 p-4 bg-muted/50 rounded-lg",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=container_classes>
            <div class="text-center">
                <div class="text-2xl font-bold text-red-600">
                    {move || heart_rate.map(|hr| hr.to_string()).unwrap_or_else(|| "--".to_string())}
                </div>
                <div class="text-sm text-muted-foreground">"Heart Rate (BPM)"</div>
            </div>
            
            <div class="text-center">
                <div class="text-2xl font-bold text-blue-600">
                    {move || blood_pressure.clone().unwrap_or_else(|| "--/--".to_string())}
                </div>
                <div class="text-sm text-muted-foreground">"Blood Pressure"</div>
            </div>
            
            <div class="text-center">
                <div class="text-2xl font-bold text-green-600">
                    {move || temperature.map(|temp| format!("{}°F", temp)).unwrap_or_else(|| "--°F".to_string())}
                </div>
                <div class="text-sm text-muted-foreground">"Temperature"</div>
            </div>
        </div>
    }
}

#[component]
pub fn AppointmentTimeSlot(
    time: String,
    available: bool,
    #[prop(optional)] selected: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let selected = selected.unwrap_or(false);
    
    let slot_classes = cn(&[
        "px-4 py-2 rounded-md border text-sm font-medium transition-colors cursor-pointer",
        if available {
            if selected {
                "bg-primary text-primary-foreground border-primary"
            } else {
                "bg-background hover:bg-muted border-border hover:border-primary"
            }
        } else {
            "bg-muted text-muted-foreground border-muted cursor-not-allowed opacity-50"
        },
        class.unwrap_or(""),
    ]);
    
    view! {
        <button 
            class=slot_classes
            disabled=!available
            on:click=move |_| {
                if let Some(callback) = on_click.as_ref() {
                    callback();
                }
            }
        >
            {time}
        </button>
    }
}

#[component]
pub fn EmergencyContactCard(
    name: String,
    relationship: String,
    phone: String,
    #[prop(optional)] email: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let card_classes = cn(&[
        "p-4 bg-red-50 border border-red-200 rounded-lg space-y-2",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=card_classes>
            <div class="flex items-center space-x-2">
                <span class="text-red-600 text-lg">"🚨"</span>
                <h3 class="font-semibold text-red-900">{name}</h3>
            </div>
            <p class="text-sm text-red-700">
                <span class="font-medium">"Relationship: "</span>
                {relationship}
            </p>
            <p class="text-sm text-red-700">
                <span class="font-medium">"Phone: "</span>
                <a href=format!("tel:{}", phone.clone()) class="underline hover:no-underline">
                    {phone.clone()}
                </a>
            </p>
            {move || {
                if let Some(email_val) = email.clone() {
                    let email_clone = email_val.clone();
                    view! {
                        <p class="text-sm text-red-700">
                            <span class="font-medium">"Email: "</span>
                            <a href=format!("mailto:{}", email_clone) class="underline hover:no-underline">
                                {email_val}
                            </a>
                        </p>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn MedicationSchedule(
    medication: String,
    dosage: String,
    frequency: String,
    next_dose: String,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let schedule_classes = cn(&[
        "p-3 bg-blue-50 border border-blue-200 rounded-lg",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=schedule_classes>
            <div class="flex items-center justify-between mb-2">
                <h4 class="font-semibold text-blue-900">{medication}</h4>
                <span class="text-blue-600 text-sm">"💊"</span>
            </div>
            <div class="space-y-1 text-sm text-blue-800">
                <p><span class="font-medium">"Dosage: "</span>{dosage}</p>
                <p><span class="font-medium">"Frequency: "</span>{frequency}</p>
                <p><span class="font-medium">"Next Dose: "</span>{next_dose}</p>
            </div>
        </div>
    }
}

#[component]
pub fn ProviderAvailabilityBadge(
    available: bool,
    #[prop(optional)] last_seen: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let badge_classes = cn(&[
        "inline-flex items-center space-x-1 px-2 py-1 rounded-full text-xs font-medium",
        if available {
            "bg-green-100 text-green-800 border border-green-200"
        } else {
            "bg-gray-100 text-gray-800 border border-gray-200"
        },
        class.unwrap_or(""),
    ]);
    
    view! {
        <span class=badge_classes>
            <div class=format!(
                "h-2 w-2 rounded-full {}",
                if available { "bg-green-500 animate-pulse" } else { "bg-gray-400" }
            )></div>
            <span>
                {if available { "Available" } else { "Offline" }}
            </span>
            {move || {
                if let Some(last_seen_val) = last_seen.clone() {
                    if !available {
                        view! {
                            <span class="text-gray-500">
                                " • Last seen " {last_seen_val}
                            </span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                } else {
                    view! {}.into_any()
                }
            }}
        </span>
    }
}
