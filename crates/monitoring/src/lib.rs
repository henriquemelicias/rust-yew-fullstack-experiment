//! Monitoring tools with tracing logger and prometheus metrics.
//!
//! Provides an implementation of a tracing logger and metrics collection for prometheus.

#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]
#![feature( once_cell )]

pub mod logger;
pub mod prometheus;
