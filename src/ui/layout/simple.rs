use leptos::prelude::*;
use crate::ui::{Icon, IconSize};

// Simple navigation structure without complex callbacks
#[derive(Debug, Clone)]
pub struct SimpleNavigationItem {
    pub title: String,
    pub icon: String,
    pub href: Option<String>,
    pub badge: Option<String>,
    pub is_emergency: bool,
    pub children: Vec<SimpleNavigationItem>,
}

impl SimpleNavigationItem {
    pub fn link(title: &str, icon: &str, href: &str) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            href: Some(href.to_string()),
            badge: None,
            is_emergency: title == "Emergency",
            children: vec![],
        }
    }
    
    pub fn group(title: &str, icon: &str, children: Vec<SimpleNavigationItem>) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            href: None,
            badge: None,
            is_emergency: false,
            children,
        }
    }
    
    pub fn provider_nav() -> Vec<Self> {
        vec![
            Self::link("Dashboard", "interface-dashboard", "/provider/dashboard"),
            Self::link("Patient Queue", "user-patient", "/provider/queue"),
            Self::link("Emergency", "emergency-alert", "/provider/emergency"),
            Self::link("Appointments", "interface-calendar", "/provider/appointments"),
            Self::link("Patient Records", "health-medical", "/provider/records"),
            Self::link("Messaging", "interface-feedback", "/provider/messages"),
            Self::link("Reports", "interface-chart", "/provider/reports"),
            Self::link("Settings", "interface-settings", "/provider/settings"),
        ]
    }
    
    pub fn patient_nav() -> Vec<Self> {
        vec![
            Self::link("Dashboard", "interface-dashboard", "/patient/dashboard"),
            Self::link("Book Appointment", "interface-calendar", "/patient/booking"),
            Self::link("My Health Records", "health-medical", "/patient/records"),
            Self::link("Medications", "health-pills", "/patient/medications"),
            Self::link("Messages", "interface-feedback", "/patient/messages"),
            Self::link("My Doctors", "user-doctor", "/patient/doctors"),
            Self::link("Insurance", "security-certificate", "/patient/insurance"),
            Self::link("Settings", "interface-settings", "/patient/settings"),
        ]
    }
}

