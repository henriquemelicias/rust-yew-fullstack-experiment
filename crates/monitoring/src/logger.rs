use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_loki::BackgroundTask;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
use url::Url;

pub enum FilterTypes
{
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl FilterTypes
{
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

#[derive(Debug, Eq, PartialEq)]
pub enum OutputType
{
    File,
    Loki,
    Stdout,
    Wasm,
}

struct LogFileWriter;

impl std::io::Write for LogFileWriter
{
    fn write( &mut self, buf: &[u8] ) -> std::io::Result<usize>
    {
        println!( "{:?}", buf );
        Ok( buf.len() )
    }

    fn flush( &mut self ) -> std::io::Result<()> { Ok( () ) }
}

#[allow( clippy::missing_panics_doc )]
/// # Panic
///
/// Panic goes here.
pub fn init<P: AsRef<Path>>(
    level_filter: &FilterTypes,
    types_enabled: &[OutputType],
    file_dir: Option<P>,
    file_prefix: Option<P>,
    loki_url: Option<Url>,
) -> ( Option<WorkerGuard>, Option<WorkerGuard>, Option<BackgroundTask> )
{
    // Layers to be used.
    let mut layers = Vec::new();

    // Write logs to stdout.
    let mut guard_io_writer = None;
    if types_enabled.contains( &OutputType::Stdout )
    {
        let ( non_blocking_io_writer, guard ) = tracing_appender::non_blocking( std::io::stdout() );
        let stdout_layer = tracing_subscriber::fmt::layer().with_writer( non_blocking_io_writer );

        guard_io_writer = Some( guard );
        layers.push( stdout_layer.boxed() );
    }

    // Write to web wasm console.log .
    if types_enabled.contains( &OutputType::Wasm )
    {
        layers.push( tracing_wasm::WASMLayer::default().boxed() );
    }

    // Write logs to local file.
    let mut guard_file_writer = None;
    if types_enabled.contains( &OutputType::File )
    {
        let file_dir = file_dir.expect( "Logging file directory is not specified." );
        let file_prefix = file_prefix.expect( "Logging file prefix is not specified." );
        let file_appender = tracing_appender::rolling::hourly( file_dir, file_prefix );
        let ( non_blocking_file_writer, guard ) = tracing_appender::non_blocking( file_appender );

        let app_name = concat!( env!( "CARGO_PKG_NAME" ), "-", env!( "CARGO_PKG_VERSION" ) ).to_string();
        let file_layer = BunyanFormattingLayer::new( app_name, non_blocking_file_writer );

        guard_file_writer = Some( guard );
        layers.push( file_layer.boxed() );
    }

    // Write logs to open telemetry prometheus.
    let mut loki_task_writer = None;
    if types_enabled.contains( &OutputType::Loki )
    {
        let ( loki_layer, task ) = tracing_loki::layer(
            loki_url.unwrap(),
            vec![( "host".into(), "mine".into() )].into_iter().collect(),
            vec![].into_iter().collect(),
        )
        .unwrap();

        loki_task_writer = Some( task );
        layers.push( loki_layer.boxed() );
    }

    // Log level filter.
    let log_level_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else( |_| tracing_subscriber::EnvFilter::new( level_filter.value() ) );

    let register = tracing_subscriber::Registry::default()
        .with( layers )
        .with( log_level_filter );

    // Default subscriber.
    tracing::subscriber::set_global_default( register ).expect( "Failed to init global monitoring" );

    tracing::info!( "Initialized logging configuration with instrumentation" );
    ( guard_io_writer, guard_file_writer, loki_task_writer )
}
