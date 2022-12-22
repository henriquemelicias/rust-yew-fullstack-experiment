use futures::StreamExt;
use lazy_static::lazy_static;
use monitoring::logger::{init, Level, OutputType};
use prometheus::{Encoder, TextEncoder};
use rand::{thread_rng, Rng};
use std::time::Duration;

use monitoring::prometheus::{ 
    add_metrics_to_registry, METRIC_CONNECTED_CLIENTS, METRIC_INCOMING_REQUESTS, METRIC_RESPONSE_CODE_COLLECTOR,
    METRIC_RESPONSE_TIME_COLLECTOR,
};
use tracing::{debug, debug_span, error, info, info_span, trace, warn};
use tracing_subscriber::filter::FilterExt;
use url::Url;
use warp::{ws::WebSocket, Filter, Rejection, Reply};

async fn some_handler() -> Result<impl Reply, Rejection>
{
    METRIC_INCOMING_REQUESTS.inc();
    Ok( "hello!" )
}

async fn ws_handler( ws: warp::ws::Ws, id: String ) -> Result<impl Reply, Rejection>
{
    Ok( ws.on_upgrade( move |socket| client_connection( socket, id ) ) )
}

async fn client_connection( ws: WebSocket, id: String )
{
    let ( _client_ws_sender, mut client_ws_rcv ) = ws.split();

    METRIC_CONNECTED_CLIENTS.inc();
    println!( "{} connected", id );

    while let Some( result ) = client_ws_rcv.next().await
    {
        match result
        {
            Ok( msg ) => println!( "received message: {:?}", msg ),
            Err( e ) =>
            {
                eprintln!( "error receiving ws message for id: {}): {}", id.clone(), e );
                break;
            }
        };
    }

    println!( "{} disconnected", id );
    METRIC_CONNECTED_CLIENTS.dec();
}

const ENVS: &'static [&'static str] = &["testing", "production"];
async fn data_collector()
{
    let mut collect_interval = tokio::time::interval( Duration::from_millis( 10 ) );
    loop
    {
        collect_interval.tick().await;
        let mut rng = thread_rng();
        let response_time: f64 = rng.gen_range( 0.001..10.0 );
        let response_code: usize = rng.gen_range( 100..599 );
        let env_index: usize = rng.gen_range( 0..2 );

        track_status_code( response_code, ENVS.get( env_index ).expect( "exists" ) );
        track_request_time( response_time, ENVS.get( env_index ).expect( "exists" ) )
    }
}

fn track_request_time( response_time: f64, env: &str )
{
    METRIC_RESPONSE_TIME_COLLECTOR
        .with_label_values( &[env] )
        .observe( response_time );
}

fn track_status_code( status_code: usize, env: &str )
{
    match status_code
    {
        500..=599 => METRIC_RESPONSE_CODE_COLLECTOR
            .with_label_values( &[env, &status_code.to_string(), "500"] )
            .inc(),
        400..=499 => METRIC_RESPONSE_CODE_COLLECTOR
            .with_label_values( &[env, &status_code.to_string(), "400"] )
            .inc(),
        300..=399 => METRIC_RESPONSE_CODE_COLLECTOR
            .with_label_values( &[env, &status_code.to_string(), "300"] )
            .inc(),
        200..=299 => METRIC_RESPONSE_CODE_COLLECTOR
            .with_label_values( &[env, &status_code.to_string(), "200"] )
            .inc(),
        100..=199 => METRIC_RESPONSE_CODE_COLLECTOR
            .with_label_values( &[env, &status_code.to_string(), "100"] )
            .inc(),
        _ => (),
    };
}

async fn metrics_handler() -> Result<impl Reply, Rejection>
{
    let encoder = TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err( e ) = encoder.encode( &REGISTRY.gather(), &mut buffer )
    {
        eprintln!( "could not encode custom metrics: {}", e );
    };
    let mut res = match String::from_utf8( buffer.clone() )
    {
        Ok( v ) => v,
        Err( e ) =>
        {
            eprintln!( "custom metrics could not be from_utf8'd: {}", e );
            String::default()
        }
    };
    buffer.clear();

    let mut buffer = Vec::new();
    if let Err( e ) = encoder.encode( &prometheus::gather(), &mut buffer )
    {
        eprintln!( "could not encode prometheus metrics: {}", e );
    };
    let res_custom = match String::from_utf8( buffer.clone() )
    {
        Ok( v ) => v,
        Err( e ) =>
        {
            eprintln!( "prometheus metrics could not be from_utf8'd: {}", e );
            String::default()
        }
    };
    buffer.clear();

    res.push_str( &res_custom );
    Ok( res )
}

lazy_static! {
    pub static ref REGISTRY: prometheus::Registry = prometheus::Registry::new();
}

#[tokio::main]
async fn main()
{
    add_metrics_to_registry(
        &REGISTRY,
        vec![
            Box::new( METRIC_INCOMING_REQUESTS.clone() ),
            Box::new( METRIC_CONNECTED_CLIENTS.clone() ),
            Box::new( METRIC_RESPONSE_CODE_COLLECTOR.clone() ),
            Box::new( METRIC_RESPONSE_TIME_COLLECTOR.clone() ),
        ],
    );

    let metrics_route = warp::path!( "metrics" ).and_then( metrics_handler );
    let some_route = warp::path!( "some" ).and_then( some_handler );
    let ws_route = warp::path!( "ws" )
        .and( warp::ws() )
        .and( warp::path::param() )
        .and_then( ws_handler );

    tokio::task::spawn( data_collector() );

    println!( "Started on port 9000" );

    // Redirect logs from 'log' crate to 'tracing' crate.
    tracing_log::LogTracer::init().expect( "Failed to initialize monitoring" );

    let ( _maybe_stdio_writer_guard, _maybe_file_writer_guard) = init(
        &Level::Info,
        &[OutputType::Stdout, OutputType::File],
        Some( "./logs" ),
        Some( "monitoring" ),
    );

    let outer_span = info_span!( "outer", level = 0, other_field = tracing::field::Empty );
    let _outer_entered = outer_span.enter();

    let inner_span = info_span!( "inner", level = 1, other_field = tracing::field::Empty );
    let _inner_entered = inner_span.enter();

    outer_span.record( "other_field", &7 );

    let inner_span = debug_span!( "inner", level = 1 );
    let _inner_guard = inner_span.enter();

    info!( message = "Hello, world!", a_bool = true, a_number = 42 );

    warp::serve( metrics_route.or( some_route ).or( ws_route ) )
        .run( ( [0, 0, 0, 0], 9000 ) )
        .await;
}
