use lazy_static::lazy_static;
use prometheus::{core::Collector, HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry};

lazy_static! {
    pub static ref METRIC_INCOMING_REQUESTS: IntCounter =
        IntCounter::new( "incoming_requests", "Incoming Requests" ).expect( "metric can't be created" );
    pub static ref METRIC_CONNECTED_CLIENTS: IntGauge =
        IntGauge::new( "connected_clients", "Connected Clients" ).expect( "metric can't be created" );
    pub static ref METRIC_RESPONSE_CODE_COLLECTOR: IntCounterVec = IntCounterVec::new(
        Opts::new( "response_code", "Response Codes" ),
        &["env", "statuscode", "type"]
    )
    .expect( "metric can't be created" );
    pub static ref METRIC_RESPONSE_TIME_COLLECTOR: HistogramVec =
        HistogramVec::new( HistogramOpts::new( "response_time", "Response Times" ), &["env"] )
            .expect( "metric can't be created" );
}

pub fn add_metrics_to_registry( registry: &Registry, metrics: Vec<Box<dyn Collector>> )
{
    for metric in metrics
    {
        registry.register( metric ).expect( "metric can't be registered" );
    }
}
