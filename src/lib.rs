//! vanitygen — Bitcoin vanity address generator library.
//!
//! All public modules are re-exported here for both the binary and
//! potential external consumers.

pub mod address;
pub mod benchmark;
pub mod checkpoint;
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod log;
pub mod mnemonic;
pub mod notify;
pub mod search;
pub mod self_test;
pub mod style;
pub mod verify;
pub mod wif;
