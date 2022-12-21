use figment::providers::Env;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use smartstring::alias::String;
use std::borrow::Borrow;

#[derive(Debug)]
pub enum RunEnvironmentType
{
    Development,
    Production,
}

impl std::fmt::Display for RunEnvironmentType
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result
    {
        match self
        {
            RunEnvironmentType::Development => write!( f, "development" ),
            RunEnvironmentType::Production => write!( f, "production" ),
        }
    }
}

impl From<&str> for RunEnvironmentType
{
    fn from( env: &str ) -> Self
    {
        match env.to_lowercase().as_str()
        {
            "development" => RunEnvironmentType::Development,
            "production" => RunEnvironmentType::Production,
            _ => RunEnvironmentType::Development,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfigs
{
    app_name: &'static str,
    run_env:  RunEnvironmentType,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfigs
{
    log_level: &'static str,

    is_file_emitted:           bool,
    is_stdout_emitted:         bool,
    is_open_telemetry_emitted: bool,

    log_file_path:           Option<&'static str>,
    open_telemetry_endpoint: Option<&'static str>,
}

impl GeneralConfigs
{
    pub fn new( file_path: &str, env_prefix: &str, profile: &RunEnvironmentType ) -> GeneralConfigs
    {
        Figment::new()
            .merge( Toml::file( file_path ).nested() )
            .select( profile.to_string() )
            .merge( Env::prefixed( env_prefix ) )
            .extrac::<GeneralConfigs>()
            .expect( "Failed to load general configs" )
    }
}

impl LoggingConfigs
{
    pub fn new( file_path: &str, env_prefix: &str, profile: &RunEnvironmentType ) -> LoggingConfigs
    {
        Figment::new()
            .merge( Toml::file( file_path ).nested() )
            .select( profile.to_string() )
            .merge( Env::prefixed( env_prefix ) )
            .extract::<LoggingConfigs>()
            .expect( "Failed to load logging config" )
    }
}
