#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::todo,
    clippy::panic
)]

// TODO: Remove unwrap_throw / expect_throw methods.

#[cfg(feature = "frontend")]
pub mod component;

pub mod api;
pub mod util;
pub mod specific;
pub mod error;

pub use specific::*;