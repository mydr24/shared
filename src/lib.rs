// Core modules
pub mod models;
pub mod errors;
#[cfg(feature = "post-quantum")]
pub mod auth;
pub mod compliance;
pub mod utils;
pub mod events;

// UI modules (feature-gated for frontend)
#[cfg(feature = "ui")]
pub mod ui;
#[cfg(feature = "ui")]
pub mod websocket_simple;
#[cfg(feature = "ui")]
pub mod chat_simple;
#[cfg(feature = "ui")]
pub mod emergency_simple;
#[cfg(feature = "ui")]
pub mod location_simple;

// Re-exports
pub use models::*;
pub use errors::*;
#[cfg(feature = "post-quantum")]
pub use auth::*;
pub use compliance::*;
pub use utils::*;
pub use events::*;

#[cfg(feature = "ui")]
pub use ui::*;
#[cfg(feature = "ui")]
pub use websocket_simple::*;
#[cfg(feature = "ui")]
pub use chat_simple::*;
#[cfg(feature = "ui")]
pub use emergency_simple::*;
#[cfg(feature = "ui")]
pub use location_simple::*;
