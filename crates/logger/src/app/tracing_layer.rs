use smartstring::alias::String as SmartString;
use std::collections::BTreeMap;
use std::fmt;
use tracing::field::Field;
use tracing::span::Record;
use tracing::{span::Attributes, Id};
use tracing_subscriber::{layer::Context, Layer};

#[derive(Debug)]
struct TracingFieldStorage( BTreeMap<SmartString, serde_json::Value> );

pub struct TracingLayer;

impl<S> Layer<S> for TracingLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_new_span( &self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S> )
    {
        // Build our JSON object from the field values.
        let mut fields = BTreeMap::new();
        let mut visitor = TracingJsonVisitor( &mut fields );
        attrs.record( &mut visitor );

        let storage = TracingFieldStorage( fields );

        // Get reference to the internal span data.
        let span = ctx.span( id ).expect( "Span not found. This shouldn't happen." );

        // Get the place where tracing stores custom data and store it.
        let mut extensions = span.extensions_mut();
        extensions.insert::<TracingFieldStorage>( storage );
    }

    fn on_record( &self, span: &Id, values: &Record<'_>, ctx: Context<'_, S> )
    {
        // Get span whose data is being recorded.
        let span = ctx.span( span ).expect( "Span not found. This shouldn't happen." );

        // Mutable reference to the span we just found.
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut TracingFieldStorage = extensions_mut.get_mut::<TracingFieldStorage>().unwrap();

        let json_data: &mut BTreeMap<SmartString, serde_json::Value> = &mut custom_field_storage.0;

        let mut visitor = TracingJsonVisitor( json_data );
        values.record( &mut visitor );
    }

    fn on_event( &self, event: &tracing::Event<'_>, ctx: Context<'_, S> )
    {
        // Span context.
        let scope = ctx.event_scope( event ).unwrap();
        let mut spans = vec![];

        for span in scope.from_root()
        {
            let extensions = span.extensions();
            let storage = extensions.get::<TracingFieldStorage>().unwrap();
            let field_data: &BTreeMap<SmartString, serde_json::Value> = &storage.0;

            spans.push( serde_json::json!( {
                "target": span.metadata().target(),
                "name": span.metadata().name(),
                "level": span.metadata().level().to_string(),
                "fields": field_data,
                "time": chrono::Utc::now().to_rfc3339(),
                "callsite": format!( "{:?}", &event.metadata().callsite() ),
            }) );
        }

        // Convert field values into a JSON object.
        let mut fields = BTreeMap::new();
        let mut visitor = TracingJsonVisitor( &mut fields );
        event.record( &mut visitor );

        // Output the event in JSON.
        let output = serde_json::json!( {
            "target": event.metadata().target(),
            "name": event.metadata().name(),
            "level": event.metadata().level().to_string(),
            "fields": fields,
            "spans": spans,
            "time": chrono::Utc::now().to_rfc3339(),
            "callsite": format!( "{:?}", &event.metadata().callsite() ),
        });

        println!( "{}", serde_json::to_string_pretty( &output ).unwrap() );
    }
}

struct TracingJsonVisitor<'a>( &'a mut BTreeMap<SmartString, serde_json::Value> );

impl<'a> tracing::field::Visit for TracingJsonVisitor<'a>
{
    fn record_f64( &mut self, field: &Field, value: f64 )
    {
        self.0
            .insert( SmartString::from( field.name() ), serde_json::json!( value ) );
    }

    fn record_i64( &mut self, field: &Field, value: i64 )
    {
        self.0
            .insert( SmartString::from( field.name() ), serde_json::json!( value ) );
    }

    fn record_u64( &mut self, field: &Field, value: u64 )
    {
        self.0
            .insert( SmartString::from( field.name() ), serde_json::json!( value ) );
    }

    fn record_i128( &mut self, field: &Field, value: i128 )
    {
        self.0
            .insert( SmartString::from( field.name() ), serde_json::json!( value ) );
    }

    fn record_u128( &mut self, field: &Field, value: u128 )
    {
        self.0
            .insert( SmartString::from( field.name() ), serde_json::json!( value ) );
    }

    fn record_bool( &mut self, field: &Field, value: bool )
    {
        self.0
            .insert( SmartString::from( field.name() ), serde_json::json!( value ) );
    }

    fn record_str( &mut self, field: &Field, value: &str )
    {
        self.0
            .insert( SmartString::from( field.name() ), serde_json::json!( value ) );
    }

    fn record_error( &mut self, field: &Field, value: &(dyn std::error::Error + 'static) )
    {
        self.0.insert(
            SmartString::from( field.name() ),
            serde_json::json!( value.to_string() ),
        );
    }

    fn record_debug( &mut self, field: &Field, value: &dyn fmt::Debug )
    {
        self.0.insert(
            SmartString::from( field.name() ),
            serde_json::json!( format!( "{:?}", value ) ),
        );
    }
}
