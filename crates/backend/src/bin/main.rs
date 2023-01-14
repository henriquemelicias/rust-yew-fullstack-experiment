use axum::{
    http,
    response::IntoResponse,
    routing::{get, get_service},
};
use backend::{settings, Result};
use clap::Parser;
use monitoring::logger;
use smartstring::alias::String as SmartString;
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use tower_http::services::{ServeDir, ServeFile};

// Command line arguments interface.
#[derive(Parser, Debug)]
#[clap( name = settings::GENERAL.app_name().as_str(), about = settings::GENERAL.about().as_str() )]
struct CliArgs
{
    /// Set the listen addr.
    #[clap( short = 'a', long = "addr", default_value = settings::SERVER.addr().as_str() )]
    addr: SmartString,

    /// Set the listen port.
    #[clap( short = 'p', long = "port", default_value_t =  *settings::SERVER.port() )]
    port: u16,

    /// Set the log level.
    /// Possible values: trace, debug, info, warn, error.
    #[clap( short = 'l', long = "log-level", default_value = settings::LOGGER.log_level().as_str() )]
    log_level: SmartString,

    /// Set the static files directory
    #[clap( short = 's', long = "static-dir", default_value = settings::SERVER.static_dir().as_str() )]
    static_dir: SmartString,
}

fn main() -> Result<()>
{
    // Enable color_eyre.
    color_eyre::install()?;

    // Parse the command line arguments.
    let cli_args = CliArgs::parse();

    // Tracing logs.
    let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard ) = start_logs( &cli_args );

    tracing::info!( "Starting backend" );

    start_server( &cli_args );

    Ok( () )
}

fn start_logs( cli_args: &CliArgs ) -> ( Option<logger::WorkerGuard>, Option<logger::WorkerGuard> )
{
    let mut log_output_types = Vec::new();

    if *settings::LOGGER.is_stdout_emitted()
    {
        log_output_types.push( logger::OutputType::Stdout );
    }

    if *settings::LOGGER.is_file_emitted()
    {
        log_output_types.push( logger::OutputType::File {
            app_name:  settings::GENERAL.app_name(),
            directory: settings::LOGGER
                .files_directory()
                .as_ref()
                .expect( "Failed to get logger files directory" ),
            prefix:    settings::LOGGER
                .files_prefix()
                .as_ref()
                .expect( "Failed to get logger files prefix" ),
        } )
    }

    logger::init(
        &logger::Level::from_str( &cli_args.log_level ).expect( "Failed to parse log level" ),
        &log_output_types,
    )
}

#[tokio::main]
async fn start_server( cli_args: &CliArgs )
{
    let serve_index = ServeFile::new( format!( "{}/index.html", cli_args.static_dir ) ).precompressed_gzip();
    let serve_dir = get_service(
        ServeDir::new( cli_args.static_dir.as_str() )
            .precompressed_gzip()
            .not_found_service( serve_index.clone() ),
    )
    .handle_error( handle_error );
    let serve_index = get_service( serve_index ).handle_error( handle_error );

    let app = axum::Router::new()
        .route( "/api/hello", get( hello ) )
        .nest_service( "/static", serve_dir.clone() )
        .fallback_service( serve_index );

    // Http tracing logs middleware layer.
    let app = logger::middleware_http_tracing( app );

    let sock_addr = SocketAddr::from( (
        IpAddr::from_str( cli_args.addr.as_str() ).unwrap_or( IpAddr::V6( Ipv6Addr::LOCALHOST ) ),
        cli_args.port,
    ) );

    tracing::info!( "Listening on https://{}", sock_addr );

    axum::Server::bind( &sock_addr )
        .serve( app.into_make_service() )
        .await
        .expect( "Unable to start server" );
}

async fn handle_error( _err: std::io::Error ) -> impl IntoResponse
{
    ( http::StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..." )
}

async fn hello() -> impl IntoResponse { "hello from the backend!" }
