use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::{env};
use figment::providers::Env;
use tracing::{debug, debug_span, info, info_span};

#[derive(Deserialize)]
struct AppConfig
{
    test: String,
}

/// Configures whether logs are emitted to a file, to stdout or to open telemetry.
pub enum LogConfig
{
    File( std::path::PathBuf ),
    Stdout,
    OpenTelemetry,
}

fn main( )
{
    // Set environment type env variable.
    let environment_type = if cfg!( debug_assertions ) { "dev" } else { "prod" };
    env::set_var( "ENVIRONMENT_TYPE", environment_type );

    let configs_path = std::path::PathBuf::from( "./configs/logger/" );
    let app_name = env!("CARGO_PKG_NAME").to_string().to_uppercase();

    // Load app config variables.
    let _app_config = Figment::new()
        .merge( Toml::file( configs_path.join( "base.dev.toml" ) ) )
        .merge( Toml::file( configs_path.join( "base.prod.toml" ) ).profile( "prod" ) )
        .select( figment::Profile::from_env_or( "ENVIRONMENT_TYPE", "dev" ) )
        .merge( Env::prefixed(format!("{}_", app_name).as_str() ) )
        .extract::<AppConfig>()
        .expect( "Failed to load app config" );

    // Redirect logs from 'log' crate to 'tracing' crate.
    tracing_log::LogTracer::init().expect( "Failed to initialize logger" );

    let subscriber = tracing_subscriber::fmt().finish();

    tracing::subscriber::set_global_default( subscriber ).expect( "Failed to set global default subscriber" );

    // // Write logs to local file.
    // let file_appender = tracing_appender::rolling::hourly( "./logs", "prefix.log" );
    // let ( non_blocking_file_writer, _guard_file_writer ) = tracing_appender::non_blocking( file_appender );
    //
    // // Initialize the tracing subscriber.
    // let subscriber = tracing_subscriber::Registry::default()
    //     .with( tracing_subscriber::EnvFilter::new( "INFO" ) )
    //     .with( TracingLayer )
    //     .with(
    //         tracing_subscriber::fmt::layer()
    //             .with_writer( non_blocking_file_writer )
    //             .with_ansi( false ),
    //     );
    //
    // tracing::subscriber::set_global_default( subscriber ).expect( "Failed to set global default subscriber" );

    let outer_span = info_span!( "outer", level = 0, other_field = tracing::field::Empty );
    let _outer_entered = outer_span.enter();

    let inner_span = info_span!( "inner", level = 1, other_field = tracing::field::Empty );
    let _inner_entered = inner_span.enter();

    outer_span.record( "other_field", &7 );

    let inner_span = debug_span!( "inner", level = 1 );
    let _inner_guard = inner_span.enter();

    info!( message = "Hello, world!", a_bool = true, a_number = 42 );
    debug!( message = "Hello, world!", a_bool = true, a_number = 42 );
}
