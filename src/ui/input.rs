use leptos::prelude::*;
use crate::ui::cn;

#[component]
pub fn Input(
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
) -> impl IntoView {
    let input_type = input_type.unwrap_or("text");
    let disabled = disabled.unwrap_or(false);
    let required = required.unwrap_or(false);
    
    let input_classes = cn(&[
        "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
        class.unwrap_or(""),
    ]);
    
    view! {
        <input
            type=input_type
            class=input_classes
            placeholder=placeholder.unwrap_or("")
            value=value.unwrap_or_default()
            disabled=disabled
            required=required
            id=id.unwrap_or("")
            name=name.unwrap_or("")
            on:input=move |ev| {
                if let Some(handler) = &on_input {
                    let value = event_target_value(&ev);
                    handler(value);
                }
            }
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    let value = event_target_value(&ev);
                    handler(value);
                }
            }
        />
    }
}

#[component]
pub fn Label(
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] for_attr: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let label_classes = cn(&[
        "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
        class.unwrap_or(""),
    ]);
    
    view! {
        <label
            class=label_classes
            for=for_attr.unwrap_or("")
        >
            {children()}
        </label>
    }
}

// Healthcare-specific input components
#[component]
pub fn PatientIdInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <Label for_attr="patient-id">"Patient ID"</Label>
            <Input
                id="patient-id"
                name="patient_id"
                placeholder="Enter patient ID"
                value=value.unwrap_or_default()
                on_input=on_input.unwrap_or_else(|| Box::new(|_| {}))
                required=required.unwrap_or(false)
                class=class.unwrap_or("")
            />
        </div>
    }
}

#[component]
pub fn ProviderLicenseInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <Label for_attr="license-number">"License Number"</Label>
            <Input
                id="license-number"
                name="license_number"
                placeholder="Enter medical license number"
                value=value.unwrap_or_default()
                on_input=on_input.unwrap_or_else(|| Box::new(|_| {}))
                required=required.unwrap_or(false)
                class=class.unwrap_or("")
            />
        </div>
    }
}

#[component]
pub fn SearchInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="relative">
            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <svg class="h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m21 21-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                </svg>
            </div>
            <Input
                input_type="search"
                placeholder=placeholder.unwrap_or("Search...")
                value=value.unwrap_or_default()
                on_input=on_input.unwrap_or_else(|| Box::new(|_| {}))
                class="pl-10"
            />
        </div>
    }
}

#[component]
pub fn EmailInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <Label for_attr="email">"Email Address"</Label>
            <Input
                input_type="email"
                id="email"
                name="email"
                placeholder="Enter your email address"
                value=value.unwrap_or_default()
                on_input=on_input.unwrap_or_else(|| Box::new(|_| {}))
                required=required.unwrap_or(false)
                disabled=disabled.unwrap_or(false)
                class=class.unwrap_or("")
            />
        </div>
    }
}

#[component]
pub fn PasswordInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <Label for_attr="password">"Password"</Label>
            <Input
                input_type="password"
                id="password"
                name="password"
                placeholder=placeholder.unwrap_or("Enter your password")
                value=value.unwrap_or_default()
                on_input=on_input.unwrap_or_else(|| Box::new(|_| {}))
                required=required.unwrap_or(false)
                class=class.unwrap_or("")
            />
        </div>
    }
}

#[component]
pub fn PhoneInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <Label for_attr="phone">"Phone Number"</Label>
            <Input
                input_type="tel"
                id="phone"
                name="phone"
                placeholder="+91 9876543210"
                value=value.unwrap_or_default()
                on_input=on_input.unwrap_or_else(|| Box::new(|_| {}))
                required=required.unwrap_or(false)
                class=class.unwrap_or("")
            />
        </div>
    }
}

#[component]
pub fn DateInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] min: Option<&'static str>,
    #[prop(optional)] max: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
) -> impl IntoView {
    let label_text = label.unwrap_or("Date");
    
    view! {
        <div class="space-y-2">
            <Label for_attr="date">{label_text}</Label>
            <Input
                input_type="date"
                id="date"
                name="date"
                value=value.unwrap_or_default()
                on_input=on_input.unwrap_or_else(|| Box::new(|_| {}))
                required=required.unwrap_or(false)
                class=class.unwrap_or("")
            />
        </div>
    }
}

#[component]
pub fn TimeInput(
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static + Send>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
) -> impl IntoView {
    let label_text = label.unwrap_or("Time");
    
    view! {
        <div class="space-y-2">
            <Label for_attr="time">{label_text}</Label>
            <Input
                input_type="time"
                id="time"
                name="time"
                value=value.unwrap_or_default()
                on_input=on_input.unwrap_or_else(|| Box::new(|_| {}))
                required=required.unwrap_or(false)
                class=class.unwrap_or("")
            />
        </div>
    }
}
