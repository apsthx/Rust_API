// Structs module - Request/Response DTOs
// Equivalent to Go's structs/ directory

pub mod auth;
pub mod user;
pub mod order;
pub mod customer;
pub mod common;

// Re-export commonly used structs
pub use auth::*;
pub use user::*;
pub use order::*;
pub use customer::*;
pub use common::*;
