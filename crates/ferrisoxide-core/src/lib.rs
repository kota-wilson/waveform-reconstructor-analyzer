//! Core library for FerrisOxide Signal.

pub mod analysis;
pub mod config;
pub mod criteria;
pub mod csv;
pub mod error;
pub mod event;
pub mod filter;
pub mod model;
pub mod report;
pub mod runtime_profile;

pub use error::{Result, WaveformError};
