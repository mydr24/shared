use leptos::prelude::*;
use crate::ui::{Icon, IconSize, Priority, HealthcareStatus};

#[derive(Debug, Clone)]
pub struct SimplePatientInfo {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub status: HealthcareStatus,
    pub last_visit: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SimpleHealthMetric {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub status: HealthcareStatus,
    pub icon: String,
}

#[component]
pub fn SimplePatientCard(
    #[prop()] patient: SimplePatientInfo,
) -> impl IntoView {
    let status_color = match patient.status {
        HealthcareStatus::Active => "bg-green-500",
        HealthcareStatus::Critical => "bg-red-500 animate-pulse",
        HealthcareStatus::NeedsAttention => "bg-yellow-500",
        _ => "bg-gray-500",
    };
    
    view! {
        <div class="bg-white rounded-xl border-2 border-blue-200 p-6 hover:shadow-lg transition-all duration-200 cursor-pointer">
            <div class="flex items-center space-x-4">
                <div class="relative">
                    <div class="w-12 h-12 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center border-2 border-white shadow-sm">
                        <span class="text-white font-bold">
                            {patient.name.chars().next().unwrap_or('P').to_string()}
                        </span>
                    </div>
                    <div class=format!("absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-white shadow-sm {}", status_color)></div>
                </div>
                
                <div class="flex-1 min-w-0">
                    <h3 class="font-semibold text-slate-900 truncate">{patient.name.clone()}</h3>
                    <p class="text-sm text-slate-600">{patient.age} years old</p>
                    <p class="text-xs text-slate-500 font-mono">ID: {patient.id.clone()}</p>
                </div>
                
                {
                    if let Some(last_visit) = &patient.last_visit {
                        view! {
                            <div class="text-right">
                                <p class="text-xs text-slate-500">Last visit</p>
                                <p class="text-sm font-medium text-slate-700">{last_visit.clone()}</p>
                            </div>
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
pub fn SimpleHealthMetricCard(
    #[prop()] metric: SimpleHealthMetric,
) -> impl IntoView {
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
            </div>
            
            <div class="flex items-baseline space-x-2">
                <span class=format!("text-2xl font-bold {}", status_colors.1)>
                    {metric.value.clone()}
                </span>
                <span class=format!("text-sm {}", status_colors.2)>
                    {metric.unit.clone()}
                </span>
            </div>
        </div>
    }
}

#[component]
pub fn SimpleEmergencyButton() -> impl IntoView {
    view! {
        <button class="w-full bg-gradient-to-r from-red-600 to-red-700 text-white px-8 py-6 rounded-2xl font-bold text-lg shadow-xl hover:from-red-700 hover:to-red-800 transition-all duration-200 transform hover:scale-105 active:scale-95 flex items-center justify-center">
            <Icon name="emergency-alert".to_string() size=IconSize::Xl class="text-white mr-3 animate-pulse".to_string() />
            <div class="text-left">
                <div class="font-bold">EMERGENCY ALERT</div>
                <div class="text-sm text-red-100">Tap for immediate help</div>
            </div>
        </button>
    }
}

#[component]
pub fn SimpleStatsCard(
    #[prop()] title: String,
    #[prop()] value: String,
    #[prop()] icon: String,
    #[prop(optional)] trend: Option<String>,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl border border-slate-200 p-6 hover:shadow-lg transition-shadow duration-200">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm font-medium text-slate-600">{title.clone()}</p>
                    <p class="text-3xl font-bold text-slate-900 mt-2">{value.clone()}</p>
                    {
                        if let Some(trend_text) = trend {
                            view! {
                                <p class="text-sm text-green-600 mt-1">{trend_text}</p>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }
                </div>
                <div class="p-3 bg-blue-100 rounded-full">
                    <Icon name=icon size=IconSize::Lg class="text-blue-600".to_string() />
                </div>
            </div>
        </div>
    }
}
