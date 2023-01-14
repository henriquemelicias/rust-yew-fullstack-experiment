#![feature( once_cell )]
#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

// Modules.
pub(crate) mod services;
pub mod settings;

// Crate use re-exports.
pub use color_eyre::eyre::Result;
