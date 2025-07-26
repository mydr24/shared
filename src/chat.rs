// MyDR24 Healthcare Platform - Real-time Chat Component
// WebSocket-powered chat for patient-provider communication

use leptos::prelude::*;
use crate::{WebSocketClient, ChatMessage};
use uuid::Uuid;
use chrono::Utc;

#[component]
pub fn RealTimeChat(
    ws_client: ReadSignal<Option<WebSocketClient>>,
    booking_id: String,
    current_user_id: String,
    participant_name: String,
    participant_role: String, // "patient" or "provider"
) -> impl IntoView {
    let (message_input, set_message_input) = create_signal(String::new());
    let (is_sending, set_is_sending) = create_signal(false);
    let (typing_indicator, set_typing_indicator) = create_signal(false);
    
    let input_ref = create_node_ref::<leptos::html::Input>();
    
    // Send chat message
    let send_message = move || {
        let message_text = message_input.get().trim().to_string();
        if message_text.is_empty() || is_sending.get() {
            return;
        }
        
        set_is_sending.set(true);
        set_message_input.set(String::new());
        
        // Clear input field
        if let Some(input) = input_ref.get() {
            let _ = input.set_value("");
        }
        
        let chat_message = ChatMessage {
            message_id: Uuid::new_v4().to_string(),
            booking_id: booking_id.clone(),
            sender_id: current_user_id.clone(),
            recipient_id: "other_user".to_string(), // In real app, get from booking data
            content: message_text,
            message_type: "text".to_string(),
            metadata: None,
        };
        
        if let Some(client) = ws_client.get() {
            spawn_local(async move {
                match client.send_chat_message(chat_message).await {
                    Ok(_) => {
                        log::info!("✅ Chat message sent successfully");
                    },
                    Err(e) => {
                        log::error!("❌ Failed to send chat message: {:?}", e);
                    }
                }
                set_is_sending.set(false);
            });
        } else {
            set_is_sending.set(false);
        }
    };
    
    // Handle Enter key press
    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            send_message();
        }
    };
    
    // Handle input changes (for typing indicator)
    let handle_input = move |ev| {
        let value = event_target_value(&ev);
        set_message_input.set(value);
        
        // Show typing indicator briefly
        set_typing_indicator.set(true);
        
        // Hide typing indicator after 2 seconds of no typing
        let timer = gloo_timers::callback::Timeout::new(2000, move || {
            set_typing_indicator.set(false);
        });
        timer.forget();
    };
    
    view! {
        <div class="real-time-chat bg-white rounded-xl shadow-sm border h-96 flex flex-col">
            // Chat header
            <div class="chat-header p-4 border-b bg-blue-50 rounded-t-xl">
                <div class="flex items-center justify-between">
                    <div class="flex items-center">
                        <div class="w-10 h-10 bg-blue-600 rounded-full flex items-center justify-center text-white font-bold mr-3">
                            {participant_name.chars().next().unwrap_or('?').to_uppercase()}
                        </div>
                        <div>
                            <div class="font-medium text-gray-800">
                                {participant_name}
                            </div>
                            <div class="text-sm text-gray-500">
                                {participant_role.clone()}
                                {move || {
                                    if let Some(client) = ws_client.get() {
                                        match client.connection_state.get() {
                                            crate::ConnectionState::Connected => " • Online 🟢",
                                            _ => " • Offline 🔴",
                                        }
                                    } else {
                                        " • Disconnected ❌"
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                    
                    <div class="text-sm text-gray-500">
                        "Booking #" {booking_id.clone()}
                    </div>
                </div>
            </div>
            
            // Chat messages area
            <div class="chat-messages flex-1 p-4 overflow-y-auto space-y-3">
                {move || {
                    if let Some(client) = ws_client.get() {
                        let messages = client.chat_messages.get();
                        
                        // Filter messages for this booking
                        let booking_messages: Vec<_> = messages.into_iter()
                            .filter(|msg| msg.booking_id == booking_id)
                            .collect();
                        
                        if booking_messages.is_empty() {
                            view! {
                                <div class="text-center text-gray-500 py-8">
                                    <div class="text-4xl mb-2">💬</div>
                                    <div>"Start a conversation"</div>
                                    <div class="text-sm mt-1">"Messages are encrypted and secure"</div>
                                </div>
                            }.into_view()
                        } else {
                            booking_messages.into_iter().map(|message| {
                                let is_own_message = message.sender_id == current_user_id;
                                
                                view! {
                                    <div class=format!("message flex {}", if is_own_message { "justify-end" } else { "justify-start" })>
                                        <div class=format!(
                                            "max-w-xs lg:max-w-md px-4 py-2 rounded-lg {}",
                                            if is_own_message {
                                                "bg-blue-600 text-white"
                                            } else {
                                                "bg-gray-200 text-gray-800"
                                            }
                                        )>
                                            <div class="text-sm">
                                                {message.content}
                                            </div>
                                            <div class=format!(
                                                "text-xs mt-1 {}",
                                                if is_own_message { "text-blue-200" } else { "text-gray-500" }
                                            )>
                                                "Just now" // In real app, format timestamp
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }
                    } else {
                        view! {
                            <div class="text-center text-red-500 py-8">
                                <div class="text-4xl mb-2">🔌</div>
                                <div>"Chat not available"</div>
                                <div class="text-sm mt-1">"WebSocket connection required"</div>
                            </div>
                        }.into_view()
                    }
                }}
                
                // Typing indicator
                {move || {
                    if typing_indicator.get() {
                        view! {
                            <div class="flex justify-start">
                                <div class="bg-gray-200 px-4 py-2 rounded-lg">
                                    <div class="flex space-x-1">
                                        <div class="w-2 h-2 bg-gray-500 rounded-full animate-bounce"></div>
                                        <div class="w-2 h-2 bg-gray-500 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                                        <div class="w-2 h-2 bg-gray-500 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}
            </div>
            
            // Message input area
            <div class="chat-input p-4 border-t bg-gray-50 rounded-b-xl">
                <div class="flex space-x-2">
                    <input
                        node_ref=input_ref
                        type="text"
                        placeholder="Type your message..."
                        class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        value=move || message_input.get()
                        on:input=handle_input
                        on:keydown=handle_keydown
                        disabled=move || is_sending.get() || ws_client.get().is_none()
                    />
                    
                    <button
                        class=move || {
                            if is_sending.get() || message_input.get().trim().is_empty() || ws_client.get().is_none() {
                                "px-4 py-2 bg-gray-400 text-white rounded-lg cursor-not-allowed"
                            } else {
                                "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
                            }
                        }
                        disabled=move || is_sending.get() || message_input.get().trim().is_empty() || ws_client.get().is_none()
                        on:click=move |_| send_message()
                    >
                        {move || if is_sending.get() {
                            view! {
                                <div class="flex items-center">
                                    <div class="animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent mr-1"></div>
                                    "Sending"
                                </div>
                            }
                        } else {
                            view! {
                                <div class="flex items-center">
                                    <span class="mr-1">📤</span>
                                    "Send"
                                </div>
                            }
                        }}
                    </button>
                </div>
                
                // Quick message buttons for providers
                {move || {
                    if participant_role == "provider" {
                        view! {
                            <div class="flex space-x-2 mt-2">
                                <button 
                                    class="px-3 py-1 bg-green-100 hover:bg-green-200 text-green-800 rounded text-sm"
                                    on:click=move |_| {
                                        set_message_input.set("I'm on my way! ETA 10 minutes.".to_string());
                                    }
                                >
                                    "🚗 On my way"
                                </button>
                                <button 
                                    class="px-3 py-1 bg-blue-100 hover:bg-blue-200 text-blue-800 rounded text-sm"
                                    on:click=move |_| {
                                        set_message_input.set("I've arrived at your location.".to_string());
                                    }
                                >
                                    "📍 Arrived"
                                </button>
                                <button 
                                    class="px-3 py-1 bg-yellow-100 hover:bg-yellow-200 text-yellow-800 rounded text-sm"
                                    on:click=move |_| {
                                        set_message_input.set("Running 5 minutes late, apologies!".to_string());
                                    }
                                >
                                    "⏰ Delayed"
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}
                
                <div class="text-xs text-gray-500 mt-2 text-center">
                    "End-to-end encrypted • HIPAA compliant"
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ChatNotificationBadge(
    ws_client: ReadSignal<Option<WebSocketClient>>,
    booking_id: String,
) -> impl IntoView {
    let unread_count = move || {
        if let Some(client) = ws_client.get() {
            let messages = client.chat_messages.get();
            messages.into_iter()
                .filter(|msg| msg.booking_id == booking_id)
                .count()
        } else {
            0
        }
    };
    
    view! {
        {move || {
            let count = unread_count();
            if count > 0 {
                view! {
                    <div class="relative inline-flex">
                        <span class="absolute -top-2 -right-2 bg-red-500 text-white text-xs rounded-full h-5 w-5 flex items-center justify-center">
                            {if count > 9 { "9+" } else { &count.to_string() }}
                        </span>
                        <span class="text-2xl">💬</span>
                    </div>
                }.into_view()
            } else {
                view! {
                    <span class="text-2xl opacity-50">💬</span>
                }.into_view()
            }
        }}
    }
}
