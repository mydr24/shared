use leptos::prelude::*;
use crate::ui::cn;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
    // Healthcare-specific variants
    Booking,
    Emergency,
    Call,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

#[component]
pub fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] loading: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static + Send>>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(ButtonVariant::Default);
    let size = size.unwrap_or(ButtonSize::Default);
    let disabled = disabled.unwrap_or(false);
    let loading = loading.unwrap_or(false);
    
    let base_classes = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
    
    let variant_classes = match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        // Healthcare-specific variants
        ButtonVariant::Booking => "bg-blue-600 text-white hover:bg-blue-700 focus:bg-blue-700",
        ButtonVariant::Emergency => "bg-red-600 text-white hover:bg-red-700 focus:bg-red-700 animate-pulse",
        ButtonVariant::Call => "bg-green-600 text-white hover:bg-green-700 focus:bg-green-700",
    };
    
    let size_classes = match size {
        ButtonSize::Default => "h-10 px-4 py-2",
        ButtonSize::Sm => "h-9 rounded-md px-3",
        ButtonSize::Lg => "h-11 rounded-md px-8",
        ButtonSize::Icon => "h-10 w-10",
    };
    
    let button_classes = cn(&[
        base_classes,
        variant_classes,
        size_classes,
        class.unwrap_or(""),
        if loading { "cursor-wait" } else { "" },
    ]);
    
    view! {
        <button 
            class=button_classes
            disabled=move || disabled || loading
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    if !disabled && !loading {
                        handler();
                    }
                }
            }
        >
        >
            {move || {
                if loading {
                    view! {
                        <svg class="mr-2 h-4 w-4 animate-spin" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            {children()}
        </button>
    }
}

// Specialized healthcare buttons
#[component]
pub fn EmergencyButton(
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static + Send>>,
    children: Children,
) -> impl IntoView {
    let emergency_classes = "bg-red-600 text-white hover:bg-red-700 border-2 border-red-800 shadow-lg animate-pulse";
    
    view! {
        <Button 
            variant=ButtonVariant::Destructive
            class=emergency_classes
            disabled=disabled.unwrap_or(false)
            on_click=on_click.unwrap_or_else(|| Box::new(|| {}))
        >
            <span class="mr-2">"🚨"</span>
            {children()}
        </Button>
    }
}

#[component]
pub fn CallButton(
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static + Send>>,
    children: Children,
) -> impl IntoView {
    let call_classes = "bg-green-600 text-white hover:bg-green-700 shadow-md";
    
    view! {
        <Button 
            variant=ButtonVariant::Default
            class=call_classes
            disabled=disabled.unwrap_or(false)
            on_click=on_click.unwrap_or_else(|| Box::new(|| {}))
        >
            <span class="mr-2">"📞"</span>
            {children()}
        </Button>
    }
}

#[component]
pub fn VideoCallButton(
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static + Send>>,
    children: Children,
) -> impl IntoView {
    let video_classes = "bg-blue-600 text-white hover:bg-blue-700 shadow-md";
    
    view! {
        <Button 
            variant=ButtonVariant::Default
            class=video_classes
            disabled=disabled.unwrap_or(false)
            on_click=on_click.unwrap_or_else(|| Box::new(|| {}))
        >
            <span class="mr-2">"📹"</span>
            {children()}
        </Button>
    }
}

#[component]
pub fn BookingButton(
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] loading: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static + Send>>,
    children: Children,
) -> impl IntoView {
    let booking_classes = "bg-indigo-600 text-white hover:bg-indigo-700 shadow-md";
    
    view! {
        <Button 
            variant=ButtonVariant::Default
            class=booking_classes
            disabled=disabled.unwrap_or(false)
            loading=loading.unwrap_or(false)
            on_click=on_click.unwrap_or_else(|| Box::new(|| {}))
        >
            <span class="mr-2">"📅"</span>
            {children()}
        </Button>
    }
}
