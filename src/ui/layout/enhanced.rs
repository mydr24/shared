use leptos::prelude::*;
use crate::ui::{Icon, IconSize, Priority, HealthcareStatus, Button, Variant};

#[derive(Debug, Clone)]
pub struct HealthcareNavigationItem {
    pub title: String,
    pub icon: String,
    pub href: Option<String>,
    pub badge: Option<String>,
    pub priority: Priority,
    pub children: Vec<HealthcareNavigationItem>,
    pub notification_count: Option<u32>,
    pub is_emergency: bool,
    pub is_active: bool,
}

impl HealthcareNavigationItem {
    pub fn link(title: &str, icon: &str, href: &str) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            href: Some(href.to_string()),
            badge: None,
            priority: Priority::Normal,
            children: vec![],
            notification_count: None,
            is_emergency: title == "Emergency",
            is_active: false,
        }
    }
    
    pub fn emergency(title: &str, icon: &str, href: &str) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            href: Some(href.to_string()),
            badge: Some("!".to_string()),
            priority: Priority::Emergency,
            children: vec![],
            notification_count: None,
            is_emergency: true,
            is_active: false,
        }
    }
    
    pub fn with_notifications(mut self, count: u32) -> Self {
        self.notification_count = Some(count);
        self
    }
    
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub name: String,
    pub role: String,
    pub avatar_url: Option<String>,
    pub status: HealthcareStatus,
    pub location: Option<String>,
    pub shift_status: Option<String>,
}

impl UserInfo {
    pub fn new(name: &str, role: &str) -> Self {
        Self {
            name: name.to_string(),
            role: role.to_string(),
            avatar_url: None,
            status: HealthcareStatus::Active,
            location: None,
            shift_status: None,
        }
    }
    
    pub fn provider(name: &str, location: &str) -> Self {
        Self {
            name: name.to_string(),
            role: "Healthcare Provider".to_string(),
            avatar_url: None,
            status: HealthcareStatus::Active,
            location: Some(location.to_string()),
            shift_status: Some("On Duty".to_string()),
        }
    }
    
    pub fn patient(name: &str) -> Self {
        Self {
            name: name.to_string(),
            role: "Patient".to_string(),
            avatar_url: None,
            status: HealthcareStatus::Active,
            location: None,
            shift_status: None,
        }
    }
    
    pub fn admin(name: &str) -> Self {
        Self {
            name: name.to_string(),
            role: "System Administrator".to_string(),
            avatar_url: None,
            status: HealthcareStatus::Active,
            location: None,
            shift_status: None,
        }
    }
}

