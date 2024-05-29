#![warn(clippy::unwrap_used, clippy::expect_used)]

pub mod error;
pub mod execute;
pub mod instantiate;
pub mod msg;
pub mod query;
pub mod reply;
pub mod state;
pub mod sudo;

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod testing;

