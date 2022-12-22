use common::settings::RunEnvironmentType;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

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

    is_file_emitted:         bool,
    is_stdout_emitted:       bool,
    is_wasm_console_emitted: bool,

    log_file_path: Option<&'static str>,
}

impl GeneralConfigs
{
    pub fn new( file_path: &str, env_prefix: &str, profile: &RunEnvironmentType ) -> GeneralConfigs
    {
        Figment::new()
            .merge( Toml::file( file_path ).nested() )
            .select( profile.to_string() )
            .merge( Env::prefixed( env_prefix ) )
            .extract::<GeneralConfigs>()
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
