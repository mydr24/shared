// MyDR24 Layout Components - Modular & Responsive Healthcare UI
// Professional layout system with sidebar navigation and responsive design

mod simple;

pub use simple::{SimpleAppLayout, SimpleNavigationItem};

#[derive(Debug, Clone)]
pub struct NavigationItem {
    pub title: String,
    pub icon: String,
    pub href: Option<String>,
    pub children: Vec<NavigationItem>,
    pub badge: Option<String>,
    pub is_emergency: bool,
}

impl NavigationItem {
    pub fn link(title: &str, icon: &str, href: &str) -> Self {
impl NavigationItem {
    pub fn link(title: &str, icon: &str, href: &str) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            href: Some(href.to_string()),
            children: vec![],
            badge: None,
            is_emergency: false,
        }
    }
    
    pub fn group(title: &str, icon: &str, children: Vec<NavigationItem>) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            href: None,
            children,
            badge: None,
            is_emergency: false,
        }
    }
    
    pub fn emergency(title: &str, icon: &str, href: &str) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            href: Some(href.to_string()),
            children: vec![],
            badge: Some("!".to_string()),
            is_emergency: true,
        }
    }
    
    pub fn with_badge(mut self, badge: &str) -> Self {
        self.badge = Some(badge.to_string());
        self
    }
}

// Utility functions for common navigation patterns
impl NavigationItem {
    pub fn admin_nav() -> Vec<Self> {
        vec![
            Self::link("Dashboard", "interface-dashboard", "/admin"),
            Self::link("Providers", "health-provider", "/admin/providers"),
            Self::link("Patients", "health-patient", "/admin/patients"),
            Self::link("Organizations", "business-building", "/admin/organizations"),
            Self::link("Analytics", "interface-chart", "/admin/analytics"),
            Self::group("Services", "health-service", vec![
                Self::link("Catalog", "interface-list", "/admin/services"),
                Self::link("Pricing", "business-currency", "/admin/pricing"),
                Self::link("Bookings", "interface-calendar", "/admin/bookings"),
            ]),
            Self::group("System", "interface-settings", vec![
                Self::link("Settings", "interface-gear", "/admin/settings"),
                Self::link("Users", "interface-users", "/admin/users"),
                Self::link("Audit", "security-shield", "/admin/audit"),
            ]),
        ]
    }
    
    pub fn provider_nav() -> Vec<Self> {
        vec![
            Self::link("Dashboard", "interface-dashboard", "/provider"),
            Self::link("Schedule", "interface-calendar", "/provider/schedule"),
            Self::link("Patients", "health-patient", "/provider/patients"),
            Self::link("Services", "health-service", "/provider/services"),
            Self::link("Earnings", "business-currency", "/provider/earnings"),
            Self::emergency("Emergency", "emergency-alert", "/provider/emergency"),
        ]
    }
    
    pub fn patient_nav() -> Vec<Self> {
        vec![
            Self::link("Dashboard", "interface-dashboard", "/patient"),
            Self::link("Find Care", "health-provider", "/patient/find"),
            Self::link("Appointments", "interface-calendar", "/patient/appointments"),
            Self::link("Health Records", "health-medical", "/patient/records"),
            Self::link("Prescriptions", "health-prescription", "/patient/prescriptions"),
            Self::emergency("Emergency", "emergency-alert", "/patient/emergency"),
        ]
    }
}
