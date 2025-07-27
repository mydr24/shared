// MyDR24 Professional Icon System
// SVG-based icon components for healthcare applications

use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconSize {
    Xs,      // 12px
    Sm,      // 16px
    Md,      // 20px
    Lg,      // 24px
    Xl,      // 32px
    Xxl,     // 48px
}

impl IconSize {
    pub fn class(&self) -> &'static str {
        match self {
            IconSize::Xs => "w-3 h-3",
            IconSize::Sm => "w-4 h-4", 
            IconSize::Md => "w-5 h-5",
            IconSize::Lg => "w-6 h-6",
            IconSize::Xl => "w-8 h-8",
            IconSize::Xxl => "w-12 h-12",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconAnimation {
    None,
    Spin,
    Pulse,
    Bounce,
    Ping,
}

impl IconAnimation {
    pub fn class(&self) -> &'static str {
        match self {
            IconAnimation::None => "",
            IconAnimation::Spin => "animate-spin",
            IconAnimation::Pulse => "animate-pulse",
            IconAnimation::Bounce => "animate-bounce", 
            IconAnimation::Ping => "animate-ping",
        }
    }
}

#[component]
pub fn Icon(
    #[prop()] name: String,
    #[prop(optional)] size: Option<IconSize>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] animation: Option<IconAnimation>,
    #[prop(optional)] color: Option<String>,
) -> impl IntoView {
    let size = size.unwrap_or(IconSize::Md);
    let animation = animation.unwrap_or(IconAnimation::None);
    
    let icon_svg = get_icon_svg(&name);
    
    let combined_class = format!(
        "{} {} {} {}",
        size.class(),
        animation.class(),
        color.unwrap_or_else(|| "text-current".to_string()),
        class.unwrap_or_default()
    );
    
    view! {
        <svg 
            class=combined_class
            fill="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
            aria-hidden="true"
        >
            {icon_svg}
        </svg>
    }
}

// Icon registry system
fn get_icon_svg(name: &str) -> String {
    let icons = icon_registry();
    icons.get(name)
        .unwrap_or(&FALLBACK_ICON)
        .to_string()
}

// Healthcare icon registry
fn icon_registry() -> HashMap<&'static str, &'static str> {
    let mut icons = HashMap::new();
    
    // Medical Icons
    icons.insert("medical-heart", MEDICAL_HEART);
    icons.insert("medical-stethoscope", MEDICAL_STETHOSCOPE);
    icons.insert("medical-hospital", MEDICAL_HOSPITAL);
    icons.insert("medical-prescription", MEDICAL_PRESCRIPTION);
    icons.insert("medical-syringe", MEDICAL_SYRINGE);
    icons.insert("medical-bag", MEDICAL_BAG);
    icons.insert("medical-cross", MEDICAL_CROSS);
    icons.insert("medical-thermometer", MEDICAL_THERMOMETER);
    icons.insert("medical-bandage", MEDICAL_BANDAGE);
    icons.insert("medical-pill", MEDICAL_PILL);
    
    // Emergency Icons
    icons.insert("emergency-alert", EMERGENCY_ALERT);
    icons.insert("emergency-call", EMERGENCY_CALL);
    icons.insert("emergency-ambulance", EMERGENCY_AMBULANCE);
    icons.insert("emergency-siren", EMERGENCY_SIREN);
    icons.insert("emergency-shield", EMERGENCY_SHIELD);
    
    // User Icons
    icons.insert("user-doctor", USER_DOCTOR);
    icons.insert("user-patient", USER_PATIENT);
    icons.insert("user-nurse", USER_NURSE);
    icons.insert("user-admin", USER_ADMIN);
    icons.insert("user-organization", USER_ORGANIZATION);
    icons.insert("user-group", USER_GROUP);
    
    // Interface Icons
    icons.insert("interface-dashboard", INTERFACE_DASHBOARD);
    icons.insert("interface-calendar", INTERFACE_CALENDAR);
    icons.insert("interface-chat", INTERFACE_CHAT);
    icons.insert("interface-settings", INTERFACE_SETTINGS);
    icons.insert("interface-search", INTERFACE_SEARCH);
    icons.insert("interface-location", INTERFACE_LOCATION);
    icons.insert("interface-clock", INTERFACE_CLOCK);
    icons.insert("interface-phone", INTERFACE_PHONE);
    icons.insert("interface-mail", INTERFACE_MAIL);
    icons.insert("interface-logout", INTERFACE_LOGOUT);
    icons.insert("interface-menu", INTERFACE_MENU);
    icons.insert("interface-close", INTERFACE_CLOSE);
    icons.insert("interface-plus", INTERFACE_PLUS);
    icons.insert("interface-minus", INTERFACE_MINUS);
    icons.insert("interface-chevron-right", INTERFACE_CHEVRON_RIGHT);
    icons.insert("interface-chevron-left", INTERFACE_CHEVRON_LEFT);
    icons.insert("interface-chevron-up", INTERFACE_CHEVRON_UP);
    icons.insert("interface-chevron-down", INTERFACE_CHEVRON_DOWN);
    
    // Status Icons
    icons.insert("status-available", STATUS_AVAILABLE);
    icons.insert("status-busy", STATUS_BUSY);
    icons.insert("status-offline", STATUS_OFFLINE);
    icons.insert("status-completed", STATUS_COMPLETED);
    icons.insert("status-pending", STATUS_PENDING);
    icons.insert("status-warning", STATUS_WARNING);
    icons.insert("status-error", STATUS_ERROR);
    icons.insert("status-success", STATUS_SUCCESS);
    
    icons
}

// SVG Icon Definitions
const FALLBACK_ICON: &str = r#"<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>"#;

// Medical Icons
const MEDICAL_HEART: &str = r#"<path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>"#;

const MEDICAL_STETHOSCOPE: &str = r#"<path d="M19.48 12.35c-1.57-4.08-7.16-4.3-5.81-10.23.1-.44.02-.81-.06-.94C13.46 1 13.21 1 12.81 1c-.4 0-.65 0-.8.18-.08.13-.16.5-.06.94 1.35 5.93-4.24 6.15-5.81 10.23C3.97 14.63 2 17.41 2 20.5 2 21.88 3.12 23 4.5 23c1.38 0 2.5-1.12 2.5-2.5 0-1.74-.73-3.26-1.87-4.36C6.27 14.93 8.12 13.5 9.5 13.5c1.38 0 3.23 1.43 4.37 2.64C12.73 17.24 12 18.76 12 20.5c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5c0-3.09-1.97-5.87-4.52-8.15z"/>"#;

const MEDICAL_HOSPITAL: &str = r#"<path d="M19 8h-2V6c0-1.1-.9-2-2-2H9c-1.1 0-2 .9-2 2v2H5c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2zM9 6h6v2H9V6zm6 10h-2v2h-2v-2H9v-2h2v-2h2v2h2v2z"/>"#;

const MEDICAL_PRESCRIPTION: &str = r#"<path d="M14 2H6c-1.1 0-2 .9-2 2v16c0 1.1.89 2 2 2h12c1.1 0 2-.9 2-2V8l-6-6zm4 18H6V4h7v5h5v11zm-7-9h-1v1h1v1h-1v1h-1v-1H8v-1h1v-1H8v-1h1V9h1v1h1v1z"/>"#;

const MEDICAL_SYRINGE: &str = r#"<path d="M19.5 3.5L18 2l-1.5 1.5L15 2l-1.5 1.5L12 2l-1.5 1.5L9 2l-1.5 1.5-1.5-1.5-1.5 1.5 1.5 1.5-1.5 1.5 1.5 1.5 1.5-1.5 1.5 1.5 1.5-1.5 1.5 1.5 1.5-1.5 1.5 1.5 1.5-1.5-1.5-1.5 1.5-1.5z"/>"#;

const MEDICAL_BAG: &str = r#"<path d="M20 6h-2.18c.11-.31.18-.65.18-1a2.996 2.996 0 0 0-5.5-1.65l-.5.67-.5-.68C10.96 2.54 10.05 2 9 2 7.34 2 6 3.34 6 5c0 .35.07.69.18 1H4c-1.11 0-2 .89-2 2v11c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zM9 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm6 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM4 19V8h16v11H4z"/>"#;

const MEDICAL_CROSS: &str = r#"<path d="M17 7h-4V3c0-.55-.45-1-1-1s-1 .45-1 1v4H7c-.55 0-1 .45-1 1s.45 1 1 1h4v4c0 .55.45 1 1 1s1-.45 1-1V9h4c.55 0 1-.45 1-1s-.45-1-1-1z"/>"#;

const MEDICAL_THERMOMETER: &str = r#"<path d="M15 13V5c0-1.66-1.34-3-3-3S9 3.34 9 5v8c-1.21.91-2 2.37-2 4 0 2.76 2.24 5 5 5s5-2.24 5-5c0-1.63-.79-3.09-2-4zM11 5c0-.55.45-1 1-1s1 .45 1 1h-2z"/>"#;

const MEDICAL_BANDAGE: &str = r#"<path d="M20.73 7.31l-3.04-3.04a.996.996 0 0 0-1.41 0l-8.49 8.49c-.78.78-.78 2.05 0 2.83l3.04 3.04c.78.78 2.05.78 2.83 0l8.49-8.49c.39-.39.39-1.02 0-1.41l-1.42-1.42zm-8.5 8.5l-1.41-1.41 8.49-8.49 1.41 1.41-8.49 8.49z"/>"#;

const MEDICAL_PILL: &str = r#"<path d="M21.5 9h-19C1.67 9 1 9.67 1 10.5v3C1 14.33 1.67 15 2.5 15h19c.83 0 1.5-.67 1.5-1.5v-3c0-.83-.67-1.5-1.5-1.5zm-10 4h-9v-2h9v2z"/>"#;

// Emergency Icons  
const EMERGENCY_ALERT: &str = r#"<path d="M12 2L1 21h22L12 2zm0 3.99L19.53 19H4.47L12 5.99zM11 16h2v2h-2zm0-6h2v4h-2z"/>"#;

const EMERGENCY_CALL: &str = r#"<path d="M20.01 15.38c-1.23 0-2.42-.2-3.53-.56-.35-.12-.74-.03-1.01.24l-1.57 1.97c-2.83-1.35-5.48-3.9-6.89-6.83l1.95-1.66c.27-.28.35-.67.24-1.02-.37-1.11-.56-2.3-.56-3.53 0-.54-.45-.99-.99-.99H4.19C3.65 3 3 3.24 3 3.99 3 13.28 10.73 21 20.01 21c.71 0 .99-.63.99-1.18v-3.45c0-.54-.45-.99-.99-.99z"/>"#;

const EMERGENCY_AMBULANCE: &str = r#"<path d="M18.92 6.01C18.72 5.42 18.16 5 17.5 5h-11C5.84 5 5.28 5.42 5.08 6.01L3 12v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.5 16c-.83 0-1.5-.67-1.5-1.5S5.67 13 6.5 13s1.5.67 1.5 1.5S7.33 16 6.5 16zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 11l1.5-4.5h11L19 11H5z"/>"#;

const EMERGENCY_SIREN: &str = r#"<path d="M18 10c0-4.42-3.58-8-8-8s-8 3.58-8 8c0 1.95.7 3.73 1.86 5.12L5 16.24C3.74 14.78 3 12.48 3 10c0-4.97 4.03-9 9-9s9 4.03 9 9c0 2.48-.74 4.78-2 6.24l1.14 1.12C21.3 13.73 22 11.95 22 10zM12 6c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4z"/>"#;

const EMERGENCY_SHIELD: &str = r#"<path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>"#;

// User Icons
const USER_DOCTOR: &str = r#"<path d="M14 6V4h-4v2c0 1.1.9 2 2 2s2-.9 2-2zm-2 4c-1.1 0-2-.9-2-2V6H8v2c0 2.21 1.79 4 4 4s4-1.79 4-4V6h-2v2c0 1.1-.9 2-2 2zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>"#;

const USER_PATIENT: &str = r#"<path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>"#;

const USER_NURSE: &str = r#"<path d="M12 6c1.1 0 2-.9 2-2 0-.38-.1-.73-.29-1.03L12 1.27l-1.71 1.7C10.1 3.27 10 3.62 10 4c0 1.1.9 2 2 2zm0 2c-2.67 0-8 1.34-8 4v1h16v-1c0-2.66-5.33-4-8-4zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/>"#;

const USER_ADMIN: &str = r#"<path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z"/>"#;

const USER_ORGANIZATION: &str = r#"<path d="M12 7V3H2v18h20V7H12zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm0-4H4V9h2v2zm0-4H4V5h2v2zm4 12H8v-2h2v2zm0-4H8v-2h2v2zm0-4H8V9h2v2zm0-4H8V5h2v2zm10 12h-8v-2h2v-2h-2v-2h2v-2h-2V9h8v10z"/>"#;

const USER_GROUP: &str = r#"<path d="M16 4c0-1.11.89-2 2-2s2 .89 2 2-.89 2-2 2-2-.89-2-2zM4 4c0-1.11.89-2 2-2s2 .89 2 2-.89 2-2 2-2-.89-2-2zm5 4c0-1.11.89-2 2-2s2 .89 2 2-.89 2-2 2-2-.89-2-2zm3 7v-2c0-1.1-.9-2-2-2H8c-1.1 0-2 .9-2 2v2H4v4h16v-4h-2z"/>"#;

// Interface Icons
const INTERFACE_DASHBOARD: &str = r#"<path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/>"#;

const INTERFACE_CALENDAR: &str = r#"<path d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z"/>"#;

const INTERFACE_CHAT: &str = r#"<path d="M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4V4c0-1.1-.9-2-2-2zm-2 12H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z"/>"#;

const INTERFACE_SETTINGS: &str = r#"<path d="M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.82,11.68,4.82,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z"/>"#;

const INTERFACE_SEARCH: &str = r#"<path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>"#;

const INTERFACE_LOCATION: &str = r#"<path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/>"#;

const INTERFACE_CLOCK: &str = r#"<path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/>"#;

const INTERFACE_PHONE: &str = r#"<path d="M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z"/>"#;

const INTERFACE_MAIL: &str = r#"<path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"/>"#;

const INTERFACE_LOGOUT: &str = r#"<path d="M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.58L17 17l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z"/>"#;

const INTERFACE_MENU: &str = r#"<path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>"#;

const INTERFACE_CLOSE: &str = r#"<path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>"#;

const INTERFACE_PLUS: &str = r#"<path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>"#;

const INTERFACE_MINUS: &str = r#"<path d="M19 13H5v-2h14v2z"/>"#;

const INTERFACE_CHEVRON_RIGHT: &str = r#"<path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>"#;

const INTERFACE_CHEVRON_LEFT: &str = r#"<path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/>"#;

const INTERFACE_CHEVRON_UP: &str = r#"<path d="M7.41 15.41L12 10.83l4.59 4.58L18 14l-6-6-6 6z"/>"#;

const INTERFACE_CHEVRON_DOWN: &str = r#"<path d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z"/>"#;

// Status Icons
const STATUS_AVAILABLE: &str = r#"<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>"#;

const STATUS_BUSY: &str = r#"<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 11H7v-2h10v2z"/>"#;

const STATUS_OFFLINE: &str = r#"<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8 0-1.85.63-3.55 1.69-4.9L16.9 18.31C15.55 19.37 13.85 20 12 20zm6.31-3.1L7.1 5.69C8.45 4.63 10.15 4 12 4c4.42 0 8 3.58 8 8 0 1.85-.63 3.55-1.69 4.9z"/>"#;

const STATUS_COMPLETED: &str = r#"<path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>"#;

const STATUS_PENDING: &str = r#"<path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/>"#;

const STATUS_WARNING: &str = r#"<path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/>"#;

const STATUS_ERROR: &str = r#"<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>"#;

const STATUS_SUCCESS: &str = r#"<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>"#;
