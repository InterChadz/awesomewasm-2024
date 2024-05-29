#![warn(clippy::unwrap_used, clippy::expect_used)]

pub mod msg;
pub mod instantiate;
pub mod error;
pub mod execute;
pub mod query;
pub mod state;
pub mod sudo;
pub mod reply;

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod testing;