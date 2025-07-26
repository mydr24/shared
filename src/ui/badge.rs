use leptos::prelude::*;
use crate::ui::{cn, Priority, HealthcareStatus};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Success,
    Warning,
    Info,
}

#[component]
pub fn Badge(
    #[prop(optional)] variant: Option<BadgeVariant>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(BadgeVariant::Default);
    
    let base_classes = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";
    
    let variant_classes = match variant {
        BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground hover:bg-primary/80",
        BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
        BadgeVariant::Destructive => "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80",
        BadgeVariant::Outline => "text-foreground",
        BadgeVariant::Success => "border-transparent bg-green-500 text-white hover:bg-green-600",
        BadgeVariant::Warning => "border-transparent bg-yellow-500 text-white hover:bg-yellow-600",
        BadgeVariant::Info => "border-transparent bg-blue-500 text-white hover:bg-blue-600",
    };
    
    let badge_classes = cn(&[
        base_classes,
        variant_classes,
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=badge_classes>
            {children()}
        </div>
    }
}

// Healthcare-specific badges
#[component]
pub fn PriorityBadge(
    priority: Priority,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (variant, icon, text) = match priority {
        Priority::Emergency => (BadgeVariant::Destructive, "🚨", "Emergency"),
        Priority::Urgent => (BadgeVariant::Warning, "⚠️", "Urgent"),
        Priority::Normal => (BadgeVariant::Info, "ℹ️", "Normal"),
        Priority::Low => (BadgeVariant::Success, "✅", "Low"),
    };
    
    view! {
        <Badge variant=variant class=class.unwrap_or("")>
            <span class="mr-1">{icon}</span>
            {text}
        </Badge>
    }
}

#[component]
pub fn StatusBadge(
    status: HealthcareStatus,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (variant, icon, text) = match status {
        HealthcareStatus::Active => (BadgeVariant::Success, "✅", "Active"),
        HealthcareStatus::Inactive => (BadgeVariant::Secondary, "⭕", "Inactive"),
        HealthcareStatus::Pending => (BadgeVariant::Warning, "⏳", "Pending"),
        HealthcareStatus::Verified => (BadgeVariant::Info, "✔️", "Verified"),
        HealthcareStatus::Suspended => (BadgeVariant::Destructive, "🚫", "Suspended"),
        HealthcareStatus::Emergency => (BadgeVariant::Destructive, "🚨", "Emergency"),
        HealthcareStatus::Stable => (BadgeVariant::Success, "✅", "Stable"),
        HealthcareStatus::NeedsAttention => (BadgeVariant::Warning, "⚠️", "Needs Attention"),
        HealthcareStatus::Critical => (BadgeVariant::Destructive, "🔴", "Critical"),
    };
    
    view! {
        <Badge variant=variant class=class.unwrap_or("")>
            <span class="mr-1">{icon}</span>
            {text}
        </Badge>
    }
}

#[component]
pub fn AppointmentStatusBadge(
    status: String,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (variant, icon) = match status.to_lowercase().as_str() {
        "confirmed" | "scheduled" => (BadgeVariant::Success, "✅"),
        "pending" => (BadgeVariant::Warning, "⏳"),
        "cancelled" => (BadgeVariant::Destructive, "❌"),
        "completed" => (BadgeVariant::Info, "✔️"),
        "in-progress" => (BadgeVariant::Default, "🔄"),
        _ => (BadgeVariant::Secondary, "❓"),
    };
    
    view! {
        <Badge variant=variant class=class.unwrap_or("")>
            <span class="mr-1">{icon}</span>
            {status}
        </Badge>
    }
}

#[component]
pub fn ProviderTypeBadge(
    provider_type: String,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (variant, icon) = match provider_type.to_lowercase().as_str() {
        "doctor" => (BadgeVariant::Info, "👨‍⚕️"),
        "nurse" => (BadgeVariant::Success, "👩‍⚕️"),
        "caregiver" => (BadgeVariant::Default, "🤲"),
        "specialist" => (BadgeVariant::Secondary, "🩺"),
        "therapist" => (BadgeVariant::Info, "🧘"),
        _ => (BadgeVariant::Outline, "👥"),
    };
    
    view! {
        <Badge variant=variant class=class.unwrap_or("")>
            <span class="mr-1">{icon}</span>
            {provider_type}
        </Badge>
    }
}

#[component]
pub fn SpecialtyBadge(
    specialty: String,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let icon = match specialty.to_lowercase().as_str() {
        s if s.contains("cardio") => "❤️",
        s if s.contains("neuro") => "🧠",
        s if s.contains("ortho") => "🦴",
        s if s.contains("pedia") => "👶",
        s if s.contains("derma") => "🧴",
        s if s.contains("eye") | s.contains("ophthal") => "👁️",
        s if s.contains("dental") => "🦷",
        s if s.contains("mental") | s.contains("psych") => "🧘",
        s if s.contains("emergency") => "🚨",
        _ => "🩺",
    };
    
    view! {
        <Badge variant=BadgeVariant::Outline class=class.unwrap_or("")>
            <span class="mr-1">{icon}</span>
            {specialty}
        </Badge>
    }
}

#[component]
pub fn ConsultationTypeBadge(
    consultation_type: String,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (variant, icon) = match consultation_type.to_lowercase().as_str() {
        "telemedicine" | "video" => (BadgeVariant::Info, "📹"),
        "in-person" | "clinic" => (BadgeVariant::Default, "🏥"),
        "home-visit" | "home" => (BadgeVariant::Success, "🏠"),
        "emergency" => (BadgeVariant::Destructive, "🚨"),
        "follow-up" => (BadgeVariant::Secondary, "🔄"),
        _ => (BadgeVariant::Outline, "💬"),
    };
    
    view! {
        <Badge variant=variant class=class.unwrap_or("")>
            <span class="mr-1">{icon}</span>
            {consultation_type}
        </Badge>
    }
}

#[component]
pub fn DurationBadge(
    duration_minutes: u32,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let duration_text = if duration_minutes < 60 {
        format!("{}m", duration_minutes)
    } else {
        let hours = duration_minutes / 60;
        let remaining_minutes = duration_minutes % 60;
        if remaining_minutes == 0 {
            format!("{}h", hours)
        } else {
            format!("{}h {}m", hours, remaining_minutes)
        }
    };
    
    view! {
        <Badge variant=BadgeVariant::Outline class=class.unwrap_or("")>
            <span class="mr-1">"⏱️"</span>
            {duration_text}
        </Badge>
    }
}

#[component]
pub fn RatingBadge(
    rating: f32,
    #[prop(optional)] max_rating: Option<f32>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let max_rating = max_rating.unwrap_or(5.0);
    let variant = if rating >= 4.5 {
        BadgeVariant::Success
    } else if rating >= 3.5 {
        BadgeVariant::Info
    } else if rating >= 2.5 {
        BadgeVariant::Warning
    } else {
        BadgeVariant::Destructive
    };
    
    view! {
        <Badge variant=variant class=class.unwrap_or("")>
            <span class="mr-1">"⭐"</span>
            {format!("{:.1}/{:.0}", rating, max_rating)}
        </Badge>
    }
}