#[component]
pub fn SimpleAppLayout(
    #[prop()] navigation_items: Vec<SimpleNavigationItem>,
    #[prop()] brand_title: String,
    #[prop()] brand_icon: String,
    #[prop()] user_name: String,
    #[prop()] user_role: String,
    children: Children,
) -> impl IntoView {
    let (is_mobile_sidebar_open, set_is_mobile_sidebar_open) = signal(false);
    
    // Clone the brand values for use in the mobile overlay closure
    let brand_icon_cloned = brand_icon.clone();
    let brand_title_cloned = brand_title.clone();
    let brand_icon_mobile = brand_icon.clone();
    let brand_title_mobile = brand_title.clone();
    
    view! {
        <div class="min-h-screen bg-gray-50 flex">
            // Sidebar for desktop
            <aside class="hidden lg:flex lg:flex-col lg:w-64 lg:fixed lg:inset-y-0 bg-white border-r border-gray-200">
                <div class="flex-1 flex flex-col min-h-0">
                    // Brand header
                    <div class="flex items-center h-16 flex-shrink-0 px-4 bg-blue-600 text-white">
                        <Icon name=brand_icon_cloned.clone() size=IconSize::Lg class="text-white mr-3".to_string() />
                        <h1 class="text-lg font-semibold">{brand_title_cloned.clone()}</h1>
                    </div>
                    
                    // Navigation
                    <nav class="flex-1 px-4 py-4 space-y-2 overflow-y-auto">
                        {navigation_items.iter().map(|item| view! {
                            <SimpleNavigationItemComponent item=item.clone() />
                        }).collect_view()}
                    </nav>
                    
                    // Emergency button
                    <div class="flex-shrink-0 p-4 border-t border-gray-200">
                        <a 
                            href="/emergency"
                            class="w-full bg-red-600 text-white flex items-center justify-center px-4 py-2 rounded-lg font-medium hover:bg-red-700 transition-colors duration-200"
                        >
                            <Icon name="emergency-alert".to_string() size=IconSize::Md class="text-white mr-2".to_string() />
                            <span class="font-medium">"Emergency"</span>
                        </a>
                    </div>
                </div>
            </aside>
            
            // Main content area
            <main class="flex-1 lg:pl-64">
                // Top bar
                <header class="bg-white shadow-sm border-b border-gray-200 px-4 py-3 lg:px-6">
                    <div class="flex items-center justify-between">
                        <button 
                            class="lg:hidden p-2 rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-100"
                            on:click=move |_| set_is_mobile_sidebar_open.update(|open| *open = !*open)
                        >
                            <Icon name="interface-menu".to_string() size=IconSize::Lg class="text-gray-600".to_string() />
                        </button>
                        
                        <div class="flex items-center space-x-4">
                            <button class="p-2 rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-100">
                                <Icon name="interface-bell".to_string() size=IconSize::Md class="text-gray-600".to_string() />
                            </button>
                            
                            {
                                let initial = user_name.chars().next().unwrap_or('U').to_string();
                                view! {
                                    <div class="flex items-center space-x-3">
                                        <div class="text-right">
                                            <p class="text-sm font-medium text-gray-900">{user_name.clone()}</p>
                                            <p class="text-xs text-gray-500">{user_role.clone()}</p>
                                        </div>
                                        <div class="h-8 w-8 bg-blue-600 rounded-full flex items-center justify-center">
                                            <span class="text-white text-sm font-bold">{initial}</span>
                                        </div>
                                    </div>
                                }
                            }
                        </div>
                    </div>
                </header>
                
                // Page content
                <div class="flex-1 p-4 lg:p-6">
                    {children()}
                </div>
            </main>
            
            // Mobile sidebar overlay
            {move || {
                if is_mobile_sidebar_open.get() {
                    view! {
                        <div class="fixed inset-0 z-50 lg:hidden">
                            <div 
                                class="fixed inset-0 bg-black bg-opacity-50"
                                on:click=move |_| set_is_mobile_sidebar_open.set(false)
                            ></div>
                            <aside class="fixed inset-y-0 left-0 w-64 bg-white border-r border-gray-200 z-50">
                                <div class="flex-1 flex flex-col min-h-0">
                                    // Brand header
                                    <div class="flex items-center justify-between h-16 flex-shrink-0 px-4 bg-blue-600 text-white">
                                        <div class="flex items-center">
                                            <Icon name=brand_icon_mobile.clone() size=IconSize::Lg class="text-white mr-3".to_string() />
                                            <h1 class="text-lg font-semibold">{brand_title_mobile.clone()}</h1>
                                        </div>
                                        <button 
                                            class="text-white"
                                            on:click=move |_| set_is_mobile_sidebar_open.set(false)
                                        >
                                            <Icon name="interface-close".to_string() size=IconSize::Md class="text-white".to_string() />
                                        </button>
                                    </div>
                                    
                                    // Navigation
                                    <nav class="flex-1 px-4 py-4 space-y-2 overflow-y-auto">
                                        {navigation_items.iter().map(|item| view! {
                                            <SimpleNavigationItemComponent item=item.clone() />
                                        }).collect_view()}
                                    </nav>
                                </div>
                            </aside>
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
fn SimpleNavigationItemComponent(
    #[prop()] item: SimpleNavigationItem,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(false);
    let has_children = !item.children.is_empty();
    
    if let Some(href) = &item.href {
        // Navigation link
        let item_class = if item.is_emergency {
            "bg-red-50 text-red-700 border-red-200"
        } else {
            "text-gray-700 hover:text-gray-900 hover:bg-gray-100"
        };
        
        view! {
            <a 
                href=href
                class=format!("group flex items-center px-3 py-2 rounded-lg text-sm font-medium transition-colors duration-200 border {}", item_class)
            >
                <Icon name=item.icon.clone() size=IconSize::Md class="mr-3".to_string() />
                <span class="flex-1">{item.title.clone()}</span>
                {item.badge.as_ref().map(|badge| {
                    view! {
                        <span class="bg-red-100 text-red-800 text-xs px-2 py-1 rounded-full">
                            {badge.clone()}
                        </span>
                    }
                })}
            </a>
        }.into_any()
    } else if has_children {
        // Group with children
        view! {
            <div>
                <button 
                    class="w-full group flex items-center px-3 py-2 rounded-lg text-sm font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-100 transition-colors duration-200"
                    on:click=move |_| set_is_expanded.update(|expanded| *expanded = !*expanded)
                >
                    <Icon name=item.icon.clone() size=IconSize::Md class="mr-3".to_string() />
                    <span class="flex-1 text-left">{item.title.clone()}</span>
                    <Icon 
                        name=if is_expanded.get() { "interface-chevron-down".to_string() } else { "interface-chevron-right".to_string() }
                        size=IconSize::Sm 
                        class="transition-transform duration-200".to_string()
                    />
                </button>
                
                {move || {
                    if is_expanded.get() {
                        view! {
                            <div class="ml-6 mt-1 space-y-1">
                                {item.children.iter().map(|child| {
                                    view! {
                                        <SimpleNavigationItemComponent item=child.clone() />
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </div>
        }.into_any()
    } else {
        // Regular item without link
        view! {
            <div class="px-3 py-2 text-sm font-medium text-gray-400 flex items-center">
                <Icon name=item.icon.clone() size=IconSize::Md class="mr-3".to_string() />
                {item.title.clone()}
            </div>
        }.into_any()
    }
}
