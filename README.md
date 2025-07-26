# mydr24-shared

[![Crates.io](https://img.shields.io/crates/v/mydr24-shared.svg)](https://crates.io/crates/mydr24-shared)
[![Documentation](https://docs.rs/mydr24-shared/badge.svg)](https://docs.rs/mydr24-shared)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

MyDR24 Shared Healthcare Library - Common types, models, UI components, and utilities for healthcare applications built with Leptos and Rust.

## Overview

This library provides the core foundation for the MyDR24 healthcare platform, including:

- **Healthcare Models**: Patient, Provider, Organization data structures
- **UI Components**: Leptos-based healthcare-specific UI components
- **Compliance Utilities**: HIPAA, GDPR, and Indian healthcare regulation helpers
- **WebSocket Support**: Real-time communication utilities
- **Emergency Services**: Location-based emergency features

## Features

### 🏥 Healthcare Models
- Patient management structures
- Provider and organization profiles
- Appointment and booking systems
- Medical record types
- Compliance-ready data structures

### 🎨 UI Components
- Healthcare-specific buttons (Emergency, Call, Booking)
- Medical cards and status displays
- Form inputs with validation
- Progress indicators and badges
- Responsive healthcare layouts

### 🔒 Compliance & Security
- HIPAA compliance validation
- GDPR data protection utilities
- Indian NMC/MCI provider verification
- Post-quantum cryptography support
- Secure data handling patterns

### 🌐 Real-time Features
- WebSocket connection management
- Chat and messaging utilities
- Emergency alert systems
- Location-based services
- Live updates and notifications

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mydr24-shared = "0.1.0"
```

For Leptos applications, ensure you have the required features:

```toml
[dependencies]
mydr24-shared = { version = "0.1.0", features = ["leptos"] }
leptos = { version = "0.8", features = ["csr"] }
```

## Usage

### Basic Models

```rust
use mydr24_shared::{Patient, Provider, PatientStatus};

// Create a patient record
let patient = Patient {
    id: "patient-123".to_string(),
    name: "John Doe".to_string(),
    status: PatientStatus::Stable,
    // ... other fields
};
```

### UI Components

```rust
use leptos::*;
use mydr24_shared::ui::{Button, ButtonVariant, EmergencyButton};

#[component]
fn HealthcareApp() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Emergency>
            "Emergency Call"
        </Button>
        
        <EmergencyButton 
            patient_id="patient-123".to_string()
            on_emergency=|id| {
                // Handle emergency
            }
        />
    }
}
```

### Compliance Utilities

```rust
use mydr24_shared::compliance::{validate_hipaa_compliance, verify_nmc_registration};

// Validate HIPAA compliance
match validate_hipaa_compliance(&patient_data) {
    Ok(()) => println!("HIPAA compliant"),
    Err(e) => println!("Compliance error: {}", e),
}

// Verify healthcare provider
match verify_nmc_registration(&provider) {
    Ok(true) => println!("Provider verified"),
    Ok(false) => println!("Provider not verified"),
    Err(e) => println!("Verification error: {}", e),
}
```

### WebSocket Communication

```rust
use mydr24_shared::websocket_simple::WebSocketManager;

let ws_manager = WebSocketManager::new("wss://api.mydr24.com/ws".to_string());
ws_manager.connect().await?;
ws_manager.send_message("Hello from healthcare app!".to_string()).await?;
```

## Architecture

This library is designed to be used across all MyDR24 applications:

- **Backend API**: Data models and validation utilities
- **Admin Web**: Administrative UI components and workflows
- **Patient App**: Patient-focused UI and real-time features
- **Provider App**: Provider-specific components and utilities

## Platform Support

- **Web**: Full WASM support via Leptos
- **Desktop**: Tauri-compatible for native applications
- **Mobile**: iOS and Android via Tauri Mobile
- **Server**: Backend utilities for Rocket applications

## Compliance & Security

This library is built with healthcare compliance in mind:

- **HIPAA**: Health Insurance Portability and Accountability Act
- **GDPR**: General Data Protection Regulation
- **NMC**: National Medical Commission (India)
- **MCI**: Medical Council of India standards

## Development

### Prerequisites

- Rust 1.70+
- `wasm-pack` for WASM builds
- Node.js for frontend tooling

### Building

```bash
# Standard build
cargo build

# WASM build for web
wasm-pack build --target web

# Run tests
cargo test

# Check documentation
cargo doc --open
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests and documentation
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Healthcare Disclaimer

This software is provided as-is and should not be used as a substitute for professional medical advice, diagnosis, or treatment. Always seek the advice of qualified healthcare providers.

## Support

For issues and support:
- GitHub Issues: [mydr24/mydr24-shared/issues](https://github.com/mydr24/mydr24-shared/issues)
- Documentation: [docs.rs/mydr24-shared](https://docs.rs/mydr24-shared)
- Email: dev@mydr24.com
