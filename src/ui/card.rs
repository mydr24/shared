use leptos::prelude::*;
use crate::ui::cn;

#[component]
pub fn Card(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let card_classes = cn(&[
        "rounded-lg border bg-card text-card-foreground shadow-sm",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=card_classes>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let header_classes = cn(&[
        "flex flex-col space-y-1.5 p-6",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=header_classes>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let title_classes = cn(&[
        "text-2xl font-semibold leading-none tracking-tight",
        class.unwrap_or(""),
    ]);
    
    view! {
        <h3 class=title_classes>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let description_classes = cn(&[
        "text-sm text-muted-foreground",
        class.unwrap_or(""),
    ]);
    
    view! {
        <p class=description_classes>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let content_classes = cn(&[
        "p-6 pt-0",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=content_classes>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let footer_classes = cn(&[
        "flex items-center p-6 pt-0",
        class.unwrap_or(""),
    ]);
    
    view! {
        <div class=footer_classes>
            {children()}
        </div>
    }
}

// Healthcare-specific card variants
#[component]
pub fn PatientCard(
    patient_name: String,
    patient_id: String,
    #[prop(optional)] age: Option<u32>,
    #[prop(optional)] condition: Option<String>,
    #[prop(optional)] priority: Option<crate::ui::Priority>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let priority = priority.unwrap_or(crate::ui::Priority::Normal);
    let priority_color = match priority {
        crate::ui::Priority::Emergency => "border-red-500 bg-red-50",
        crate::ui::Priority::Urgent => "border-orange-500 bg-orange-50",
        crate::ui::Priority::Normal => "border-blue-200 bg-blue-50",
        crate::ui::Priority::Low => "border-green-200 bg-green-50",
    };
    
    view! {
        <Card class=priority_color>
            <CardHeader>
                <div class="flex items-center justify-between">
                    <CardTitle class="text-lg">
                        {patient_name}
                    </CardTitle>
                    <div class="text-sm text-muted-foreground">
                        "ID: " {patient_id}
                    </div>
                </div>
                {move || {
                    if let Some(age_val) = age {
                        view! {
                            <CardDescription>
                                "Age: " {age_val.to_string()} " years"
                            </CardDescription>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </CardHeader>
            {move || {
                if let Some(condition_val) = condition.clone() {
                    view! {
                        <CardContent>
                            <div class="flex items-center space-x-2">
                                <span class="text-sm font-medium">"Condition:"</span>
                                <span class="text-sm text-muted-foreground">{condition_val}</span>
                            </div>
                        </CardContent>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </Card>
    }
}

#[component]
pub fn ProviderCard(
    provider_name: String,
    specialty: String,
    #[prop(optional)] rating: Option<f32>,
    #[prop(optional)] availability: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] on_book: Option<Box<dyn Fn() + 'static + Send>>,
) -> impl IntoView {
    view! {
        <Card class="hover:shadow-md transition-shadow">
            <CardHeader>
                <div class="flex items-start justify-between">
                    <div>
                        <CardTitle class="text-lg">
                            {provider_name}
                        </CardTitle>
                        <CardDescription>
                            {specialty}
                        </CardDescription>
                    </div>
                    {move || {
                        if let Some(rating_val) = rating {
                            view! {
                                <div class="flex items-center space-x-1">
                                    <span class="text-sm text-yellow-500">"⭐"</span>
                                    <span class="text-sm font-medium">{format!("{:.1}", rating_val)}</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}
                </div>
            </CardHeader>
            {move || {
                if let Some(avail) = availability.clone() {
                    view! {
                        <CardContent>
                            <div class="flex items-center space-x-2">
                                <span class="text-sm font-medium">"Available:"</span>
                                <span class="text-sm text-green-600">{avail}</span>
                            </div>
                        </CardContent>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            {if on_book.is_some() {
                view! {
                    <CardFooter>
                        <crate::ui::button::BookingButton>
                            "Book Appointment"
                        </crate::ui::button::BookingButton>
                    </CardFooter>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </Card>
    }
}

#[component]
pub fn AppointmentCard(
    appointment_id: String,
    provider_name: String,
    date_time: String,
    #[prop(optional)] status: Option<String>,
    #[prop(optional)] appointment_type: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let status = status.unwrap_or_else(|| "Scheduled".to_string());
    let appointment_type = appointment_type.unwrap_or_else(|| "Consultation".to_string());
    
    let status_color = match status.as_str() {
        "Confirmed" | "Scheduled" => "text-green-600 bg-green-100",
        "Pending" => "text-yellow-600 bg-yellow-100",
        "Cancelled" => "text-red-600 bg-red-100",
        "Completed" => "text-blue-600 bg-blue-100",
        _ => "text-gray-600 bg-gray-100",
    };
    
    view! {
        <Card class="hover:shadow-md transition-shadow">
            <CardHeader>
                <div class="flex items-start justify-between">
                    <div>
                        <CardTitle class="text-lg">
                            {provider_name}
                        </CardTitle>
                        <CardDescription>
                            {date_time}
                        </CardDescription>
                    </div>
                    <div class=format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color)>
                        {status}
                    </div>
                </div>
            </CardHeader>
            <CardContent>
                <div class="flex items-center justify-between text-sm">
                    <span class="text-muted-foreground">"Type: " {appointment_type}</span>
                    <span class="text-muted-foreground">"ID: " {appointment_id}</span>
                </div>
            </CardContent>
        </Card>
    }
}

#[component]
pub fn StatsCard(
    title: String,
    value: String,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] icon: Option<String>,
    #[prop(optional)] trend: Option<String>,
    #[prop(optional)] change: Option<String>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <Card class="hover:shadow-md transition-shadow">
            <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle class="text-sm font-medium">
                    {title}
                </CardTitle>
                {move || {
                    if let Some(icon_val) = icon.clone() {
                        view! {
                            <span class="text-lg">{icon_val}</span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold">{value}</div>
                {move || {
                    if let Some(desc) = description.clone() {
                        view! {
                            <p class="text-xs text-muted-foreground">{desc}</p>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
                {move || {
                    if let Some(trend_val) = trend.clone() {
                        view! {
                            <p class="text-xs text-green-600 mt-1">{trend_val}</p>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
                {move || {
                    if let Some(change_val) = change.clone() {
                        view! {
                            <p class="text-xs text-muted-foreground mt-1">{change_val}</p>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </CardContent>
        </Card>
    }
}
