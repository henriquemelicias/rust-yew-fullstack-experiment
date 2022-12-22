//! Provides metrics collection for prometheus.
//!
//! Provides a set of metrics that can be used to collect data for prometheus, as well as a function
//! to add them to a registry.
//!
//! # Examples
//! ```
//! use lazy_static::lazy_static;
//! use monitoring::prometheus;
//! use monitoring::prometheus::metrics;
//!
//! lazy_static! {
//!    pub static ref REGISTRY: prometheus::Registry = prometheus::Registry::new();//!
//! }
//!
//! prometheus::add_metrics_to_registry(
//!    &REGISTRY,
//!    vec![
//!         Box::new( metrics::INCOMING_REQUESTS.clone() ),
//!         Box::new( metrics::CONNECTED_CLIENTS.clone() ),
//!         Box::new( metrics::RESPONSE_CODE_COLLECTOR.clone() ),
//!         Box::new( metrics::RESPONSE_TIME_COLLECTOR.clone() )
//!    ],
//! );
//!
//! ```
//!

use prometheus::{core::Collector, Registry};

/// Metrics that can be used to collect data for prometheus.
pub mod metrics
{
    use lazy_static::lazy_static;
    use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts};

    lazy_static! {
        pub static ref INCOMING_REQUESTS: IntCounter =
            IntCounter::new( "incoming_requests", "Incoming Requests" ).expect( "metric can't be created" );
        pub static ref CONNECTED_CLIENTS: IntGauge =
            IntGauge::new( "connected_clients", "Connected Clients" ).expect( "metric can't be created" );
        pub static ref RESPONSE_CODE_COLLECTOR: IntCounterVec = IntCounterVec::new(
            Opts::new( "response_code", "Response Codes" ),
            &["env", "statuscode", "type"]
        )
        .expect( "metric can't be created" );
        pub static ref RESPONSE_TIME_COLLECTOR: HistogramVec =
            HistogramVec::new( HistogramOpts::new( "response_time", "Response Times" ), &["env"] )
                .expect( "metric can't be created" );
    }
}

/// Adds a set of metrics to a registry.
///
/// # Arguments
///
/// * `registry` - The registry to add the metrics to.
/// * `metrics` - The metrics to add to the registry.
///
/// # Examples
///
/// see [`crate::prometheus`] for an example.
///
/// # Panics
///
/// If the metrics can't be added to the registry.
///
pub fn add_metrics_to_registry( registry: &Registry, metrics: Vec<Box<dyn Collector>> )
{
    for metric in metrics
    {
        registry.register( metric ).expect( "metric can't be registered" );
    }
}
