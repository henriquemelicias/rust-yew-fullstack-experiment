#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

// Modules.
pub(crate) mod app;
pub(crate) mod settings;

// Crate use re-exports.
pub use color_eyre::eyre::Result;
