#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::todo,
    clippy::panic
)]

#[cfg(feature = "frontend")]
pub mod component;

pub mod api;
pub mod error;
pub mod specific;
pub mod util;

pub use specific::*;