#[component]
pub fn HealthcareAppLayout(
    #[prop()] navigation_items: Vec<HealthcareNavigationItem>,
    #[prop()] brand_title: String,
    #[prop()] brand_icon: String,
    #[prop()] user_info: UserInfo,
    #[prop(optional)] emergency_count: Option<u32>,
    #[prop(optional)] system_status: Option<HealthcareStatus>,
    #[prop(optional)] on_emergency_click: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    let (is_mobile_sidebar_open, set_is_mobile_sidebar_open) = signal(false);
    let (show_user_menu, set_show_user_menu) = signal(false);
    
    // Emergency button handler
    let emergency_handler = move |_| {
        if let Some(callback) = &on_emergency_click {
            callback();
        }
    };
    
    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 flex">
            // Enhanced Desktop Sidebar
            <aside class="hidden lg:flex lg:flex-col lg:w-72 lg:fixed lg:inset-y-0 bg-white border-r border-slate-200 shadow-lg">
                <div class="flex-1 flex flex-col min-h-0">
                    // Enhanced Brand Header with Status
                    <div class="flex items-center justify-between h-20 flex-shrink-0 px-6 bg-gradient-to-r from-blue-600 to-blue-700 text-white shadow-lg">
                        <div class="flex items-center">
                            <Icon name=brand_icon.clone() size=IconSize::Xl class="text-white mr-3 drop-shadow-sm".to_string() />
                            <div>
                                <h1 class="text-xl font-bold tracking-tight">{brand_title.clone()}</h1>
                                <p class="text-blue-100 text-xs">Healthcare Platform</p>
                            </div>
                        </div>
                        
                        // System Status Indicator
                        {
                            let status_color = match system_status.unwrap_or(HealthcareStatus::Active) {
                                HealthcareStatus::Active => "bg-green-400",
                                HealthcareStatus::Emergency => "bg-red-400 animate-pulse",
                                HealthcareStatus::Pending => "bg-yellow-400",
                                _ => "bg-gray-400",
                            };
                            view! {
                                <div class="flex items-center">
                                    <div class=format!("w-3 h-3 rounded-full {} shadow-sm", status_color)></div>
                                </div>
                            }
                        }
                    </div>
                    
                    // User Info Panel
                    <div class="px-6 py-4 bg-slate-50 border-b border-slate-200">
                        <div class="flex items-center space-x-3">
                            <div class="relative">
                                {
                                    if let Some(avatar) = &user_info.avatar_url {
                                        view! {
                                            <img src=avatar.clone() class="h-10 w-10 rounded-full object-cover border-2 border-white shadow-sm" alt="User avatar" />
                                        }.into_any()
                                    } else {
                                        let initial = user_info.name.chars().next().unwrap_or('U').to_string();
                                        view! {
                                            <div class="h-10 w-10 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center border-2 border-white shadow-sm">
                                                <span class="text-white text-sm font-bold">{initial}</span>
                                            </div>
                                        }.into_any()
                                    }
                                }
                                
                                // Status indicator
                                <div class=format!("absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-white shadow-sm {}", 
                                    match user_info.status {
                                        HealthcareStatus::Active => "bg-green-400",
                                        HealthcareStatus::Emergency => "bg-red-400 animate-pulse",
                                        HealthcareStatus::Pending => "bg-yellow-400",
                                        _ => "bg-gray-400",
                                    }
                                )></div>
                            </div>
                            
                            <div class="flex-1 min-w-0">
                                <p class="text-sm font-semibold text-slate-900 truncate">{user_info.name.clone()}</p>
                                <p class="text-xs text-slate-500 truncate">{user_info.role.clone()}</p>
                                {
                                    if let Some(location) = &user_info.location {
                                        view! {
                                            <p class="text-xs text-blue-600 truncate flex items-center">
                                                <Icon name="location-pin".to_string() size=IconSize::Xs class="mr-1".to_string() />
                                                {location.clone()}
                                            </p>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                            </div>
                        </div>
                    </div>
                    
                    // Enhanced Navigation
                    <nav class="flex-1 px-4 py-6 space-y-2 overflow-y-auto">
                        {navigation_items.iter().map(|item| view! {
                            <HealthcareNavigationItemComponent item=item.clone() />
                        }).collect_view()}
                    </nav>
                    
                    // Enhanced Emergency Button
                    <div class="flex-shrink-0 p-6 border-t border-slate-200 bg-slate-50">
                        <button 
                            class="w-full bg-gradient-to-r from-red-600 to-red-700 text-white flex items-center justify-center px-6 py-3 rounded-xl font-semibold hover:from-red-700 hover:to-red-800 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-105 active:scale-95"
                            on:click=emergency_handler
                        >
                            <Icon name="emergency-alert".to_string() size=IconSize::Lg class="text-white mr-3 animate-pulse".to_string() />
                            <div class="text-left">
                                <div class="font-bold">Emergency Alert</div>
                                {
                                    if let Some(count) = emergency_count {
                                        view! {
                                            <div class="text-xs text-red-100">{count} active alerts</div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="text-xs text-red-100">24/7 Response</div>
                                        }.into_any()
                                    }
                                }
                            </div>
                        </button>
                    </div>
                </div>
            </aside>
            
            // Main Content Area
            <main class="flex-1 lg:pl-72">
                // Enhanced Top Bar
                <header class="bg-white shadow-sm border-b border-slate-200 px-4 py-4 lg:px-8 sticky top-0 z-40 backdrop-blur-sm bg-white/95">
                    <div class="flex items-center justify-between">
                        // Mobile menu button
                        <button 
                            class="lg:hidden p-2 rounded-lg text-slate-600 hover:text-slate-900 hover:bg-slate-100 transition-colors duration-200"
                            on:click=move |_| set_is_mobile_sidebar_open.update(|open| *open = !*open)
                        >
                            <Icon name="interface-menu".to_string() size=IconSize::Lg class="text-slate-600".to_string() />
                        </button>
                        
                        // Breadcrumb or page title could go here
                        <div class="hidden lg:block">
                            <h2 class="text-lg font-semibold text-slate-900">Dashboard</h2>
                        </div>
                        
                        // Top bar actions
                        <div class="flex items-center space-x-3">
                            // Quick Emergency Button (mobile-friendly)
                            <button 
                                class="lg:hidden p-2 rounded-lg bg-red-600 text-white hover:bg-red-700 transition-colors duration-200 shadow-sm"
                                on:click=emergency_handler
                            >
                                <Icon name="emergency-alert".to_string() size=IconSize::Md class="text-white".to_string() />
                            </button>
                            
                            // Notifications
                            <button class="relative p-2 rounded-lg text-slate-600 hover:text-slate-900 hover:bg-slate-100 transition-colors duration-200">
                                <Icon name="interface-bell".to_string() size=IconSize::Md class="text-slate-600".to_string() />
                                <span class="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full"></span>
                            </button>
                            
                            // User menu (mobile)
                            <div class="lg:hidden relative">
                                <button 
                                    class="flex items-center space-x-2 p-2 rounded-lg hover:bg-slate-100 transition-colors duration-200"
                                    on:click=move |_| set_show_user_menu.update(|show| *show = !*show)
                                >
                                    <div class="h-8 w-8 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center">
                                        <span class="text-white text-sm font-bold">
                                            {user_info.name.chars().next().unwrap_or('U').to_string()}
                                        </span>
                                    </div>
                                </button>
                            </div>
                        </div>
                    </div>
                </header>
                
                // Page Content with Enhanced Styling
                <div class="flex-1 p-4 lg:p-8">
                    <div class="max-w-7xl mx-auto">
                        {children()}
                    </div>
                </div>
            </main>
            
            // Enhanced Mobile Sidebar
            {move || {
                if is_mobile_sidebar_open.get() {
                    view! {
                        <div class="fixed inset-0 z-50 lg:hidden">
                            <div 
                                class="fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm"
                                on:click=move |_| set_is_mobile_sidebar_open.set(false)
                            ></div>
                            <aside class="fixed inset-y-0 left-0 w-80 bg-white border-r border-slate-200 z-50 shadow-2xl">
                                <div class="flex-1 flex flex-col min-h-0">
                                    // Mobile brand header
                                    <div class="flex items-center justify-between h-20 flex-shrink-0 px-6 bg-gradient-to-r from-blue-600 to-blue-700 text-white">
                                        <div class="flex items-center">
                                            <Icon name=brand_icon.clone() size=IconSize::Xl class="text-white mr-3".to_string() />
                                            <div>
                                                <h1 class="text-xl font-bold">{brand_title.clone()}</h1>
                                                <p class="text-blue-100 text-xs">Healthcare Platform</p>
                                            </div>
                                        </div>
                                        <button 
                                            class="text-white p-2 hover:bg-blue-800 rounded-lg transition-colors duration-200"
                                            on:click=move |_| set_is_mobile_sidebar_open.set(false)
                                        >
                                            <Icon name="interface-close".to_string() size=IconSize::Md class="text-white".to_string() />
                                        </button>
                                    </div>
                                    
                                    // Mobile user info
                                    <div class="px-6 py-4 bg-slate-50 border-b border-slate-200">
                                        <div class="flex items-center space-x-3">
                                            <div class="h-12 w-12 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center border-2 border-white shadow-sm">
                                                <span class="text-white font-bold">
                                                    {user_info.name.chars().next().unwrap_or('U').to_string()}
                                                </span>
                                            </div>
                                            <div class="flex-1">
                                                <p class="font-semibold text-slate-900">{user_info.name.clone()}</p>
                                                <p class="text-sm text-slate-500">{user_info.role.clone()}</p>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    // Mobile navigation
                                    <nav class="flex-1 px-4 py-6 space-y-2 overflow-y-auto">
                                        {navigation_items.iter().map(|item| view! {
                                            <HealthcareNavigationItemComponent item=item.clone() />
                                        }).collect_view()}
                                    </nav>
                                    
                                    // Mobile emergency button
                                    <div class="flex-shrink-0 p-6 border-t border-slate-200 bg-slate-50">
                                        <button 
                                            class="w-full bg-gradient-to-r from-red-600 to-red-700 text-white flex items-center justify-center px-6 py-4 rounded-xl font-semibold shadow-lg"
                                            on:click=emergency_handler
                                        >
                                            <Icon name="emergency-alert".to_string() size=IconSize::Lg class="text-white mr-3 animate-pulse".to_string() />
                                            <div>
                                                <div class="font-bold">Emergency Alert</div>
                                                <div class="text-xs text-red-100">Tap for immediate help</div>
                                            </div>
                                        </button>
                                    </div>
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
fn HealthcareNavigationItemComponent(
    #[prop()] item: HealthcareNavigationItem,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(false);
    let has_children = !item.children.is_empty();
    
    if let Some(href) = &item.href {
        // Enhanced navigation link with healthcare styling
        let (base_class, hover_class, icon_class) = match item.priority {
            Priority::Emergency => (
                "bg-gradient-to-r from-red-50 to-red-100 text-red-800 border border-red-200 shadow-sm",
                "hover:from-red-100 hover:to-red-200 hover:shadow-md",
                "text-red-600"
            ),
            Priority::Urgent => (
                "bg-gradient-to-r from-orange-50 to-orange-100 text-orange-800 border border-orange-200",
                "hover:from-orange-100 hover:to-orange-200",
                "text-orange-600"
            ),
            _ => if item.is_active {
                (
                    "bg-gradient-to-r from-blue-50 to-blue-100 text-blue-900 border border-blue-200 shadow-sm",
                    "hover:from-blue-100 hover:to-blue-200",
                    "text-blue-600"
                )
            } else {
                (
                    "text-slate-700 border border-transparent",
                    "hover:text-slate-900 hover:bg-slate-100 hover:border-slate-200",
                    "text-slate-500"
                )
            }
        };
        
        view! {
            <a 
                href=href
                class=format!("group flex items-center px-4 py-3 rounded-xl text-sm font-medium transition-all duration-200 {} {}", base_class, hover_class)
            >
                <Icon name=item.icon.clone() size=IconSize::Md class=format!("mr-3 {}", icon_class) />
                <span class="flex-1">{item.title.clone()}</span>
                
                // Notification badge
                {
                    if let Some(count) = item.notification_count {
                        view! {
                            <span class="bg-red-500 text-white text-xs px-2 py-1 rounded-full min-w-[1.25rem] h-5 flex items-center justify-center font-semibold shadow-sm">
                                {if count > 99 { "99+".to_string() } else { count.to_string() }}
                            </span>
                        }.into_any()
                    } else if let Some(badge) = &item.badge {
                        view! {
                            <span class="bg-red-500 text-white text-xs px-2 py-1 rounded-full font-bold animate-pulse shadow-sm">
                                {badge.clone()}
                            </span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
            </a>
        }.into_any()
    } else if has_children {
        // Group with children - enhanced styling
        view! {
            <div>
                <button 
                    class="w-full group flex items-center px-4 py-3 rounded-xl text-sm font-medium text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-all duration-200 border border-transparent hover:border-slate-200"
                    on:click=move |_| set_is_expanded.update(|expanded| *expanded = !*expanded)
                >
                    <Icon name=item.icon.clone() size=IconSize::Md class="mr-3 text-slate-500".to_string() />
                    <span class="flex-1 text-left">{item.title.clone()}</span>
                    <Icon 
                        name=if is_expanded.get() { "interface-chevron-down".to_string() } else { "interface-chevron-right".to_string() }
                        size=IconSize::Sm 
                        class="transition-transform duration-200 text-slate-400".to_string()
                    />
                </button>
                
                {move || {
                    if is_expanded.get() {
                        view! {
                            <div class="ml-8 mt-2 space-y-1 pl-4 border-l-2 border-slate-200">
                                {item.children.iter().map(|child| {
                                    view! {
                                        <HealthcareNavigationItemComponent item=child.clone() />
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
        // Section header
        view! {
            <div class="px-4 py-2 text-xs font-semibold text-slate-400 uppercase tracking-wider flex items-center">
                <Icon name=item.icon.clone() size=IconSize::Sm class="mr-2".to_string() />
                {item.title.clone()}
            </div>
        }.into_any()
    }
}
