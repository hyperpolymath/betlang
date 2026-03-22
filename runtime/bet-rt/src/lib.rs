// SPDX-License-Identifier: MIT OR Apache-2.0
//! Betlang Runtime Library
//!
//! This crate provides the core runtime functionality for betlang:
//! - I/O operations (file, network, stdio)
//! - Data structures (arrays, maps, sets)
//! - Serialization (JSON, MessagePack, Arrow)
//! - Random number generation
//! - Parallel execution

#![forbid(unsafe_code)]
pub mod io;
pub mod data;
pub mod serial;
pub mod random;
pub mod parallel;
pub mod value;

pub use value::Value;
