#![feature( once_cell )]
#![deny( clippy::all )]
#![warn( clippy::pedantic )]
#![warn( clippy::nursery )]
#![warn( clippy::complexity )]
#![warn( clippy::perf )]

// Modules.
pub mod services;
pub mod settings;

// Crate use re-exports.
pub use color_eyre::eyre::Result;

use monitoring::logger;

use axum::{
    body::{Body, StreamBody},
    extract::State,
    handler::Handler,
    http,
    http::Request,
    response::IntoResponse,
    routing::{get, get_service},
};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use futures::stream::{self, StreamExt};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};

pub fn start_logs( log_level: &str ) -> ( Option<logger::WorkerGuard>, Option<logger::WorkerGuard> )
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
        &logger::Level::from_str( log_level ).expect( "Failed to parse log level" ),
        &log_output_types,
    )
}

#[cfg( feature = "ssr" )]
#[derive(Clone)]
struct YewRendererState
{
    index_html_before: String,
    index_html_after:  String,
}

#[cfg( feature = "ssr" )]
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

#[cfg( feature = "ssr" )]
async fn get_yew_render_state( static_dir: &str ) -> YewRendererState
{
    // Get index file.
    let index_html_s = tokio::fs::read_to_string( format!( "{}/index.html", static_dir ) )
        .await
        .expect( "Failed to read index.html" );
    let ( index_html_before, index_html_after ) = index_html_s.split_once( "<body>" ).unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str( "<body>" );

    let index_html_after = index_html_after.to_owned();

    let state = YewRendererState {
        index_html_before,
        index_html_after,
    };

    state
}

#[tokio::main]
pub async fn start_server( addr: &str, port: u16, static_dir: &str, assets_dir: &str )
{
    let br_compression = CompressionLayer::new().br( true ).no_gzip().no_deflate();

    // Api router.
    let mut app = axum::Router::new().route( "/api/hello", get( hello ).layer( br_compression.clone() ) );

    #[cfg( feature = "ssr" )]
    {
        // Yew render service for SSR.
        let state = get_yew_render_state( static_dir ).await;
        let renderer = render_yew_app.layer( br_compression.clone() ).with_state( state );

        // Robot.txt file get service.
        let robots_file = get_service( ServeFile::new( format!( "{}/robots.txt", assets_dir ) ) )
            .layer( br_compression.clone() )
            .handle_error( handle_error );

        // Static files directory get service.
        let serve_static_dir =
            get_service( ServeDir::new( static_dir ).precompressed_br() ).handle_error( handle_error );

        // Assets files directory get service.
        let serve_assets_dir =
            get_service( ServeDir::new( assets_dir ).precompressed_br() ).handle_error( handle_error );

        // Routes.
        app = app
            .route( "/robots.txt", robots_file )
            .nest_service( "/static", serve_static_dir.clone() )
            .nest_service( "/assets", serve_assets_dir.clone() )
            .fallback_service( renderer );
    }

    // Http tracing logs middleware layer.
    let app = logger::middleware_http_tracing( app );

    // Serve server.
    let sock_addr = SocketAddr::from( (
        IpAddr::from_str( addr ).unwrap_or( IpAddr::V6( Ipv6Addr::LOCALHOST ) ),
        port,
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
