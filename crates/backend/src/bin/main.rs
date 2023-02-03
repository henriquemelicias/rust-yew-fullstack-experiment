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

use axum::{
    body::{Body, StreamBody},
    extract::{Query, State},
    handler::Handler,
    http::Request,
};
use futures::stream::{self, StreamExt};
use std::convert::Infallible;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};

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

    /// Set the assets files directory
    #[clap( long = "assets-dir", default_value = settings::SERVER.assets_dir().as_str() )]
    assets_dir: SmartString,
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

#[derive(Clone)]
struct YewRendererState
{
    index_html_before: String,
    index_html_after:  String,
}

async fn render_yew_app( State( state ): State<YewRendererState>, req: Request<Body> ) -> impl IntoResponse
{
    let req_url = req.uri().path().to_string();
    let req_queries: Vec<( String, String )> = qstring::QString::from( req.uri().query().unwrap_or( "" ) ).into();

    let renderer = yew::ServerRenderer::<frontend::ServerApp>::with_props( move || frontend::ServerAppProps {
        request_data: frontend::RequestData {
            url:     req_url,
            queries: req_queries,
        },
    } );

    StreamBody::new(
        stream::once( async move { state.index_html_before } )
            .chain( renderer.render_stream() )
            .chain( stream::once( async move { state.index_html_after } ) )
            .map( Result::<_, Infallible>::Ok ),
    )
}

#[tokio::main]
async fn start_server( cli_args: &CliArgs )
{
    // Get index file.
    let index_html_s = tokio::fs::read_to_string( format!( "{}/index.html", cli_args.static_dir ) )
        .await
        .expect( "failed to read index.html" );
    let ( index_html_before, index_html_after ) = index_html_s.split_once( "<body>" ).unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str( "<body>" );

    // Create yew render state.
    let index_html_after = index_html_after.to_owned();
    let state = YewRendererState {
        index_html_before,
        index_html_after,
    };

    let br_compression = CompressionLayer::new().br( true ).no_gzip().no_deflate();

    // Create render.
    let renderer = render_yew_app.layer( br_compression.clone() ).with_state( state );

    // Robot.txt file get service.
    let robots_file = get_service( ServeFile::new( format!( "{}/robots.txt", cli_args.assets_dir ) ) )
        .layer( br_compression.clone() )
        .handle_error( handle_error );

    // Static files directory get service.
    let serve_static_dir =
        get_service( ServeDir::new( cli_args.static_dir.as_str() ).precompressed_br() ).handle_error( handle_error );

    // Assets files directory get service.
    let serve_assets_dir = get_service( ServeDir::new( cli_args.assets_dir.as_str() ) ).handle_error( handle_error );

    // Routes.
    let app = axum::Router::new()
        .route( "/api/hello", get( hello ).layer( br_compression.clone() ) )
        .route( "/robots.txt", robots_file )
        .nest_service( "/static", serve_static_dir.clone() )
        .nest_service( "/assets", serve_assets_dir.clone() )
        .fallback_service( renderer );

    // Http tracing logs middleware layer.
    let app = logger::middleware_http_tracing( app );

    // Serve server.
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
