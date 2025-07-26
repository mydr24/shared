use leptos::prelude::*;
use crate::ui::cn;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertVariant {
    Default,
    Destructive,
    Warning,
    Success,
    Info,
}

#[component]
pub fn Alert(
    #[prop(optional)] variant: Option<AlertVariant>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(AlertVariant::Default);
    
    let base_classes = "relative w-full rounded-lg border p-4";
    
    let variant_classes = match variant {
        AlertVariant::Default => "bg-background text-foreground",
        AlertVariant::Destructive => "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive",
        AlertVariant::Warning => "border-yellow-500/50 bg-yellow-50 text-yellow-900 dark:border-yellow-500 [&>svg]:text-yellow-600",
        AlertVariant::Success => "border-green-500/50 bg-green-50 text-green-900 dark:border-green-500 [&>svg]:text-green-600",
        AlertVariant::Info => "border-blue-500/50 bg-blue-50 text-blue-900 dark:border-blue-500 [&>svg]:text-blue-600",
    };
    
    let alert_classes = cn(&[
        base_classes,
        variant_classes,
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=alert_classes role="alert">
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let title_classes = cn(&[
        "mb-1 font-medium leading-none tracking-tight",
        class.unwrap_or(""),
    ]);
    
    view! {
        <h5 class=title_classes>
            {children()}
        </h5>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let description_classes = cn(&[
        "text-sm [&_p]:leading-relaxed",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=description_classes>
            {children()}
        </div>
    }
}

// Healthcare-specific alert components
#[component]
pub fn EmergencyAlert(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let title_text = title.unwrap_or_else(|| "Emergency Alert".to_string());
    
    view! {
        <Alert variant=Some(AlertVariant::Destructive) class=Some("border-red-600 bg-red-50 animate-pulse")>
            <div class="flex items-center">
                <span class="text-red-600 mr-2 text-lg">"🚨"</span>
                <AlertTitle class=Some("text-red-800")>
                    {title_text}
                </AlertTitle>
            </div>
            <AlertDescription class=Some("text-red-700 mt-2")>
                {children()}
            </AlertDescription>
        </Alert>
    }
}

#[component]
pub fn MedicalWarningAlert(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let title_text = title.unwrap_or_else(|| "Medical Warning".to_string());
    
    view! {
        <Alert variant=Some(AlertVariant::Warning) class=class>
            <div class="flex items-center">
                <span class="text-yellow-600 mr-2 text-lg">"⚠️"</span>
                <AlertTitle>
                    {title_text}
                </AlertTitle>
            </div>
            <AlertDescription class=Some("mt-2")>
                {children()}
            </AlertDescription>
        </Alert>
    }
}

#[component]
pub fn SuccessAlert(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let title_text = title.unwrap_or_else(|| "Success".to_string());
    
    view! {
        <Alert variant=Some(AlertVariant::Success) class=class>
            <div class="flex items-center">
                <span class="text-green-600 mr-2 text-lg">"✅"</span>
                <AlertTitle>
                    {title_text}
                </AlertTitle>
            </div>
            <AlertDescription class=Some("mt-2")>
                {children()}
            </AlertDescription>
        </Alert>
    }
}

#[component]
pub fn InfoAlert(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let title_text = title.unwrap_or_else(|| "Information".to_string());
    
    view! {
        <Alert variant=Some(AlertVariant::Info) class=class>
            <div class="flex items-center">
                <span class="text-blue-600 mr-2 text-lg">"ℹ️"</span>
                <AlertTitle>
                    {title_text}
                </AlertTitle>
            </div>
            <AlertDescription class=Some("mt-2")>
                {children()}
            </AlertDescription>
        </Alert>
    }
}

#[component]
pub fn AppointmentReminderAlert(
    provider_name: String,
    appointment_time: String,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <Alert variant=Some(AlertVariant::Info) class=class>
            <div class="flex items-center">
                <span class="text-blue-600 mr-2 text-lg">"📅"</span>
                <AlertTitle>
                    "Upcoming Appointment Reminder"
                </AlertTitle>
            </div>
            <AlertDescription class=Some("mt-2")>
                <p>"You have an appointment with " <strong>{provider_name}</strong> " at " <strong>{appointment_time}</strong></p>
                <p class="mt-1 text-sm">"Please ensure you're ready 10 minutes before the scheduled time."</p>
            </AlertDescription>
        </Alert>
    }
}

#[component]
pub fn MedicationReminderAlert(
    medication_name: String,
    dosage: String,
    #[prop(optional)] next_dose_time: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <Alert variant=Some(AlertVariant::Warning) class=class>
            <div class="flex items-center">
                <span class="text-orange-600 mr-2 text-lg">"💊"</span>
                <AlertTitle>
                    "Medication Reminder"
                </AlertTitle>
            </div>
            <AlertDescription class=Some("mt-2")>
                <p>"Time to take your " <strong>{medication_name}</strong> " - " <strong>{dosage}</strong></p>
                {move || {
                    if let Some(next_time) = next_dose_time.clone() {
                        view! {
                            <p class="mt-1 text-sm">"Next dose: " {next_time}</p>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </AlertDescription>
        </Alert>
    }
}

#[component]
pub fn SystemMaintenanceAlert(
    maintenance_time: String,
    #[prop(optional)] duration: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <Alert variant=Some(AlertVariant::Warning) class=class>
            <div class="flex items-center">
                <span class="text-orange-600 mr-2 text-lg">"🔧"</span>
                <AlertTitle>
                    "Scheduled System Maintenance"
                </AlertTitle>
            </div>
            <AlertDescription class=Some("mt-2")>
                <p>"The system will be under maintenance on " <strong>{maintenance_time}</strong></p>
                {move || {
                    if let Some(duration_val) = duration.clone() {
                        view! {
                            <p class="mt-1 text-sm">"Expected duration: " {duration_val}</p>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
                <p class="mt-1 text-sm">"Some services may be temporarily unavailable during this time."</p>
            </AlertDescription>
        </Alert>
    }
}

#[component]
pub fn PrivacyComplianceAlert(
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <Alert variant=Some(AlertVariant::Info) class=class>
            <div class="flex items-center">
                <span class="text-blue-600 mr-2 text-lg">"🔒"</span>
                <AlertTitle>
                    "Privacy & Security Notice"
                </AlertTitle>
            </div>
            <AlertDescription class=Some("mt-2")>
                <p>"This platform is HIPAA compliant and uses end-to-end encryption to protect your health information."</p>
                <p class="mt-1 text-sm">"Your data is secure and will never be shared without your explicit consent."</p>
            </AlertDescription>
        </Alert>
    }
}
