use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
            Self::Development => write!( f, "development" ),
            Self::Production => write!( f, "production" ),
        }
    }
}

impl From<&str> for RunEnvironmentType
{
    fn from( env: &str ) -> Self
    {
        match env.to_lowercase().as_str()
        {
            "production" => Self::Production,
            _ => Self::Development,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CommonConfigs
{
    frontend_url: &'static str,
    backend_url:  &'static str,
}

impl CommonConfigs
{
    #[must_use]
    pub fn new( file_path: &str, env_prefix: &str, profile: &RunEnvironmentType ) -> Self
    {
        Figment::new()
            .merge( Toml::file( file_path ).nested() )
            .select( profile.to_string() )
            .merge( Env::prefixed( env_prefix ) )
            .extract::<Self>()
            .expect( "Failed to load general configs" )
    }
}
