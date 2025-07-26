use leptos::prelude::*;
use crate::ui::cn;

#[component]
pub fn Separator(
    #[prop(optional)] orientation: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or("horizontal");
    
    let separator_classes = match orientation {
        "vertical" => cn(&[
            "shrink-0 bg-border w-px h-full",
            class.unwrap_or(""),
        ]),
        _ => cn(&[
            "shrink-0 bg-border h-px w-full",
            class.unwrap_or(""),
        ]),
    };
    
    view! {
        <div class=separator_classes role="separator"></div>
    }
}

#[component]
pub fn Progress(
    value: f32,
    #[prop(optional)] max: Option<f32>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] show_percentage: Option<bool>,
) -> impl IntoView {
    let max = max.unwrap_or(100.0);
    let show_percentage = show_percentage.unwrap_or(false);
    let percentage = (value / max * 100.0).min(100.0).max(0.0);
    
    let progress_classes = cn(&[
        "relative h-4 w-full overflow-hidden rounded-full bg-secondary",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class="space-y-2">
            <div class=progress_classes>
                <div 
                    class="h-full w-full flex-1 bg-primary transition-all duration-500 ease-in-out"
                    style=format!("transform: translateX(-{}%)", 100.0 - percentage)
                ></div>
            </div>
            {move || {
                if show_percentage {
                    view! {
                        <div class="text-sm text-muted-foreground text-center">
                            {format!("{:.1}%", percentage)}
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn Avatar(
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] alt: Option<String>,
    #[prop(optional)] fallback: Option<String>,
    #[prop(optional)] size: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or("default");
    let alt = alt.unwrap_or_else(|| "Avatar".to_string());
    
    let size_classes = match size {
        "sm" => "h-8 w-8",
        "lg" => "h-12 w-12",
        "xl" => "h-16 w-16",
        _ => "h-10 w-10",
    };
    
    let avatar_classes = cn(&[
        "relative flex shrink-0 overflow-hidden rounded-full",
        size_classes,
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=avatar_classes>
            {move || {
                if let Some(src_url) = src.clone() {
                    view! {
                        <img 
                            class="aspect-square h-full w-full object-cover"
                            src=src_url
                            alt=alt.clone()
                        />
                    }.into_any()
                } else if let Some(fallback_text) = fallback.clone() {
                    view! {
                        <div class="flex h-full w-full items-center justify-center rounded-full bg-muted text-muted-foreground font-medium">
                            {fallback_text}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex h-full w-full items-center justify-center rounded-full bg-muted">
                            <svg class="h-4 w-4 text-muted-foreground" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
                            </svg>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn Skeleton(
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] width: Option<&'static str>,
    #[prop(optional)] height: Option<&'static str>,
) -> impl IntoView {
    let width = width.unwrap_or("w-full");
    let height = height.unwrap_or("h-4");
    
    let skeleton_classes = cn(&[
        "animate-pulse rounded-md bg-muted",
        width,
        height,
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=skeleton_classes></div>
    }
}

// Healthcare-specific components
#[component]
pub fn HealthProgressBar(
    current_value: f32,
    target_value: f32,
    metric_name: String,
    #[prop(optional)] unit: Option<String>,
    #[prop(optional)] color_scheme: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let unit = unit.unwrap_or_else(|| "".to_string());
    let color_scheme = color_scheme.unwrap_or("default");
    let percentage = (current_value / target_value * 100.0).min(100.0).max(0.0);
    
    let color_classes = match color_scheme {
        "heart" => "bg-red-500",
        "pressure" => "bg-blue-500",
        "glucose" => "bg-yellow-500",
        "weight" => "bg-green-500",
        _ => "bg-primary",
    };
    
    view! {
        <div class="space-y-2">
            <div class="flex justify-between items-center">
                <span class="text-sm font-medium text-gray-700">{metric_name}</span>
                <span class="text-sm text-gray-500">
                    {format!("{:.1}{} / {:.1}{}", current_value, unit, target_value, unit)}
                </span>
            </div>
            <div class="relative h-4 w-full overflow-hidden rounded-full bg-gray-200">
                <div 
                    class=format!("h-full transition-all duration-500 ease-in-out rounded-full {}", color_classes)
                    style=format!("width: {}%", percentage)
                ></div>
            </div>
            <div class="text-xs text-gray-500 text-center">
                {format!("{:.1}% of target", percentage)}
            </div>
        </div>
    }
}

#[component]
pub fn MedicalRecordAvatar(
    patient_name: String,
    #[prop(optional)] patient_photo: Option<String>,
    #[prop(optional)] medical_record_number: Option<String>,
    #[prop(optional)] size: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or("default");
    let initials = patient_name
        .split_whitespace()
        .map(|word| word.chars().next().unwrap_or(' '))
        .take(2)
        .collect::<String>()
        .to_uppercase();
    
    view! {
        <div class="flex items-center space-x-3">
            <Avatar 
                src=patient_photo
                alt=Some(format!("{} Avatar", patient_name))
                fallback=Some(initials)
                size=Some(size)
                class=class
            />
            <div class="flex flex-col">
                <span class="text-sm font-medium text-gray-900">{patient_name}</span>
                {move || {
                    if let Some(mrn) = medical_record_number.clone() {
                        view! {
                            <span class="text-xs text-gray-500">"MRN: " {mrn}</span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
pub fn ProviderAvatar(
    provider_name: String,
    specialty: String,
    #[prop(optional)] provider_photo: Option<String>,
    #[prop(optional)] rating: Option<f32>,
    #[prop(optional)] verification_status: Option<String>,
    #[prop(optional)] size: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or("default");
    let initials = provider_name
        .split_whitespace()
        .map(|word| word.chars().next().unwrap_or(' '))
        .take(2)
        .collect::<String>()
        .to_uppercase();
    
    let verification_color = match verification_status.as_deref() {
        Some("verified") => "text-green-600",
        Some("pending") => "text-yellow-600",
        Some("suspended") => "text-red-600",
        _ => "text-gray-500",
    };
    
    view! {
        <div class="flex items-center space-x-3">
            <div class="relative">
                <Avatar 
                    src=provider_photo
                    alt=Some(format!("Dr. {} Avatar", provider_name))
                    fallback=Some(initials)
                    size=Some(size)
                    class=class
                />
                {move || {
                    if let Some(status) = verification_status.clone() {
                        if status == "verified" {
                            view! {
                                <div class="absolute -bottom-1 -right-1 bg-green-500 rounded-full p-1">
                                    <svg class="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                                    </svg>
                                </div>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    } else {
                        view! {}.into_any()
                    }
                }}
            </div>
            <div class="flex flex-col">
                <div class="flex items-center space-x-2">
                    <span class="text-sm font-medium text-gray-900">{provider_name}</span>
                    {move || {
                        if let Some(rating_val) = rating {
                            view! {
                                <div class="flex items-center space-x-1">
                                    <span class="text-yellow-400 text-xs">"⭐"</span>
                                    <span class="text-xs text-gray-600">{format!("{:.1}", rating_val)}</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}
                </div>
                <span class="text-xs text-gray-500">{specialty}</span>
                {move || {
                    if let Some(status) = verification_status.clone() {
                        view! {
                            <span class=format!("text-xs font-medium {}", verification_color)>
                                {status.to_uppercase()}
                            </span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </div>
        </div>
    }
}
