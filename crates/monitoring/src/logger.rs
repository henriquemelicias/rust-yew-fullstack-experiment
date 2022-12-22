//! Tracing logger.
//!
//! Provides a tracing logger that can be used to log events and spans to user enabled options:
//! * stdout
//! * file
//! * wasm
//!
//! # Examples
//!
//! ```
//! use monitoring::logger;
//! use monitoring::logger::{Level, OutputType};
//!
//! // Initialize the logger with the desired options. The guards returned by this function must be
//! // kept alive for the duration of the program.
//! let ( _maybe_stdout_writer_guard, _maybe_file_writer_guard  ) = logger::init(
//!     &Level::Info,
//!     &[OutputType::Stdout, OutputType::File { app_name: "monitoring", directory: "../../logs", prefix: "doc.tests"}]
//! );
//! ```

use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{layer::SubscriberExt, Layer};

/// The log level.
///
/// Order of the levels, from least to most verbose:
///
/// Error > Warn > Info > Debug > Trace
///
pub enum Level
{
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level
{
    /// Returns the log level representation as a string.
    const fn value( &self ) -> &str
    {
        match self
        {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

/// Output types for the logs.
pub enum OutputType<'a>
{
    /// Output to a file.
    File
    {
        /// The name of the application.
        app_name:  &'a str,
        /// The directory to create the file in.
        directory: &'a str,
        /// The prefix to use for the file name.
        prefix:    &'a str,
    },
    /// Output to stdout.
    Stdout,
    /// Output to wasm ( console.log ).
    Wasm,
}

/// Initializes the logger with the given options.
///
/// # Arguments
///
/// * `level_filter` - The filter to use. Any filter level below this will be ignored.
/// * `output_types` - The outputs to use.
///
/// # Returns
///
/// A tuple of the stdout writer guard and the file writer guard. These guards must be kept alive
/// for the duration of the program.
///
/// # Examples
///
/// see [`crate::logger`] for an example.
///
pub fn init( level_filter: &Level, output_types_enabled: &[OutputType] )
    -> ( Option<WorkerGuard>, Option<WorkerGuard> )
{
    // Layers to be used.
    let mut layers = Vec::new();

    let mut guard_io_writer = None;
    let mut guard_file_writer = None;

    // Check output_types_enabled.
    for output_type in output_types_enabled
    {
        match output_type
        {
            // Write logs to stdout.
            OutputType::Stdout =>
            {
                let ( non_blocking_io_writer, guard ) = tracing_appender::non_blocking( std::io::stdout() );
                let stdout_layer = tracing_subscriber::fmt::layer().with_writer( non_blocking_io_writer );

                guard_io_writer = Some( guard );
                layers.push( stdout_layer.boxed() );
            }
            // Write logs to local file.
            OutputType::File {
                app_name,
                directory,
                prefix,
            } =>
            {
                let file_appender = tracing_appender::rolling::hourly( directory, prefix );
                let ( non_blocking_file_writer, guard ) = tracing_appender::non_blocking( file_appender );

                // TODO: Change into tracing_subscriber::fmt::format::Json when stable.
                let file_layer = BunyanFormattingLayer::new( (*app_name).to_string(), non_blocking_file_writer );

                guard_file_writer = Some( guard );
                layers.push( file_layer.boxed() );
            }
            // Write logs to wasm console.
            OutputType::Wasm =>
            {
                layers.push( tracing_wasm::WASMLayer::default().boxed() );
            }
        }
    }

    // Log level filter.
    let log_level_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else( |_| tracing_subscriber::EnvFilter::new( level_filter.value() ) );

    // Register layers to registry.
    let register = tracing_subscriber::Registry::default()
        .with( layers )
        .with( log_level_filter );

    // Default subscriber.
    tracing::subscriber::set_global_default( register ).expect( "Failed to init global monitoring" );

    tracing::info!( "Initialized logging configuration with instrumentation" );
    ( guard_io_writer, guard_file_writer )
}
